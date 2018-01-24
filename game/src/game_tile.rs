use world::tiles::Tile;

/// A tile within the game. This wraps the worlds tiles in order to add more
/// functionality.
#[derive(Copy, Clone)]
pub struct GameTile {
    /// The land for this tile.
    pub tile: Tile,
}

impl GameTile {
    /// Trivial constructor.
    pub fn new(tile: Tile) -> GameTile {
        GameTile { tile: tile }
    }
}
