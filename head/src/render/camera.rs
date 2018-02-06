
use util::shapes::Point2;

pub struct Camera {
    zoom: f32,
    pos: Point2<i32>,
}

impl Camera {
    pub fn new(
        pos : Point2<i32>,
    ) -> Camera {
        return Camera {
            zoom: 1.0,
            pos : pos,
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

    pub fn position() -> Point2<i32> {
        self.pos
    }

    pub fn move(
        &mut self,
        move: Point2<u32>,
    ) {
        self.pos += move;
    }
}
