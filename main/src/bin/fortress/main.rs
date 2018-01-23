#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

use std::process::exit;
use std::error::Error;
use std::path::Path;
use std::io::Result;
use std::io::BufReader;
use std::fs::File;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate game;
extern crate head;
extern crate util;
extern crate world;

use args::Args;
use world::load;
use world::player::Player;
use world::world_setup::WorldSetup;
use world::calendar::WorldCalendar;
use world::calendar::WorldTime;
use util::shapes::Size;
use game::model::Game;
use game::model::GameSetup;
use head::render::setup::Setup;
use head::render::run::run;

mod args;

struct FortressCalendar {}

impl WorldCalendar for FortressCalendar {
    fn get_time(&self, time: u32) -> WorldTime {
        WorldTime {
            second: (time % 60) as u8,
            minute: 1,
            hour: 1,
            day: 1,
            month: 1,
            year: 1,
        }
    }
}

fn main() {
    let args = Args::new_from_args();
    if !Path::new(&args.map).exists() {
        eprintln!("");
        eprintln!("File not found {}", args.map);
        eprintln!("");

        exit(1);
    }

    match main_run(args) {
        Err(err) => {
            eprintln!("Error, {}", err.description());
            panic!(err);
        }
        Ok(()) => {}
    }
}

fn main_run(args: Args) -> Result<()> {
    let file = File::open(args.map)?;
    let mut file = BufReader::new(file);

    let player = Player::new(22, 18);
    let map = load::read_to_map(&mut file)?;

    let world_setup = WorldSetup {
        calendar: &FortressCalendar {},
    };

    let game_setup = GameSetup { time_tick_speed: 5 };

    let game = Game::new(map, player, world_setup, game_setup);

    let setup = Setup {
        title: "Fortress",

        window_size: Size {
            width: 800,
            height: 600,
        },
        tile_size: Size {
            width: 24,
            height: 24,
        },
    };

    run(setup, &game);

    return Ok(());
}
