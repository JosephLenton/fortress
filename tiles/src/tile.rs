
use tile_colour::TileColour;
use std::fmt;

pub const GROUND : Tile = Tile { display: '.', colour: TileColour::Brown };

pub const HILL   : Tile = Tile { display: '^', colour: TileColour::Green };
pub const GRASS  : Tile = Tile { display: '"', colour: TileColour::Green };

pub const ROCKS  : Tile = Tile { display: 'n', colour: TileColour::Grey  };

pub const WATER  : Tile = Tile { display: '~', colour: TileColour::Blue  };

pub const WALL   : Tile = Tile { display: '#', colour: TileColour::GreyAndBlack };

#[derive(Copy, Clone)]
pub struct Tile {
    pub display : char,
    pub colour  : TileColour,
}

impl Tile {
    #[inline]
    pub fn to_char(self) -> char {
        return self.display;
    }

    #[inline]
    pub fn to_string(self) -> String {
        return self.display.to_string();
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.display);
    }
}

