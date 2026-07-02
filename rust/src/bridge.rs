use crate::game::creature::{BehaviorState, Creature, MovementState};
use crate::game::direction::Direction;
use crate::game::grid_pos::GridPos;
use crate::game::pathfinding::find_path;
use crate::game::tile_content::TileContent;
use crate::game::world::World;
use godot::prelude::*;
use rand::RngExt;
use crate::game::terrain_type::TerrainType;

#[derive(GodotClass)]
#[class(base=Node)]
struct HearthBridge {
    base: Base<Node>,
    pub world: World,
    pub tile_size: i32,
    pub creatures: Vec<Creature>,
    pub selected_creatures: Vec<u32>,
    pub next_id: u32,
}

// Godot functions
#[godot_api]
impl INode for HearthBridge {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            world: World::new(50, 30),
            tile_size: 32,
            creatures: vec![],
            selected_creatures: vec![],
            next_id: 0,
        }
    }

    fn ready(&mut self) {
        godot_print!("Hearth extension is alive!");
    }

    fn process(&mut self, delta: f64) {
        for creature in self.creatures.iter_mut() {
            if !creature.path.is_empty() {
                creature.move_towards_target(delta as f32, &mut self.world)
            } else if creature.behavior_state == BehaviorState::Wandering {
                creature.wander(delta as f32, &self.world)
            } else {
                Self::try_return_to_wander(creature, &self.selected_creatures);
            }
        }
    }
}

// Public functions
#[godot_api]
impl HearthBridge {
    #[func]
    pub fn spawn_creature(&mut self) -> i64 {
        let spawn_position = self.generate_spawn_position();
        let current_id = self.next_id;
        if self.world.is_walkable(&spawn_position) {
            let new_creature = Creature::new(
                self.next_id,
                spawn_position,
                Self::generate_creature_speed(),
            );
            self.creatures.push(new_creature);
            self.world
                .try_occupy_tile(&spawn_position, TileContent::Creature(current_id));
            self.next_id += 1;
            current_id as i64
        } else {
            godot_warn!("Spawn position is already taken");
            -1
        }
    }

    #[func]
    pub fn get_creature_position(&mut self, id: u32) -> Vector2 {
        let tile_size = self.tile_size;
        if let Some(creature) = self.find_creature(id) {
            Self::grid_to_world(&creature.position, tile_size)
        } else {
            godot_error!("Creature with id {} not found", id);
            Vector2::ZERO
        }
    }

    #[func]
    pub fn select_creature(&mut self, id: u32) {
        if self.selected_creatures.contains(&id) {
            self.deselect_creature(id);
        } else {
            self.selected_creatures.push(id);
        }
    }

    #[func]
    pub fn select_all_creature(&mut self) {
        for creature in self.creatures.iter_mut() {
            if !self.selected_creatures.contains(&creature.id) {
                self.selected_creatures.push(creature.id);
            }
        }
    }

    #[func]
    pub fn deselect_all_creature(&mut self) {
        self.selected_creatures.clear()
    }

    #[func]
    pub fn set_creature_target(&mut self, target: Vector2) {
        let grid_target = Self::world_to_grid(target, self.tile_size);
        if self.world.is_walkable(&grid_target) {
            let ids: Vec<u32> = self.selected_creatures.clone();
            let world = &self.world;
            let creatures = &mut self.creatures;
            for creature_id in ids {
                if let Some(creature) = creatures.iter_mut().find(|c| c.id == creature_id) {
                    creature.path =
                        find_path(creature.position, grid_target, world).unwrap_or_default();
                    creature.behavior_state = BehaviorState::BeingOrdered;
                }
            }
        }
    }

    #[func]
    pub fn get_animation_name(&mut self, id: u32) -> GString {
        if let Some(creature) = self.find_creature(id) {
            match &creature.movement_state {
                MovementState::Idle => "idle".into(),
                MovementState::Moving(dir) => match dir {
                    Direction::Up => "moving_up".into(),
                    Direction::Down => "moving_down".into(),
                    Direction::Left => "moving_left".into(),
                    Direction::Right => "moving_right".into(),
                },
            }
        } else {
            godot_error!("Creature with id {} not found", id);
            "idle".into()
        }
    }

    #[func]
    pub fn get_world_width(&self) -> i32 {
        self.world.width
    }

    #[func]
    pub fn get_world_height(&self) -> i32 {
        self.world.height
    }

    #[func]
    pub fn get_tile_size(&self) -> i32 {
        self.tile_size
    }

    #[func]
    pub fn get_terrain_type(&self, x: i32, y: i32) -> i32 {
        let pos = GridPos { x, y };
        if let Some(tile) = self.world.tiles.get(&pos) {
            match tile.terrain {
                TerrainType::Water => 0,
                TerrainType::Sand => 1,
                TerrainType::Grass => 2,
                TerrainType::Forest => 3,
                TerrainType::Rock => 4,
                TerrainType::Snow => 5,
            }
        } else {
            -1
        }
    }
}

// private functions
impl HearthBridge {
    fn find_creature(&mut self, id: u32) -> Option<&mut Creature> {
        self.creatures.iter_mut().find(|c| c.id == id)
    }

    fn world_to_grid(pos: Vector2, tile_size: i32) -> GridPos {
        GridPos {
            x: (pos.x / tile_size as f32) as i32,
            y: (pos.y / tile_size as f32) as i32,
        }
    }

    fn grid_to_world(pos: &GridPos, tile_size: i32) -> Vector2 {
        let half = tile_size / 2;
        Vector2 {
            x: ((pos.x * tile_size) + half) as f32,
            y: ((pos.y * tile_size) + half) as f32,
        }
    }

    fn generate_spawn_position(&self) -> GridPos {
        let mut rng = rand::rng();
        GridPos {
            x: rng.random_range(0..self.world.width),
            y: rng.random_range(0..self.world.height),
        }
    }

    fn try_return_to_wander(creature: &mut Creature, selected_creatures: &[u32]) {
        creature.movement_state = MovementState::Idle;
        if !selected_creatures.contains(&creature.id) {
            creature.behavior_state = BehaviorState::Wandering;
        }
    }

    fn deselect_creature(&mut self, id: u32) {
        self.selected_creatures.retain(|&c| c != id)
    }

    fn generate_creature_speed() -> f32 {
        rand::rng().random_range(3.0..5.0)
    }
}
