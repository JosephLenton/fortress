
use std::vec::Vec;

use world::tile::Tile;
use world::map::Map;
use world::map::MapIterator;

use model::GameTile;

use util::shapes::Point3;

pub struct Game {

    ///
    /// The tiles in our game.
    /// This holds the world data.
    ///
    map : Map<GameTile>,

    ///
    /// The width of our game map.
    ///
    pub width  : u32,

    ///
    /// The height of our game map.
    ///
    pub height : u32,

}

impl Game {
    pub fn new( map : Map<Tile> ) -> Game {
        return Game {
            map : map.transform( GameTile::new ),

            width  : map.width  as u32,
            height : map.height as u32,
        }
    }

    pub fn slice( &self, x : i32, y : i32, w : u32, h : u32 ) -> MapIterator<GameTile> {
        return self.map.slice( x, y, w, h );
    }
}

