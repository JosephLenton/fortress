
use std::fmt;

#[derive(Copy, Clone)]
pub struct Tile {
    pub display : char
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

pub const GROUND : Tile = Tile { display: '.' };
pub const HILL   : Tile = Tile { display: '^' };
pub const GRASS  : Tile = Tile { display: '"' };

pub const ROCKS  : Tile = Tile { display: 'n' };

pub const WATER  : Tile = Tile { display: '~' };

