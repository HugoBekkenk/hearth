use godot::prelude::*;
use crate::game::creature::{Creature, Direction, MovementState};
use crate::game::vec2::Vec2;

#[derive(GodotClass)]
#[class(base=Node)]
struct HearthBridge {
    base: Base<Node>,
    creature: Creature
}

#[godot_api]
impl INode for HearthBridge {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            creature: Creature::new()
        }
    }

    fn ready(&mut self) {
        godot_print!("Hearth extension is alive!");
    }

    fn process(&mut self, delta: f64) {
        if !self.creature.is_at_target() {
            self.creature.move_towards_target(delta as f32)
        } else {
            self.creature.movement_state = MovementState::Idle;
        }
    }
}

#[godot_api]
impl  HearthBridge {
    #[func]
    pub fn get_creature_position(&mut self) -> Vector2 {
        Vector2 {x: self.creature.position.x, y: self.creature.position.y}
    }

    #[func]
    pub fn set_creature_target(&mut self, target: Vector2) {
        self.creature.target = Vec2 { x: target.x, y: target.y };
    }

    #[func]
    pub fn get_animation_name(&self) -> GString {
        match &self.creature.movement_state {
            MovementState::Idle => "idle".into(),
            MovementState::Moving(dir) => match dir {
                Direction::Up => "moving_up".into(),
                Direction::Down => "moving_down".into(),
                Direction::Left => "moving_left".into(),
                Direction::Right => "moving_right".into(),
            }
        }
    }
}