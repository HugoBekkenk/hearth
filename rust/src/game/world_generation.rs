use crate::game::grid_pos::GridPos;
use crate::game::terrain_type::TerrainType;
use crate::game::tile::Tile;
use crate::game::tile_content::TileContent;
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;

pub fn generate(width: i32, height: i32, seed: u32) -> HashMap<GridPos, Tile> {
    let perlin = Perlin::new(seed);
    let mut map: HashMap<GridPos, Tile> = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            map.insert(
                GridPos { x, y },
                Tile {
                    terrain: generate_terrain_type(&perlin, x, y),
                    content: TileContent::Empty,
                },
            );
        }
    }
    map
}

fn generate_terrain_type(perlin: &Perlin, x: i32, y: i32) -> TerrainType {
    let height_value = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
    if height_value <= 0.0 {
        TerrainType::Water
    } else if height_value < 0.1 {
        TerrainType::Sand
    } else if height_value < 0.4 {
        TerrainType::Grass
    } else if height_value < 0.6 {
        TerrainType::Forest
    } else if height_value < 0.8 {
        TerrainType::Rock
    } else {
        TerrainType::Snow
    }
}
