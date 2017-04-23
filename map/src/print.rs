
extern crate tiles;

use std::io;
use map::Map;

// The map is map[x][y], but we want to traverse map[y][x].
// This is so we print line by line.
pub fn print_map( map : Map, out : &mut io::Write ) -> io::Result<()> {
    let width  = map.len();
    let height = map[0].len();

    for y in 0 .. height {
        for x in 0 .. width {
            let tile = map[x][y];

            write!( out, "{}", tile.to_string() );
        }

        writeln!( out, "" );
    }

    out.flush();

    Ok(())
}

