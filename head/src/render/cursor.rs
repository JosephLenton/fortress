
pub struct Cursor {
    pub x : u32,
    pub y : u32,

    pub start_x : u32,
    pub start_y : u32,

    pub is_down : bool,
}

impl Cursor {
    pub fn new( x : u32, y : u32 ) -> Cursor {
        return Cursor {
            x : x,
            y : y,

            start_x : x,
            start_y : y,

            is_down : false,
        }
    }

    pub fn to_xy( &self ) -> (i32, i32) {
        return ( self.x as i32, self.y as i32 );
    }

    pub fn set_down( &mut self ) {
        self.start_x = self.x;
        self.start_y = self.y;
        self.is_down = true;
    }

    pub fn set_up( &mut self ) {
        self.is_down = false;
    }

    pub fn xy( &mut self, x : u32, y : u32 ) {
        self.x = x;
        self.y = y;

        if ! self.is_down {
            self.start_x = x;
            self.start_y = y;
        }
    }
}

