
use std::fmt;

#[derive(Copy, Clone)]
pub enum Tile {
    Ground,
    Grass,
    GrassThick,

    Hill,

    Rocks,

    Water,
    Wall,
    TreeStump,

    Ice,
}

impl Tile {
    pub fn to_char( self ) -> char {
        return match self {
            Tile::Ground        => '.',

            Tile::Grass         => '.',
            Tile::GrassThick    => '"',

            Tile::Rocks         => '∩',

            Tile::Hill          => '^',

            Tile::Water         => '~',
            Tile::Wall          => '#',
            Tile::TreeStump     => 'o',

            Tile::Ice           => '∴',
        }
    }

    pub fn to_string(&self) -> String {
        return self.to_char().to_string();
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.to_char());
    }
}

