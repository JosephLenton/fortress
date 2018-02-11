use util::shapes::Point;

pub struct Camera {
    zoom: f32,
    pos: Point<i32>,
}

impl Camera {
    pub fn new(pos: Point<i32>) -> Camera {
        return Camera {
            zoom: 1.0,
            pos: pos,
        };
    }

    pub fn zoom(&self) -> f32 {
        return self.zoom;
    }

    pub fn zoom_in(&mut self) {
        self.zoom += 0.25;

        if self.zoom > 2.0 {
            self.zoom = 2.0
        }
    }

    pub fn zoom_out(&mut self) {
        self.zoom -= 0.25;

        if self.zoom < 0.25 {
            self.zoom = 0.25
        }
    }

    pub fn position(&self) -> Point<i32> {
        self.pos
    }

    pub fn move_position(
        &mut self,
        pos_move: Point<i32>,
    ) {
        self.pos = self.pos + pos_move;
    }
}
