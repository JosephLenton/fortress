
use std::fmt;

use colour::Colour;
use tile::Tile;
use tile::tile_colour::tile_to_colour;

#[derive(Copy, Clone)]
pub struct DisplayTile {
    pub tile       : Tile,
    pub display    : char,
    pub foreground : Colour,
    pub background : Colour,
}

impl DisplayTile {
    pub fn new( tile : Tile ) -> DisplayTile {
        let ( foreground, background ) = tile_to_colour( tile );

        return DisplayTile {
            tile    : tile,
            display : tile_to_char( tile ),

            foreground : foreground,
            background : background,
        }
    }

    pub fn to_char( self ) -> char {
        return self.display;
    }

    pub fn to_string(&self) -> String {
        return self.to_char().to_string();
    }
}

impl fmt::Display for DisplayTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.to_char());
    }
}

pub fn tile_to_char( tile : Tile ) -> char {
    return match tile {
        Tile::Empty         => '.',
        Tile::Ground        => ':',

        Tile::Grass         => ',',
        Tile::GrassThick    => '"',

        Tile::Rocks         => '∩',

        Tile::Hill          => '^',

        Tile::Water         => '~',
        Tile::Wall          => '#',
        Tile::TreeStump     => 'o',

        Tile::Ice           => '∴',
    }
}

