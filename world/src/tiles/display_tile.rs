use std::fmt;

use tiles::Tile;
use tiles::tile_colour::tile_to_colour;
use tiles::tile_colour::TileColour;

#[derive(Copy, Clone)]
pub struct DisplayTile {
    pub tile: Tile,
    pub display: char,
    pub colour: TileColour,
}

impl DisplayTile {
    pub fn new(tile: Tile) -> DisplayTile {
        DisplayTile {
            tile: tile,
            display: tile_to_char(tile),
            colour: tile_to_colour(tile),
        }
    }

    pub fn to_char(self) -> char {
        self.display
    }

    pub fn to_string(&self) -> String {
        self.to_char().to_string()
    }
}

impl fmt::Display for DisplayTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

pub fn tile_to_char(tile: Tile) -> char {
    match tile {
        Tile::Empty => '.',
        Tile::Ground => ':',

        Tile::Grass => ',',
        Tile::GrassThick => '"',

        Tile::Rocks => '∩',

        Tile::Hill => '^',

        Tile::Water => '~',
        Tile::Wall => '#',
        Tile::TreeStump => 'o',

        Tile::Ice => '∴',
    }
}

