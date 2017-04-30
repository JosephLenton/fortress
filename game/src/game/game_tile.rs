
extern crate tiles;

use tiles::tile::tile::Tile;

#[derive(Copy, Clone)]
pub struct GameTile {
    pub tile : Tile,
}

impl GameTile {
    pub fn new( tile : Tile ) -> GameTile {
        return GameTile {
            tile : tile,
        }
    }
}
