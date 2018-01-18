
use world::colour::Colour;
use util::colour::RGBA;

pub static BLACK        : RGBA = RGBA { red :   0, green :   0, blue :   0, alpha : 255, };
pub static WHITE        : RGBA = RGBA { red : 255, green : 255, blue : 255, alpha : 255, };

pub static LIGHT_RED    : RGBA = RGBA { red : 250, green : 128, blue : 144, alpha : 255, };
pub static RED          : RGBA = RGBA { red : 255, green :   0, blue :   0, alpha : 255, };

pub static PINK         : RGBA = RGBA { red : 255, green :   0, blue : 255, alpha : 255, };
pub static PURPLE       : RGBA = RGBA { red : 128, green :   0, blue : 128, alpha : 255, };

pub static BROWN        : RGBA = RGBA { red : 175, green :  90, blue :  35, alpha : 255, };
pub static YELLOW       : RGBA = RGBA { red : 255, green : 215, blue :   0, alpha : 255, };

pub static DARK_GREY    : RGBA = RGBA { red :  75, green :  75, blue :  75, alpha : 255, };
pub static GREY         : RGBA = RGBA { red : 120, green : 120, blue : 120, alpha : 255, };
pub static LIGHT_GREY   : RGBA = RGBA { red : 180, green : 180, blue : 180, alpha : 255, };

pub static LIGHT_CYAN   : RGBA = RGBA { red :   0, green : 255, blue : 255, alpha : 255, };
pub static CYAN         : RGBA = RGBA { red :  64, green : 224, blue : 208, alpha : 255, };

pub static LIGHT_BLUE   : RGBA = RGBA { red :  30, green : 144, blue : 255, alpha : 255, };
pub static BLUE         : RGBA = RGBA { red :   0, green :   0, blue : 255, alpha : 255, };

pub static LIGHT_GREEN  : RGBA = RGBA { red :   0, green : 255, blue :   0, alpha : 255, };
pub static GREEN        : RGBA = RGBA { red :  50, green : 205, blue :  50, alpha : 255, };

pub trait ToRGBA {
    fn to_rgba( self ) -> RGBA;
}

impl ToRGBA for Colour {
    fn to_rgba( self ) -> RGBA {
        match self {
            Colour::Black        => BLACK,
            Colour::White        => WHITE,

            Colour::LightRed     => LIGHT_RED,
            Colour::Red          => RED,

            Colour::Pink         => PINK,
            Colour::Purple       => PURPLE,

            Colour::Brown        => BROWN,
            Colour::Yellow       => YELLOW,

            Colour::DarkGrey     => DARK_GREY,
            Colour::Grey         => GREY,
            Colour::LightGrey    => LIGHT_GREY,

            Colour::LightCyan    => LIGHT_CYAN,
            Colour::Cyan         => CYAN,

            Colour::LightBlue    => LIGHT_BLUE,
            Colour::Blue         => BLUE,

            Colour::LightGreen   => LIGHT_GREEN,
            Colour::Green        => GREEN,
        }
    }
}

