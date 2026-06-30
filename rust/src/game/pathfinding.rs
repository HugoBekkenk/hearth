use crate::game::direction::Direction;
use crate::game::grid_pos::GridPos;
use crate::game::node::Node;
use crate::game::world::World;
use std::collections::{BinaryHeap, HashMap};

// A* pathfinding: finds the shortest walkable path from start to goal.
// Returns None if no path exists (goal is blocked or unreachable).
pub fn find_path(start: GridPos, goal: GridPos, world: &World) -> Option<Vec<GridPos>> {
    let mut priority_que = BinaryHeap::new();
    // came_from[tile] = the tile we stepped from to reach it — used to reconstruct the path at the end
    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
    // path_cost[tile] = cheapest known number of steps to reach that tile from start
    let mut path_cost: HashMap<GridPos, i32> = HashMap::new();

    path_cost.insert(start, 0);
    priority_que.push(Node {
        position: start,
        cost: 0,
    });

    while let Some(current) = priority_que.pop() {
        if current.position == goal {
            // Found the goal — walk came_from backwards from goal to start to reconstruct the path
            let mut route: Vec<GridPos> = vec![];
            let mut latest_position = current.position;
            while latest_position != start {
                route.push(latest_position);
                latest_position = came_from[&latest_position];
            }
            route.reverse(); // came_from gives path backwards, so flip it
            return Some(route);
        }

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
                // Only explore this neighbor if we haven't reached it yet, or found a cheaper route
                if !path_cost.contains_key(&step) || tentative_cost < path_cost[&step] {
                    path_cost.insert(step, tentative_cost);
                    came_from.insert(step, current.position);
                    // Priority = steps taken so far + estimated steps remaining (heuristic)
                    let estimated_total_cost = tentative_cost + heuristic(step, goal);
                    priority_que.push(Node {
                        position: step,
                        cost: estimated_total_cost,
                    });
                }
            }
        }
    }

    None // Queue emptied without reaching goal — no path exists
}

// Manhattan distance: minimum steps between two tiles ignoring obstacles
fn heuristic(a: GridPos, b: GridPos) -> i32 {
    let delta_x = b.x - a.x;
    let delta_y = b.y - a.y;
    delta_x.abs() + delta_y.abs()
}
