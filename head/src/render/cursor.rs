use util::shapes::Point;

pub struct Cursor {
    pos: Point<f32>,
    start_pos: Point<f32>,

    is_down: bool,
}

impl Cursor {
    pub fn new(
        x: f32,
        y: f32,
    ) -> Cursor {
        return Cursor {
            pos: Point::new(x, y),
            start_pos: Point::new(x, y),

            is_down: false,
        };
    }

    pub fn to_xy(&self) -> Point<f32> {
        return self.pos;
    }

    pub fn set_down(&mut self) {
        self.start_pos = self.pos;
        self.is_down = true;
    }

    pub fn set_up(&mut self) {
        self.is_down = false;
    }

    pub fn xy(
        &mut self,
        pos: Point<f32>,
    ) {
        self.pos = pos;
    }
}
