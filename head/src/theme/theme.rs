use game::GameTile;
use llr::LLRPixel;
use theme::colours;
use world::tiles::Tile;

/// This holds the colour type.
pub struct Theme {}

impl Theme {
    /// Trivial constructor.
    pub fn new() -> Theme {
        Theme {}
    }

    /// Gets the players draw information.
    pub fn get_player(&self) -> LLRPixel {
        LLRPixel {
            character : '@',
            background : colours::PINK,
            foreground : colours::BLACK,
        }
    }

    /// Gets the pixel information to use when drawing, for the tile inside of
    /// the game tile.
    pub fn get_game_tile(
        &self,
        tile: GameTile,
    ) -> LLRPixel {
        self.get_tile( tile.tile )
    }

    /// Converts the tile into two colours; a back and front colour.
    /// These two colours are returned.
    pub fn get_tile(
        &self,
        tile: Tile,
    ) -> LLRPixel {
        match tile {
            Tile::Empty => {
                LLRPixel {
                    character: '.',
                    background: colours::BLACK,
                    foreground: colours::GREY,
                }
            },
            Tile::Ground => {
                LLRPixel {
                    character: ':',
                    background: colours::BLACK,
                    foreground: colours::BROWN,
                }
            },

            Tile::Grass => {
                LLRPixel {
                    character: ',',
                    background: colours::BLACK,
                    foreground: colours::GREEN,
                }
            },
            Tile::GrassThick => {
                LLRPixel {
                    character: '"',
                    background: colours::BLACK,
                    foreground: colours::GREEN,
                }
            },

            Tile::Rocks => {
                LLRPixel {
                    character: '∩',
                    background: colours::BLACK,
                    foreground: colours::LIGHT_GREY,
                }
            },

            Tile::Hill => {
                LLRPixel {
                    character: '^',
                    background: colours::BLACK,
                    foreground: colours::GREEN,
                }
            },

            Tile::Water => {
                LLRPixel {
                    character: '~',
                    background: colours::BLACK,
                    foreground: colours::BLUE,
                }
            },
            Tile::Wall => {
                LLRPixel {
                    character: '#',
                    background: colours::GREY,
                    foreground: colours::LIGHT_GREY,
                }
            },
            Tile::TreeStump => {
                LLRPixel {
                    character: 'o',
                    background: colours::BLACK,
                    foreground: colours::BROWN,
                }
            },

            Tile::Ice => {
                LLRPixel {
                    character: '∴',
                    background: colours::LIGHT_CYAN,
                    foreground: colours::WHITE,
                }
            },
        }
    }
}
