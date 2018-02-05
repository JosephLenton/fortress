
use util::colour::RGBA;
use theme::colours;
use theme::TileColour;
use game::GameTile;
use world::tiles::Tile;

/// This holds the colour type.
pub struct Theme {
}

impl Theme {
    pub fn new() -> Theme {
        Theme { }
    }

    /// Converts the tile into two colours; a back and front colour.
    /// These two colours are returned.
    pub fn get_game_tile_colour( &self, tile : GameTile ) -> TileColour {
        self.get_tile_colour( tile.tile )
    }

    pub fn get_tile_colour( &self, tile : Tile ) -> TileColour {
        match tile {
            Tile::Empty => TileColour {
                background: colours::BLACK,
                foreground: colours::GREY,
            },
            Tile::Ground => TileColour {
                background: colours::BLACK,
                foreground: colours::BROWN,
            },

            Tile::Grass => TileColour {
                background: colours::BLACK,
                foreground: colours::GREEN,
            },
            Tile::GrassThick => TileColour {
                background: colours::BLACK,
                foreground: colours::GREEN,
            },

            Tile::Rocks => TileColour {
                background: colours::BLACK,
                foreground: colours::LIGHT_GREY,
            },

            Tile::Hill => TileColour {
                background: colours::BLACK,
                foreground: colours::GREEN,
            },

            Tile::Water => TileColour {
                background: colours::BLACK,
                foreground: colours::BLUE,
            },
            Tile::Wall => TileColour {
                background: colours::GREY,
                foreground: colours::LIGHT_GREY,
            },
            Tile::TreeStump => TileColour {
                background: colours::BLACK,
                foreground: colours::BROWN,
            },

            Tile::Ice => TileColour {
                background: colours::LIGHT_CYAN,
                foreground: colours::WHITE,
            },
        }
    }

    pub fn get_player_colour( &self ) -> RGBA {
        colours::PINK
    }
}


