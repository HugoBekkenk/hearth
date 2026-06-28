use crate::game::direction::Direction;
use crate::game::grid_pos::GridPos;
use rand::RngExt;

pub struct Creature {
    pub id: u32,
    pub position: GridPos,
    pub target: GridPos,
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

impl Creature {
    pub fn new(id: u32) -> Self {
        Creature {
            id,
            position: GridPos { x: 0, y: 0 },
            target: GridPos { x: 0, y: 0 },
            movement_timer: 0.0,
            wander_timer: 0.0,
            behavior_state: BehaviorState::Wandering,
            movement_state: MovementState::Idle,
            config: CreatureConfig {
                speed: 3.0,
                min_wander_wait: 1.0,
                max_wander_wait: 3.0,
                min_wander_distance: 1,
                max_wander_distance: 5,
            },
        }
    }

    pub fn choose_wander_target(&mut self) {
        let mut rng = rand::rng();
        let wander_amount =
            rng.random_range(self.config.min_wander_distance..self.config.max_wander_distance);
        let direction = match rng.random_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        };

        let mut target_pos = GridPos {
            x: self.position.x,
            y: self.position.y,
        };
        for _ in 0..wander_amount {
            target_pos = target_pos.step(&direction);
        }
        self.target = target_pos;
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
        self.position == self.target
    }

    pub fn move_towards_target(&mut self, delta: f32) {
        self.movement_timer -= delta;
        if self.movement_timer <= 0.0 {
            let x_bias = self.target.x - self.position.x;
            let y_bias = self.target.y - self.position.y;
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
            self.position = self.position.step(&direction);
            self.movement_state = MovementState::Moving(direction);
            self.movement_timer = 1.0 / self.config.speed;
        }
    }
}
