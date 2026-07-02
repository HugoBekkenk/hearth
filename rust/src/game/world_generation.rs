use crate::game::grid_pos::GridPos;
use crate::game::terrain_type::TerrainType;
use crate::game::tile::Tile;
use crate::game::tile_content::TileContent;
use noise::{NoiseFn, OpenSimplex};
use std::collections::HashMap;

pub fn generate(width: i32, height: i32, seed: u32) -> HashMap<GridPos, Tile> {
    let simplex = OpenSimplex::new(seed);
    let mut map: HashMap<GridPos, Tile> = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            map.insert(
                GridPos { x, y },
                Tile {
                    terrain: generate_terrain_type(&simplex, x, y, width, height),
                    content: TileContent::Empty,
                },
            );
        }
    }
    map
}

fn generate_terrain_type(
    simplex: &OpenSimplex,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> TerrainType {
    let x_offset = (x - width / 2) as f64;
    let y_offset = (y - height / 2) as f64;
    let distance = (x_offset.powi(2) + y_offset.powi(2)).sqrt();
    let max_distance = ((width as f64 / 2.0).powi(2) + (height as f64 / 2.0).powi(2)).sqrt();
    let normalized_distance = distance / max_distance;
    let mut height_value = simplex.get([x as f64 * 0.05, y as f64 * 0.05]);
    height_value -= normalized_distance * 0.8;
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
