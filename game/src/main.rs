
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

use std::error::Error;
use std::io::Result;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

extern crate sdl2;

#[macro_use]
extern crate structopt_derive;
extern crate structopt;

extern crate tiles;

use args::Args;
use tiles::load;
use game::game::Game;
use render::setup::Setup;
use util::size::Size;

mod args;
mod game;
mod render;
mod util;

fn main() {
    match main_run() {
        Err(err) => {
            eprintln!("Error, {}", err.description());
            panic!(err);
        },
        Ok(()) => {}
    }
}

fn main_run() -> Result<()> {
    let args = Args::new_from_args();

    let file = File::open( args.map )?;
    let mut file = BufReader::new( file );

    let map   = load::read_to_map( &mut file )?;
    let game  = Game::new( map );
    let setup = Setup {
        title : "Fortress",

        window_size : Size { width: 800, height: 600 },
        tile_size   : Size { width:  24, height:  24 },
    };

    render::run::run( setup, game );

    return Ok(());
}

