use crate::game::creature::{BehaviorState, Creature, Direction, MovementState};
use crate::game::vec2::Vec2;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct HearthBridge {
    base: Base<Node>,
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
        if let Some(creature) = self.find_creature(id) {
            Vector2 {
                x: creature.position.x,
                y: creature.position.y,
            }
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
        for creature_id in ids {
            if let Some(creature) = self.find_creature(creature_id) {
                creature.target = Vec2 {
                    x: target.x,
                    y: target.y,
                };
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
    fn find_creature(&mut self, id: u32) -> Option<&mut Creature> {
        self.creatures.iter_mut().find(|c| c.id == id)
    }
}
