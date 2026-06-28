use crate::game::creature::{BehaviorState, Creature, MovementState};
use crate::game::direction::Direction;
use crate::game::world::World;
use crate::game::grid_pos::GridPos;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct HearthBridge {
    base: Base<Node>,
    pub world: World,
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
            world: World::new(50, 50),
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
            if !creature.is_at_target() {
                creature.move_towards_target(delta as f32)
            } else if creature.behavior_state == BehaviorState::Wandering {
                creature.wander(delta as f32)
            } else {
                creature.movement_state = MovementState::Idle;
            }
        }
    }
}

// Public functions
#[godot_api]
impl HearthBridge {
    #[func]
    pub fn spawn_creature(&mut self) -> u32 {
        let current_id = self.next_id;
        let new_creature = Creature::new(self.next_id);
        self.creatures.push(new_creature);
        self.next_id += 1;
        current_id
    }

    #[func]
    pub fn get_creature_position(&mut self, id: u32) -> Vector2 {
        let tile_size = self.world.tile_size;
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
            self.selected_creatures.retain(|&c| c != id);
        } else {
            self.selected_creatures.push(id);
        }
    }

    #[func]
    pub fn set_creature_target(&mut self, target: Vector2) {
        let ids: Vec<u32> = self.selected_creatures.clone();
        let tile_size = self.world.tile_size;
        for creature_id in ids {
            if let Some(creature) = self.find_creature(creature_id) {
                creature.target = Self::world_to_grid(target, tile_size);
                creature.behavior_state = BehaviorState::BeingOrdered;
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
}

// private functions
impl HearthBridge {
    pub(crate) fn find_creature(&mut self, id: u32) -> Option<&mut Creature> {
        self.creatures.iter_mut().find(|c| c.id == id)
    }

    fn world_to_grid(pos: Vector2, tile_size: i32) -> GridPos {
        GridPos {
            x: (pos.x / tile_size as f32) as i32,
            y: (pos.y / tile_size as f32) as i32,
        }
    }

    fn grid_to_world(pos: &GridPos, tile_size: i32) -> Vector2 {
        Vector2 {
            x: (pos.x * tile_size) as f32,
            y: (pos.y * tile_size) as f32,
        }
    }
}
