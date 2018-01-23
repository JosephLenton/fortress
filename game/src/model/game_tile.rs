use world::tile::Tile;

#[derive(Copy, Clone)]
pub struct GameTile {
    pub tile: Tile,
}

impl GameTile {
    pub fn new(tile: Tile) -> GameTile {
        GameTile { tile: tile }
    }
}
