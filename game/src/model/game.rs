use world::player::Player;
use world::tiles::Tile;
use world::map::Map;
use world::map::MapIterator;
use world::world_setup::WorldSetup;
use world::calendar::WorldTime;

use model::GameTile;
use model::GameSetup;

pub struct Game<'a> {
    ///
    /// The tiles in our game.
    /// This holds the world data.
    ///
    map: Map<GameTile>,

    ///
    /// The width of our game map.
    ///
    pub width: u32,

    ///
    /// The height of our game map.
    ///
    pub height: u32,

    ///
    /// The player in the world.
    ///
    pub player: Player,

    ///
    /// The current time. In seconds.
    ///
    time: u32,

    ///
    /// A setup or description of the world.
    /// Like it's calendar, and things like that.
    ///
    world_setup: WorldSetup<'a>,

    ///
    /// Setup of the game for it's running.
    ///
    game_setup: GameSetup,
}

impl<'a> Game<'a> {
    pub fn new(
        map: Map<Tile>,
        player: Player,
        world_setup: WorldSetup<'a>,
        game_setup: GameSetup,
    ) -> Game {
        Game {
            map: map.transform(GameTile::new),

            width: map.width,
            height: map.height,

            player: player,

            time: 0,

            world_setup: world_setup,
            game_setup: game_setup,
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
    pub fn tick(&mut self) -> () {
        self.time += self.game_setup.time_tick_speed;
    }

    pub fn get_time(&self) -> WorldTime {
        self.world_setup.calendar.get_time(self.time)
    }

    pub fn slice(&self, x: i32, y: i32, w: u32, h: u32) -> MapIterator<GameTile> {
        self.map.slice(x, y, w, h)
    }
}
