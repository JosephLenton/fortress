
extern crate tiles;

mod map;
mod print;

use std::io;

fn main() {
    let map_options = map::MapOptions {
        width  : 50,
        height : 50,
        seed   : None,
    };

    let     map = map::new_map( map_options );
    let mut out = io::stdout();

    print::print_map( map, &mut out );
}

