pub struct Camera {
    zoom: f32,
    x: i32,
    y: i32,
}

impl Camera {
    pub fn new(
        x: i32,
        y: i32,
    ) -> Camera {
        return Camera {
            zoom: 1.0,
            x: x,
            y: y,
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

    pub fn x(&self) -> i32 {
        return self.x;
    }

    pub fn y(&self) -> i32 {
        return self.y;
    }

    pub fn add_xy(
        &mut self,
        x: i32,
        y: i32,
    ) {
        self.x += x;
        self.y += y;
    }
}
