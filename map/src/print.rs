
extern crate tiles;

use tiles::map::Map;
use tiles::tile::Tile;
use tiles::tile_colour::tile_to_cmd;

use std::io;

pub struct PrintOptions {
    pub colour : PrintColourOptions,
}

pub enum PrintColourOptions {
    On,
    Off,
}

// The map is map[x][y], but we want to traverse map[y][x].
// This is so we print line by line.
pub fn print_map( options : PrintOptions, map : Map<Tile>, out : &mut io::Write ) {
    let has_colour = match options.colour {
        PrintColourOptions::On  => true,
        PrintColourOptions::Off => false,
    };

    map.each(|tile, x, y| {
        // This is for the previous line, if we just ended it.
        if x == 0 && y > 0 {
            writeln!( out, "\x1B[0m" )
                .expect("failed to write end of line");
        }

        if has_colour {
            let tile_colour = tile_to_cmd( tile );

            write!( out, "{}{}", tile_colour, tile.to_string() )
                .expect( "failed to write tile" );
        } else {
            write!( out, "{}"  ,              tile.to_string() )
                .expect( "failed to write tile" );
        }
    });

    writeln!( out, "\x1B[0m" )
        .expect("failed to set colour back to normal");
}

