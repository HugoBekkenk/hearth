use crate::game::grid_pos::GridPos;
use crate::game::tile_content::TileContent;
use std::collections::HashMap;

pub struct World {
    pub width: i32,
    pub height: i32,
    pub tile_size: i32,
    pub tiles: HashMap<GridPos, TileContent>,
}

impl World {
    pub fn new(width: i32, height: i32, tile_size: i32) -> Self {
        let mut tiles = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                tiles.insert(GridPos { x, y }, TileContent::Empty);
            }
        }
        World {
            width,
            height,
            tile_size,
            tiles,
        }
    }

    pub fn is_walkable(&self, grid_pos: &GridPos) -> bool {
        if let Some(tile) = self.tiles.get(grid_pos)
            && *tile == TileContent::Empty
        {
            return true;
        }
        false
    }

    pub fn find_nearest_walkable(&self, goal: GridPos) -> Option<GridPos> {
        for distance in 1..4i32 {
            for dx in -distance..=distance {
                let dy = distance - dx.abs();
                for candidate in [
                    GridPos {
                        x: goal.x + dx,
                        y: goal.y + dy,
                    },
                    GridPos {
                        x: goal.x + dx,
                        y: goal.y - dy,
                    },
                ] {
                    if self.is_walkable(&candidate) {
                        return Some(candidate);
                    }
                }
            }
        }
        None
    }
}
