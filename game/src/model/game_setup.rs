
///
/// Describes parameters for the running of the game.
///
/// This does not include data like creatures, the player,
/// maps, and so on.
///
/// This are things like how fast the game should run.
/// Debug flag. Things like that.
///
pub struct GameSetup {

    ///
    /// When the game performs a tick, it will increment the current time.
    /// This is how much to increment by. In seconds.
    ///
    pub time_tick_speed : u32,

}

