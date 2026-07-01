use crate::game::direction::Direction;
use crate::game::grid_pos::GridPos;
use crate::game::node::Node;
use crate::game::world::World;
use std::collections::{BinaryHeap, HashMap};

// A* pathfinding: finds the shortest walkable path from start to goal.
// Returns None if no path exists (goal is blocked or unreachable).
pub fn find_path(start: GridPos, goal: GridPos, world: &World) -> Option<Vec<GridPos>> {
    let mut priority_que = BinaryHeap::new();
    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
    let mut path_cost: HashMap<GridPos, i32> = HashMap::new();

    path_cost.insert(start, 0);
    priority_que.push(Node {
        position: start,
        cost: 0,
    });

    while let Some(current) = priority_que.pop() {
        if current.position == goal {
            return Some(reconstruct_path(start, &came_from, goal));
        }

        explore_neighbors(
            goal,
            world,
            &mut priority_que,
            &mut came_from,
            &mut path_cost,
            &current,
        );
    }

    None
}

fn explore_neighbors(
    goal: GridPos,
    world: &World,
    priority_que: &mut BinaryHeap<Node>,
    came_from: &mut HashMap<GridPos, GridPos>,
    path_cost: &mut HashMap<GridPos, i32>,
    current: &Node,
) {
    let directions = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];
    for direction in directions {
        let step = current.position.step(&direction);
        if world.is_walkable(&step) {
            let tentative_cost = path_cost[&current.position] + 1;
            if is_better_path(path_cost, &step, tentative_cost) {
                queue_neighbor(
                    goal,
                    priority_que,
                    came_from,
                    path_cost,
                    current,
                    step,
                    tentative_cost,
                );
            }
        }
    }
}

fn queue_neighbor(
    goal: GridPos,
    priority_que: &mut BinaryHeap<Node>,
    came_from: &mut HashMap<GridPos, GridPos>,
    path_cost: &mut HashMap<GridPos, i32>,
    current: &Node,
    step: GridPos,
    tentative_cost: i32,
) {
    path_cost.insert(step, tentative_cost);
    came_from.insert(step, current.position);

    let estimated_total_cost = tentative_cost + manhattan_distance(step, goal);
    priority_que.push(Node {
        position: step,
        cost: estimated_total_cost,
    });
}

fn is_better_path(path_cost: &HashMap<GridPos, i32>, step: &GridPos, tentative_cost: i32) -> bool {
    !path_cost.contains_key(&step) || tentative_cost < path_cost[&step]
}

fn reconstruct_path(
    start: GridPos,
    came_from: &HashMap<GridPos, GridPos>,
    goal: GridPos,
) -> Vec<GridPos> {
    let mut route: Vec<GridPos> = vec![];
    let mut current = goal;
    while current != start {
        route.push(current);
        current = came_from[&current];
    }
    route.reverse();
    route
}

// minimum steps between two tiles ignoring obstacles
fn manhattan_distance(a: GridPos, b: GridPos) -> i32 {
    let delta_x = b.x - a.x;
    let delta_y = b.y - a.y;
    delta_x.abs() + delta_y.abs()
}
