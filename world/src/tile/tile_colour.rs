
use colour::Colour;
use tile::Tile;

pub fn tile_to_cmd( tile : Tile ) -> String {
    return colour_to_cmd( tile_to_colour( tile ) );
}

pub fn tile_to_colour( tile : Tile ) -> ( Colour, Colour ) {
    return match tile {
        Tile::Empty       => ( Colour::Black        , Colour::Grey  ),
        Tile::Ground      => ( Colour::Black        , Colour::Brown ),

        Tile::Grass       => ( Colour::Black        , Colour::Green ),
        Tile::GrassThick  => ( Colour::Black        , Colour::Green ),

        Tile::Rocks       => ( Colour::Black        , Colour::LightGrey ),

        Tile::Hill        => ( Colour::Black        , Colour::Green ),

        Tile::Water       => ( Colour::Black        , Colour::Blue  ),
        Tile::Wall        => ( Colour::Grey         , Colour::LightGrey ),
        Tile::TreeStump   => ( Colour::Black        , Colour::Brown ),

        Tile::Ice         => ( Colour::LightCyan    , Colour::White ),
    }
}

pub fn colour_to_cmd( colour : ( Colour, Colour ) ) -> String {
    return colour_to_cmd_background( colour.0 ) + &colour_to_cmd_foreground( colour.1 );
}

fn colour_to_cmd_foreground( colour : Colour ) -> String {
    return "\x1B[38;2;".to_string() + colour_to_cmd_colour( colour ) + "m"
}

fn colour_to_cmd_background( colour : Colour ) -> String {
    return "\x1B[48;2;".to_string() + colour_to_cmd_colour( colour ) + "m"
}

fn colour_to_cmd_colour( colour : Colour ) -> &'static str {
    return match colour {
        Colour::Black        => "0;0;0",
        Colour::White        => "255;255;255",

        Colour::LightRed     => "250;128;144",
        Colour::Red          => "255;0;0",

        Colour::Pink         => "255;0;255",
        Colour::Purple       => "128;0;128",

        Colour::Brown        => "175;90;35",
        Colour::Yellow       => "255;215;0",

        Colour::DarkGrey     => "75;75;75",
        Colour::Grey         => "120;120;120",
        Colour::LightGrey    => "180;180;180",

        Colour::LightCyan    => "0;255;255",
        Colour::Cyan         => "64;224;208",

        Colour::LightBlue    => "30;144;255",
        Colour::Blue         => "0;0;255",

        Colour::LightGreen   => "0;255;0",
        Colour::Green        => "50;205;50",
    }
}


