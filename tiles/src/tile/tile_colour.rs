
use tile::tile::Tile;

#[derive(Copy, Clone)]
pub enum TileColour {
    Black,
    White,

    LightRed,
    Red,

    Pink,
    Purple,

    Brown,
    Yellow,

    LightGrey,
    Grey,
    DarkGrey,

    LightCyan,
    Cyan,

    LightBlue,
    Blue,

    LightGreen,
    Green,
}

pub fn tile_to_cmd( tile : Tile ) -> String {
    return colour_to_cmd( tile_to_colour( tile ) );
}

pub fn tile_to_colour( tile : Tile ) -> ( TileColour, TileColour ) {
    return match tile {
        Tile::Empty       => ( TileColour::Black        , TileColour::Grey  ),
        Tile::Ground      => ( TileColour::Black        , TileColour::Brown ),

        Tile::Grass       => ( TileColour::Black        , TileColour::Green ),
        Tile::GrassThick  => ( TileColour::Black        , TileColour::Green ),

        Tile::Rocks       => ( TileColour::Black        , TileColour::LightGrey ),

        Tile::Hill        => ( TileColour::Black        , TileColour::Green ),

        Tile::Water       => ( TileColour::Black        , TileColour::Blue  ),
        Tile::Wall        => ( TileColour::Grey         , TileColour::LightGrey ),
        Tile::TreeStump   => ( TileColour::Black        , TileColour::Brown ),

        Tile::Ice         => ( TileColour::LightCyan    , TileColour::White ),
    }
}

pub fn colour_to_cmd( colour : ( TileColour, TileColour ) ) -> String {
    return colour_to_cmd_background( colour.0 ) + &colour_to_cmd_foreground( colour.1 );
}

fn colour_to_cmd_foreground( colour : TileColour ) -> String {
    return "\x1B[38;2;".to_string() + colour_to_cmd_colour( colour ) + "m"
}

fn colour_to_cmd_background( colour : TileColour ) -> String {
    return "\x1B[48;2;".to_string() + colour_to_cmd_colour( colour ) + "m"
}

fn colour_to_cmd_colour( colour : TileColour ) -> &'static str {
    return match colour {
        TileColour::Black        => "0;0;0",
        TileColour::White        => "255;255;255",

        TileColour::LightRed     => "250;128;144",
        TileColour::Red          => "255;0;0",

        TileColour::Pink         => "255;0;255",
        TileColour::Purple       => "128;0;128",

        TileColour::Brown        => "175;90;35",
        TileColour::Yellow       => "255;215;0",

        TileColour::DarkGrey     => "75;75;75",
        TileColour::Grey         => "120;120;120",
        TileColour::LightGrey    => "180;180;180",

        TileColour::LightCyan    => "0;255;255",
        TileColour::Cyan         => "64;224;208",

        TileColour::LightBlue    => "30;144;255",
        TileColour::Blue         => "0;0;255",

        TileColour::LightGreen   => "0;255;0",
        TileColour::Green        => "50;205;50",
    }
}


