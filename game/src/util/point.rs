
pub struct Point {
    pub x : u32,
    pub y : u32,
}

impl Point {
    pub fn new( x : u32, y : u32 ) -> Point {
        return Point {
            x : x,
            y : y,
        }
    }

    pub fn set( &mut self, x : u32, y : u32 ) {
        self.x = x;
        self.y = y;
    }
}

