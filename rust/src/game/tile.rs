use crate::game::terrain_type::TerrainType;
use crate::game::tile_content::TileContent;

pub struct Tile {
    pub terrain: TerrainType,
    pub content: TileContent,
}

impl Tile {
    pub fn set_content(&mut self, content: TileContent) {
        self.content = content
    }

    pub fn empty(&mut self) {
        self.content = TileContent::Empty
    }

    pub fn is_passable(&self) -> bool {
        self.content == TileContent::Empty
            && self.terrain != TerrainType::Water
            && self.terrain != TerrainType::Rock
    }
}
