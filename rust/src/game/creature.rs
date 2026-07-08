use crate::game::direction::Direction;
use crate::game::grid_pos::GridPos;
use crate::game::pathfinding::find_path;
use crate::game::tile_content::TileContent;
use crate::game::timer::Timer;
use crate::game::world::World;
use rand::RngExt;
use rand::prelude::ThreadRng;

pub struct Creature {
    pub id: u32,
    pub position: GridPos,
    pub target: Option<GridPos>,
    pub path: Vec<GridPos>,
    pub movement_timer: Timer,
    pub wander_timer: Timer,
    pub repath_timer: Timer,
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
            target: None,
            path: vec![],
            movement_timer: Timer::new(0.0),
            wander_timer: Timer::new(0.0),
            repath_timer: Timer::new(0.0),
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

    pub fn wander(&mut self, delta: f32, world: &World) {
        self.wander_timer.tick_down(delta);
        if self.wander_timer.is_complete() {
            self.choose_wander_target(world);
            self.wander_timer.reset(self.calculate_wander_delay());
        } else {
            self.movement_state = MovementState::Idle;
        }
    }

    pub fn move_towards_target(&mut self, delta: f32, world: &mut World) {
        self.movement_timer.tick_down(delta);
        self.repath_timer.tick_down(delta);
        if self.movement_timer.is_complete()
            && let Some(&next_tile) = self.path.first()
        {
            if !world.is_walkable(&next_tile) {
                if self.repath_timer.is_complete() {
                    self.repath_around_obstacle(world);
                    self.repath_timer.reset(1.0);
                }
            } else {
                self.step_to(world, next_tile);
            }
            self.movement_timer.reset(self.calculate_movement_delay());
        }
    }
}

// Private helpers
impl Creature {
    fn choose_wander_target(&mut self, world: &World) {
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
        self.update_path(world, target_pos);
    }

    fn random_wander_distance(&self, rng: &mut ThreadRng) -> i32 {
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

    fn direction_to(&self, next_tile: GridPos) -> Direction {
        let x_bias = next_tile.x - self.position.x;
        let y_bias = next_tile.y - self.position.y;

        match (x_bias, y_bias) {
            (x, _) if x > 0 => Direction::Right,
            (x, _) if x < 0 => Direction::Left,
            (_, y) if y > 0 => Direction::Down,
            _ => Direction::Up,
        }
    }

    fn step_to(&mut self, world: &mut World, next_tile_pos: GridPos) {
        let direction = self.direction_to(next_tile_pos);
        world.try_vacant_tile(&self.position);
        world.try_occupy_tile(&next_tile_pos, TileContent::Creature(self.id));
        self.position = next_tile_pos;
        self.movement_state = MovementState::Moving(direction);
        self.path.remove(0);
    }

    fn repath_around_obstacle(&mut self, world: &World) {
        if let Some(goal) = self.path.last().copied()
            && let Some(available_goal) = world.find_nearest_walkable(goal)
        {
            self.target = Some(available_goal);
        }
    }

    fn update_path(&mut self, world: &World, goal: GridPos) {
        self.path = find_path(self.position, goal, world).unwrap_or_default();
    }

    fn calculate_wander_delay(&self) -> f32 {
        rand::rng().random_range(self.config.min_wander_wait..self.config.max_wander_wait)
    }

    fn calculate_movement_delay(&self) -> f32 {
        1.0 / self.config.speed
    }
}
