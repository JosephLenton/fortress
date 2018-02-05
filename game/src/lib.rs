//! # Game
//!
//! This crate contains the algorithms for updating the game. You have a world,
//! creatures, player, time, weather, and so on. Those are things in the game.
//! This does the update from one to the next.
//! 

extern crate world;

mod game;
mod game_tile;
mod game_setup;

pub use self::game::Game;
pub use self::game_setup::GameSetup;
pub use self::game_tile::GameTile;
