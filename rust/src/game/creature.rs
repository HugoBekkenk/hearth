use crate::game::vec2::Vec2;
use rand::RngExt;
use std::f64::consts::PI;

pub struct Creature {
    pub id: u32,
    pub position: Vec2,
    pub target: Vec2,
    pub wander_timer: f32,
    pub behavior_state: BehaviorState,
    pub movement_state: MovementState,
    pub config: CreatureConfig,
}

pub struct CreatureConfig {
    pub speed: f32,
    pub acceptance_radius: f32,
    pub min_wander_wait: f32,
    pub max_wander_wait: f32,
    pub min_wander_distance: f64,
    pub max_wander_distance: f64,
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

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Creature {
    pub fn new(id: u32) -> Self {
        Creature {
            id,
            position: Vec2 { x: 0.0, y: 0.0 },
            target: Vec2 { x: 0.0, y: 0.0 },
            wander_timer: 0.0,
            behavior_state: BehaviorState::Wandering,
            movement_state: MovementState::Idle,
            config: CreatureConfig {
                speed: 100.0,
                acceptance_radius: 5.0,
                min_wander_wait: 1.0,
                max_wander_wait: 3.0,
                min_wander_distance: 100.0,
                max_wander_distance: 500.0,
            },
        }
    }

    pub fn choose_wander_target(&mut self) {
        let mut rng = rand::rng();
        let angle = rng.random_range(0.0..(2.0 * PI));
        let distance =
            rng.random_range(self.config.min_wander_distance..self.config.max_wander_distance);
        self.target = self.position.point_in_direction(angle, distance);
    }

    pub fn wander(&mut self, delta: f32) {
        self.wander_timer -= delta;
        if self.wander_timer <= 0.0 {
            self.choose_wander_target();
            let mut rng = rand::rng();
            self.wander_timer =
                rng.random_range(self.config.min_wander_wait..self.config.max_wander_wait);
        } else {
            self.movement_state = MovementState::Idle;
        }
    }

    pub fn is_at_target(&self) -> bool {
        self.target.subtract(&self.position).length() < self.config.acceptance_radius
    }

    pub fn move_towards_target(&mut self, delta: f32) {
        let direction = self.target.subtract(&self.position);
        let normalize_direction = direction.normalize();
        let step = normalize_direction.scale(self.config.speed * delta);

        let dir = if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            if direction.y > 0.0 {
                Direction::Down
            } else {
                Direction::Up
            }
        };

        self.movement_state = MovementState::Moving(dir);
        self.position = self.position.add(&step);
    }
}
