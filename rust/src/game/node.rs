use crate::game::grid_pos::GridPos;

// A tile candidate in the A* priority queue.
// cost = estimated total steps to reach the goal through this tile (g + manhattan_distance).
// Ord is implemented in reverse so the BinaryHeap (normally max-first) pops the lowest cost first.
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Node {
    pub position: GridPos,
    pub cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // reversed: lower cost = higher priority
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
