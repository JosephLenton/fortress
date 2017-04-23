
extern crate tiles;

mod map;
mod print;

use std::io;

fn main() {
    let mut mapOptions = map::MapOptions {
        width  : 50,
        height : 50,
        seed   : None
    };

    let     map = map::new_map( mapOptions );
    let mut out = io::stdout();

    let _ = print::print_map( map, &mut out );
}

