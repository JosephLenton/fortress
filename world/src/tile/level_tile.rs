
use tile::Tile;

pub enum MoveType {

    /// 
    /// For empty spaces, like in the ground holes.
    ///
    /// You do not want to move here. But if you did, you'd fall.
    /// 
    Empty,

    /// 
    /// This means you can walk on it.
    ///
    /// This may include unusual things, like chairs.
    /// You can walk over a chair.
    /// 
    Ground,

    /// 
    /// It is water-like.
    /// Larva counts as water-like because it moves.
    ///
    Water,

    /// 
    /// Things you cannot move through.
    /// Like walls, and tree stumps.
    ///
    Impassable,

}

pub struct LevelTile {
    tile      : Tile,

    move_type : MoveType,
}

impl LevelTile {
    pub fn new( tile : Tile ) -> LevelTile {
        return match tile {
            Empty => LevelTile {
                tile        : tile,
                move_type   : MoveType::Empty,
            },

            Ground => LevelTile {
                tile        : tile,
                move_type   : MoveType::Ground,
            },

            Grass => LevelTile {
                tile        : tile,
                move_type   : MoveType::Ground,
            },
            GrassThick => LevelTile {
                tile        : tile,
                move_type   : MoveType::Ground,
            },

            Hill => LevelTile {
                tile        : tile,
                move_type   : MoveType::Ground,
            },

            Rocks => LevelTile {
                tile        : tile,
                move_type   : MoveType::Impassable,
            },

            Water => LevelTile {
                tile        : tile,
                move_type   : MoveType::Water,
            },

            Wall => LevelTile {
                tile        : tile,
                move_type   : MoveType::Impassable,
            },
            TreeStump => LevelTile {
                tile        : tile,
                move_type   : MoveType::Impassable,
            },

            Ice => LevelTile {
                tile        : tile,
                move_type   : MoveType::Ground,
            },
        }
    }
}

