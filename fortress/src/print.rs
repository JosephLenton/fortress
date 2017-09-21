
use map::Map;
use tile::tile::Tile;
use tile::tile_colour::tile_to_cmd;
use tile::display_tile::tile_to_char;

use std::io;

#[derive(PartialEq, Eq)]
pub enum PrintColourOptions {
    On,
    Off,
}

pub fn print_map( colour : PrintColourOptions, map : & Map<Tile>, out : &mut io::Write ) {
    let has_colour = colour == PrintColourOptions::On;

    for (tile, x, y) in map.slice_all() {
        let tile_char = tile_to_char( tile );

        // This is for the previous line, if we just ended it.
        if x == 0 && y > 0 {
            write_end_of_line( out, has_colour, "failed to write end of line during print" );
        }

        if has_colour {
            let tile_colour = tile_to_cmd( tile );

            write!( out, "{}{}", tile_colour, tile_char )
                .expect( "failed to write tile" );
        } else {
            write!( out, "{}"  ,              tile_char )
                .expect( "failed to write tile" );
        }
    }

    write_end_of_line( out, has_colour, "failed to perform final end of line after print" );
}

fn write_end_of_line( out : &mut io::Write, has_colour : bool, msg : &'static str ) {
    match has_colour {
        true  => writeln!( out, "\x1B[0m" ).expect( msg ),
        false => writeln!( out, ""        ).expect( msg ),
    }
}

