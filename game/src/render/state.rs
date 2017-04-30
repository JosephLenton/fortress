
use render::setup::Setup;

use std::cmp::min;
use std::cmp::max;
use util::size::Size;
use util::point::Point;

pub struct State {
    pub window_size : Size,

    /// 
    /// The Users view of the world.
    /// The location is in game tiles.
    /// 
    pub camera : Point,
    pub cursor : Cursor,
    pub zoom   : Zoom,
}

impl State {
    pub fn new(
            setup  : & Setup,
            camera : Point,
    ) -> State {
        return State {
            window_size : Size::new(
                setup.window_size.width,
                setup.window_size.height,
            ),

            camera : camera,
            cursor : Cursor::new(
                setup.window_size.width / 2,
                setup.window_size.height / 2,
            ),

            zoom : Zoom::new(),
        }
    }
}

pub struct Zoom {
    pub zoom : f64,
}

impl Zoom {
    pub fn new() -> Zoom {
        return Zoom {
            zoom : 1.0
        }
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
}

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

    pub fn set_down( &mut self ) {
        self.start_x = self.x;
        self.start_y = self.y;
        self.is_down = true;
    }

    pub fn set_up( &mut self ) {
        self.is_down = false;
    }

    pub fn set( &mut self, x : u32, y : u32 ) {
        self.x = x;
        self.y = y;

        if ! self.is_down {
            self.start_x = x;
            self.start_y = y;
        }
    }
}

