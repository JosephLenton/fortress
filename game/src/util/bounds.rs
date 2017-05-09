
#[derive(Copy, Clone)]
pub struct Bounds {
    pub x : u32,
    pub y : u32,

    pub width  : u32,
    pub height : u32,
}

impl Bounds {
    pub fn new( x : u32, y : u32, w : u32, h : u32 ) -> Bounds {
        return Bounds {
            x : x,
            y : y,

            width  : w,
            height : h,
        }
    }
}

