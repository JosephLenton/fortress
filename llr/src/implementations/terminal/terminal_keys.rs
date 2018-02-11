use LLRKey;

pub const ARROW_UP: u8 = 72;
pub const ARROW_DOWN: u8 = 80;
pub const ARROW_LEFT: u8 = 75;
pub const ARROW_RIGHT: u8 = 77;

pub const ESCAPE: u8 = 27;
pub const LETTER_Q: u8 = 113;

pub const LETTER_W: u8 = 119;
pub const LETTER_S: u8 = 115;
pub const LETTER_A: u8 = 97;
pub const LETTER_D: u8 = 100;

pub const SPACE: u8 = 32;

pub fn u8_to_key(key: u8) -> Option<LLRKey> {
    match key {
        ESCAPE => Some(LLRKey::Escape),

        LETTER_Q => Some(LLRKey::Q),

        LETTER_W => Some(LLRKey::W),

        LETTER_S => Some(LLRKey::S),

        LETTER_A => Some(LLRKey::A),

        LETTER_D => Some(LLRKey::D),

        ARROW_UP => Some(LLRKey::Up),
        ARROW_DOWN => Some(LLRKey::Down),
        ARROW_LEFT => Some(LLRKey::Left),
        ARROW_RIGHT => Some(LLRKey::Right),

        SPACE => Some(LLRKey::Space),

        _ => None,
    }
}
