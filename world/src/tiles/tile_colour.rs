
use colour::Colour;
use tiles::Tile;

/// The colour for a tile.
///
/// It combines a background and a foreground.
#[derive(Copy, Clone)]
pub struct TileColour {
    /// The background colour.
    pub background : Colour,

    /// The foreground colour.
    pub foreground : Colour,
}

/// Converts the tile to it's colour, and then returns the command version
/// of that colour.
pub fn tile_to_cmd(tile: Tile) -> String {
    colour_to_cmd(tile_to_colour(tile))
}

/// Converts the tile into two colours; a back and front colour.
/// These two colours are returned.
pub fn tile_to_colour(tile: Tile) -> TileColour {
    match tile {
        Tile::Empty => TileColour { background: Colour::Black, foreground: Colour::Grey },
        Tile::Ground => TileColour { background: Colour::Black, foreground: Colour::Brown },

        Tile::Grass => TileColour { background: Colour::Black, foreground: Colour::Green },
        Tile::GrassThick => TileColour { background: Colour::Black, foreground: Colour::Green },

        Tile::Rocks => TileColour { background: Colour::Black, foreground: Colour::LightGrey },

        Tile::Hill => TileColour { background: Colour::Black, foreground: Colour::Green },

        Tile::Water => TileColour { background: Colour::Black, foreground: Colour::Blue },
        Tile::Wall => TileColour { background: Colour::Grey, foreground: Colour::LightGrey },
        Tile::TreeStump => TileColour { background: Colour::Black, foreground: Colour::Brown },

        Tile::Ice => TileColour { background: Colour::LightCyan, foreground: Colour::White },
    }
}

/// Given a colour, this returns the string version for use on the command line.
/// That string will contain the colour code for this tile.
pub fn colour_to_cmd(colour: TileColour) -> String {
    colour_to_cmd_background(colour.background) + &colour_to_cmd_foreground(colour.foreground)
}

fn colour_to_cmd_foreground(colour: Colour) -> String {
    "\x1B[38;2;".to_string() + colour_to_cmd_colour(colour) + "m"
}

fn colour_to_cmd_background(colour: Colour) -> String {
    "\x1B[48;2;".to_string() + colour_to_cmd_colour(colour) + "m"
}

fn colour_to_cmd_colour(colour: Colour) -> &'static str {
    match colour {
        Colour::Black => "0;0;0",
        Colour::White => "255;255;255",

        Colour::LightRed => "250;128;144",
        Colour::Red => "255;0;0",

        Colour::Pink => "255;0;255",
        Colour::Purple => "128;0;128",

        Colour::Brown => "175;90;35",
        Colour::Yellow => "255;215;0",

        Colour::DarkGrey => "75;75;75",
        Colour::Grey => "120;120;120",
        Colour::LightGrey => "180;180;180",

        Colour::LightCyan => "0;255;255",
        Colour::Cyan => "64;224;208",

        Colour::LightBlue => "30;144;255",
        Colour::Blue => "0;0;255",

        Colour::LightGreen => "0;255;0",
        Colour::Green => "50;205;50",
    }
}

