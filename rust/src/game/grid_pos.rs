use crate::game::direction::Direction;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn step(&self, direction: &Direction) -> GridPos {
        let mut new_pos = GridPos {
            x: self.x,
            y: self.y,
        };
        match direction {
            Direction::Up => new_pos.y -= 1,
            Direction::Down => new_pos.y += 1,
            Direction::Left => new_pos.x -= 1,
            Direction::Right => new_pos.x += 1,
        }
        new_pos
    }

    pub fn offset(&self, x_offset: i32, y_offset: i32) -> GridPos {
        GridPos {
            x: self.x + x_offset,
            y: self.y + y_offset,
        }
    }
}
