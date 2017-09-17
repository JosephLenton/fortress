
use render::setup::Setup;

use util::size::Size;

pub struct WindowState {
    pub size : Size,
}

impl WindowState {
    pub fn new(
            setup : & Setup,
    ) -> WindowState {
        return WindowState {
            size : Size::new(
                setup.window_size.width,
                setup.window_size.height,
            ),
        }
    }

    pub fn on_resize( &mut self, w : u32, h : u32 ) {
        self.size = Size::new( w, h );
    }
}

