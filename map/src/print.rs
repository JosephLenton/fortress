
extern crate tiles;

use tiles::map::Map;
use tiles::tile_colour::TileColour;

use std::io;

// The map is map[x][y], but we want to traverse map[y][x].
// This is so we print line by line.
pub fn print_map( map : Map, out : &mut io::Write ) {
    map.each(|tile, x, y| {
        let tile_colour = tile_colour_to_cmd( tile.colour );

        // This is for the previous line, if we just ended it.
        if x == 0 && y > 0 {
            writeln!( out, "" )
                .expect("failed to write end of line");
        }

        write!( out, "{}{}", tile_colour, tile.to_string() )
            .expect( "failed to write tile" );
    });

    writeln!( out, "\x1B[0m" )
        .expect("failed to set colour back to normal");

    out
        .flush()
        .expect("failed to flush");
}

fn tile_colour_to_cmd( colour : TileColour ) -> &'static str {
    return match colour {
        TileColour::Black        => "\x1B[0;30m",
        TileColour::White        => "\x1B[1;37m",

        TileColour::LightRed     => "\x1B[1;31m",
        TileColour::Red          => "\x1B[0;31m",

        TileColour::Pink         => "\x1B[1;35m",
        TileColour::Purple       => "\x1B[0;35m",

        TileColour::Brown        => "\x1B[0;33m",
        TileColour::Yellow       => "\x1B[1;33m",

        TileColour::Grey         => "\x1B[0;37m",
        TileColour::DarkGrey     => "\x1B[1;30m",

        TileColour::LightCyan    => "\x1B[1;36m",
        TileColour::Cyan         => "\x1B[0;36m",

        TileColour::LightBlue    => "\x1B[1;34m",
        TileColour::Blue         => "\x1B[0;34m",

        TileColour::Green        => "\x1B[0;32m",
        TileColour::LightGreen   => "\x1B[1;32m",

        TileColour::GreyAndBlack => "\x1B[0;47m\x1B[5;30m",
        TileColour::PinkAndBlack => "\x1B[0;45m\x1B[5;30m",
    }
}

