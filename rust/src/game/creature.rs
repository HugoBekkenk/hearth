use rand::prelude::ThreadRng;
use crate::game::direction::Direction;
use crate::game::grid_pos::GridPos;
use crate::game::pathfinding::find_path;
use crate::game::tile_content::TileContent;
use crate::game::world::World;
use rand::RngExt;

pub struct Creature {
    pub id: u32,
    pub position: GridPos,
    pub path: Vec<GridPos>,
    pub movement_timer: f32,
    pub wander_timer: f32,
    pub behavior_state: BehaviorState,
    pub movement_state: MovementState,
    pub config: CreatureConfig,
}

pub struct CreatureConfig {
    pub speed: f32,
    pub min_wander_wait: f32,
    pub max_wander_wait: f32,
    pub min_wander_distance: i32,
    pub max_wander_distance: i32,
}

#[derive(PartialEq)]
pub enum BehaviorState {
    Wandering,
    BeingOrdered,
}

pub enum MovementState {
    Idle,
    Moving(Direction),
}

// public functions
impl Creature {
    pub fn new(id: u32, position: GridPos, speed: f32) -> Self {
        Creature {
            id,
            position,
            path: vec![],
            movement_timer: 0.0,
            wander_timer: 0.0,
            behavior_state: BehaviorState::Wandering,
            movement_state: MovementState::Idle,
            config: CreatureConfig {
                speed,
                min_wander_wait: 1.0,
                max_wander_wait: 3.0,
                min_wander_distance: 1,
                max_wander_distance: 5,
            },
        }
    }

    pub fn choose_wander_target(&mut self, world: &World) {
        let mut rng = rand::rng();
        let wander_distance = self.random_wander_distance(&mut rng);
        let direction = Self::random_direction(&mut rng);

        let mut target_pos = self.position;
        for _ in 0..wander_distance {
            let next_pos = target_pos.step(&direction);
            if world.is_in_bound(&next_pos) {
                target_pos = next_pos;
            }
        }
        self.find_path(&world, target_pos);
    }

    pub fn wander(&mut self, delta: f32, world: &World) {
        self.tick_down_wander_timer(delta);
        if self.wander_timer_is_complete() {
            self.choose_wander_target(world);
            self.reset_wander_timer(&mut rand::rng());
        } else {
            self.movement_state = MovementState::Idle;
        }
    }

    pub fn move_towards_target(&mut self, delta: f32, world: &mut World) {
        self.tick_down_movement_timer(delta);
        if self.movement_timer_is_complete()
            && let Some(&next_tile) = self.path.first()
        {
            if !world.is_walkable(&next_tile) {
                self.repath_around_obstacle(world);
            } else {
                self.step_to(world, next_tile);
            }
            self.reset_movement_timer();
        }
    }
}

// Private helpers
impl Creature {
    fn random_wander_distance(&mut self, rng: &mut ThreadRng) -> i32 {
        rng.random_range(self.config.min_wander_distance..self.config.max_wander_distance)
    }

    fn random_direction(rng: &mut ThreadRng) -> Direction {
        match rng.random_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }

    fn vacant_tile(&mut self, world: &mut World) {
        world.tiles.insert(self.position, TileContent::Empty);
    }

    fn occupy_tile(&mut self, world: &mut World, next_tile: GridPos) {
        self.position = next_tile;
        world
            .tiles
            .insert(next_tile, TileContent::Creature(self.id));
    }

    fn reset_movement_timer(&mut self) {
        self.movement_timer = 1.0 / self.config.speed;
    }

    fn direction_to(&self, next_tile: GridPos) -> Direction {
        let x_bias = next_tile.x - self.position.x;
        let y_bias = next_tile.y - self.position.y;
        let direction: Direction;
        if x_bias != 0 {
            if x_bias > 0 {
                direction = Direction::Right
            } else {
                direction = Direction::Left
            }
        } else {
            if y_bias > 0 {
                direction = Direction::Down
            } else {
                direction = Direction::Up
            }
        }
        direction
    }

    fn step_to(&mut self, world: &mut World, next_tile: GridPos) {
        let direction = self.direction_to(next_tile);
        self.vacant_tile(world);
        self.occupy_tile(world, next_tile);
        self.movement_state = MovementState::Moving(direction);
        self.path.remove(0);
    }

    fn repath_around_obstacle(&mut self, world: &mut World) {
        if let Some(available_goal) =
            world.find_nearest_walkable(*self.path.last().unwrap())
        {
            self.find_path(world, available_goal);
        }
    }

    fn reset_wander_timer(&mut self, rng: &mut ThreadRng) {
        self.wander_timer =
            rng.random_range(self.config.min_wander_wait..self.config.max_wander_wait);
    }

    fn wander_timer_is_complete(&self) -> bool {
        self.wander_timer <= 0.0
    }

    fn tick_down_wander_timer(&mut self, delta: f32) {
        self.wander_timer -= delta;
    }

    fn find_path(&mut self, world: &World, goal: GridPos) {
        self.path = find_path(self.position, goal, world).unwrap_or_default();
    }

    fn movement_timer_is_complete(&self) -> bool {
        self.movement_timer <= 0.0
    }

    fn tick_down_movement_timer(&mut self, delta: f32) {
        self.movement_timer -= delta;
    }
}
