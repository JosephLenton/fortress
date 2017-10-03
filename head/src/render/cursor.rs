
use util::shapes::Point2;

pub struct Cursor {
    pos : Point2<f32>,
    start_pos : Point2<f32>,

    is_down : bool,
}

impl Cursor {
    pub fn new( x : f32, y : f32 ) -> Cursor {
        return Cursor {
            pos : Point2::new( x, y ),
            start_pos : Point2::new( x, y ),

            is_down : false,
        }
    }

    pub fn to_xy( &self ) -> Point2<f32> {
        return self.pos;
    }

    pub fn set_down( &mut self ) {
        self.start_pos = self.pos;
        self.is_down   = true;
    }

    pub fn set_up( &mut self ) {
        self.is_down = false;
    }

    pub fn xy( &mut self, pos : Point2<f32> ) {
        self.pos = pos;
    }
}

