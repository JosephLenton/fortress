use std::convert::From;

use tiles::Tile;

impl From<Tile> for char {
    fn from(tile: Tile) -> char {
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
}
