use crate::game::vec2::Vec2;

pub struct Creature {
    pub position: Vec2,
    pub target: Vec2,
    pub speed: f32,
    pub acceptance_radius: f32,
    pub movement_state: MovementState
}

pub enum MovementState {
    Idle,
    Moving(Direction),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Creature {
    pub fn new() -> Self {
        Creature {
            position: Vec2 { x: 0.0, y: 0.0 },
            target: Vec2 { x: 0.0, y: 0.0 },
            speed: 100.0,
            acceptance_radius: 5.0,
            movement_state: MovementState::Idle
        }
    }

    pub fn is_at_target(&self) -> bool {
        self.target.subtract(&self.position).length() < self.acceptance_radius
    }

    pub fn move_towards_target(&mut self, delta: f32) {
        let direction = self.target.subtract(&self.position);
        let normalize_direction = direction.normalize();
        let step = normalize_direction.scale(self.speed * delta);

        let dir = if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 { Direction::Right } else { Direction::Left }
        } else {
            if direction.y > 0.0 { Direction::Down } else { Direction::Up }
        };

        self.movement_state = MovementState::Moving(dir);
        self.position = self.position.add(&step);
    }
}