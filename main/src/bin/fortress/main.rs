
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

use std::process::exit;
use std::error::Error;
use std::path::Path;
use std::io::Result;
use std::io::BufReader;
use std::fs::File;

#[macro_use]
extern crate structopt_derive;
extern crate structopt;

extern crate fortress;
extern crate util;
extern crate game;
extern crate head;

use args::Args;
use fortress::load;
use util::shapes::size::Size;
use game::model::Game;
use head::render::setup::Setup;
use head::render::run::run;

mod args;

fn main() {
    let args = Args::new_from_args();
    if ! Path::new( & args.map ).exists() {
        eprintln!("");
        eprintln!("File not found {}", args.map);
        eprintln!("");

        exit(1);
    }

    match main_run( args ) {
        Err(err) => {
            eprintln!("Error, {}", err.description());
            panic!(err);
        },
        Ok(()) => {}
    }
}

fn main_run(
    args : Args,
) -> Result<()> {
    let file = File::open( args.map )?;
    let mut file = BufReader::new( file );

    let map   = load::read_to_map( &mut file )?;
    let game  = Game::new( map );
    let setup = Setup {
        title : "Fortress",

        window_size : Size { width: 800, height: 600 },
        tile_size   : Size { width:  24, height:  24 },
    };

    run( setup, game );

    return Ok(());
}

