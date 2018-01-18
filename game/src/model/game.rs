
use world::player::Player;
use world::tile::Tile;
use world::map::Map;
use world::map::MapIterator;

use model::GameTile;

pub struct Game {

    ///
    /// The tiles in our game.
    /// This holds the world data.
    ///
    map : Map<GameTile>,

    ///
    /// The width of our game map.
    ///
    pub width : u32,

    ///
    /// The height of our game map.
    ///
    pub height : u32,

    ///
    /// The player in the world.
    ///
    pub player : Player,

}

impl Game {
    pub fn new(
            map : Map<Tile>,
            player : Player,
    ) -> Game {
        return Game {
            map : map.transform( GameTile::new ),

            width : map.width as u32,
            height : map.height as u32,

            player : player,
        }
    }

    ///
    /// A lot of the world has natural ways to update.
    /// Calling this will cause the world to update.
    ///
    /// This update can range from updating the weather,
    /// to triggerring a random encounter, to causing other
    /// effects.
    ///
    pub fn tick() {
        // todo
    }

    pub fn slice( &self, x : i32, y : i32, w : u32, h : u32 ) -> MapIterator<GameTile> {
        return self.map.slice( x, y, w, h );
    }
}

