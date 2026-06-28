use crate::game::direction::Direction;

#[derive(PartialEq, Eq, Hash)]
pub struct GridPos {
    pub(crate) x: i32,
    pub(crate) y: i32,
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
}
