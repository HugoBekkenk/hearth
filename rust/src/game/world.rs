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
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                tiles.insert(GridPos { x, y }, TileContent::Empty);
            }
        }
        World {
            width,
            height,
            tile_size: 32,
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
}
