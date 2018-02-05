//! # Generate.
//!
//! This is a command line utility for generating maps.
//! 

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate generate;
extern crate head;

use std::io::stdout;

use args::Args;
use args::ArgsColour;
use generate::generate::MapOptions;
use generate::generate::new_map;
use head::native::cmd::print;

mod args;

/// Entry point.
/// 
fn main() {
    let args = Args::new_from_args();

    let map = new_map(MapOptions {
        width: args.width,
        height: args.height,
        seed: args.seed,
    });

    let mut out = stdout();
    out.lock();

    let colour = if args.colour == Some(ArgsColour::colour) {
        print::OnOff::On
    } else {
        print::OnOff::Off
    };
    print::print_map(colour, &map, &mut out);
}
