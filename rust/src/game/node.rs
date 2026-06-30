use crate::game::grid_pos::GridPos;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Node {
    pub position: GridPos,
    pub cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
