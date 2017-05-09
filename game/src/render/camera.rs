
pub struct Camera {
    zoom : f64,
    x    : u32,
    y    : u32,
}

impl Camera {
    pub fn new( x : u32, y : u32 ) -> Camera {
        return Camera {
            zoom : 1.0,
            x    : x,
            y    : y,
        }
    }

    pub fn get_zoom( &self ) -> f64 {
        return self.zoom;
    }

    pub fn zoom_in( &mut self ) {
        self.zoom += 0.25;

        if self.zoom > 2.0 {
            self.zoom = 2.0
        }
    }

    pub fn zoom_out( &mut self ) {
        self.zoom -= 0.25;

        if self.zoom < 0.25 {
            self.zoom = 0.25
        }
    }

    pub fn get_x( &self ) -> u32 {
        return self.x;
    }

    pub fn get_y( &self ) -> u32 {
        return self.y;
    }
}

