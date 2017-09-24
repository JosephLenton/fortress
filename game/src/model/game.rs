
use fortress::tile::Tile;
use fortress::map::Map;
use fortress::map::MapIterator;

use model::GameTile;

pub struct Game {
    map : Map<GameTile>,

    pub width  : u32,
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

