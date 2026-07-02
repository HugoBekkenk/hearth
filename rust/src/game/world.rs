use crate::game::grid_pos::GridPos;
use crate::game::terrain_type::TerrainType;
use crate::game::tile::Tile;
use crate::game::tile_content::TileContent;
use std::collections::HashMap;

const MAX_WALKABLE_SEARCH_RADIUS: i32 = 5;

pub struct World {
    pub width: i32,
    pub height: i32,
    pub tiles: HashMap<GridPos, Tile>,
}

// public functions
impl World {
    pub fn new(width: i32, height: i32) -> Self {
        World {
            width,
            height,
            tiles: Self::create_initial_tiles(width, height),
        }
    }

    pub fn is_walkable(&self, grid_pos: &GridPos) -> bool {
        if let Some(tile) = self.tiles.get(grid_pos)
            && tile.is_passable()
        {
            return true;
        }
        false
    }

    pub fn is_in_bound(&self, grid_pos: &GridPos) -> bool {
        grid_pos.x >= 0 && grid_pos.x < self.width && grid_pos.y >= 0 && grid_pos.y < self.height
    }

    pub fn find_nearest_walkable(&self, goal: GridPos) -> Option<GridPos> {
        for distance in 1..MAX_WALKABLE_SEARCH_RADIUS {
            for x_offset in -distance..=distance {
                let y_offset = distance - x_offset.abs();
                for candidate in [
                    goal.offset(x_offset, y_offset),
                    goal.offset(x_offset, -y_offset),
                ] {
                    if self.is_walkable(&candidate) {
                        return Some(candidate);
                    }
                }
            }
        }
        None
    }

    pub fn try_vacant_tile(&mut self, position: &GridPos) {
        if let Some(tile) = self.tiles.get_mut(position) {
            tile.empty();
        }
    }

    pub fn try_occupy_tile(&mut self, position: &GridPos, content: TileContent) {
        if let Some(next_tile) = self.tiles.get_mut(position) {
            next_tile.set_content(content);
        }
    }
}

// Private helpers
impl World {
    fn create_initial_tiles(width: i32, height: i32) -> HashMap<GridPos, Tile> {
        let mut tiles = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                tiles.insert(
                    GridPos { x, y },
                    Tile {
                        terrain: TerrainType::Grass,
                        content: TileContent::Empty,
                    },
                );
            }
        }
        tiles
    }
}
