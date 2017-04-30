
pub struct Size {
    pub width  : u32,
    pub height : u32,
}

impl Size {
    pub fn new( w : u32, h : u32 ) -> Size {
        return Size {
            width  : w,
            height : h,
        }
    }

    pub fn size( &mut self, w : u32, h : u32 ) {
        self.width  = w;
        self.height = h;
    }
}

