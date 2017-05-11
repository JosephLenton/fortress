
use tiles::tile::tile_colour::tile_to_colour;

use render::gfx::GFX;
use render::camera::Camera;
use render::setup::Setup;
use render::window_state::WindowState;

use game::game::Game;

use util::size::Size;
use util::bounds::Bounds;

pub struct RenderGame<'a> {

    /// 
    /// The game state we are using for rendering.
    ///
    game : &'a Game,

    /// 
    /// Used for rendering.
    ///
    /// The size of the tile when drawn to the screen.
    tile_size : Size,

    /// 
    /// The camera whilst drawing.
    /// 
    camera : Camera,

}

impl<'a> RenderGame<'a> {
    pub fn new(
        setup : &   Setup,
        game  : &'a Game,
    ) -> RenderGame<'a> {
        return RenderGame {
            game      : game,
            tile_size : setup.tile_size,
            camera    : Camera::new( game.width / 2, game.height / 2 ),
        }
    }

    pub fn on_mouse_scroll( &mut self, y : f32 ) {
        if y > 0.0 {
            self.camera.zoom_in();
        } else if y < 0.0 {
            self.camera.zoom_out();
        }
    }

    pub fn render(
        &self,
        window  : & WindowState,
        gfx     : & mut GFX,
    ) {
        let zoom          = self.camera.get_zoom();
        let camera_x      = self.camera.get_x();
        let camera_y      = self.camera.get_y();
        let window_width  = window.size.width  as f32;
        let window_height = window.size.height as f32;
        let tile_width    = self.tile_size.width  as f32 * zoom;
        let tile_height   = self.tile_size.height as f32 * zoom;

        // Work out the area that we are rendering.
        // We want to skip areas outside of the window.
        let num_game_tiles_x =   window_width  as f32 / tile_width  as f32 ;
        let num_game_tiles_y =   window_height as f32 / tile_height as f32 ;
        let top_left_x       = ( camera_x as f32 - num_game_tiles_x/2.0 ).floor() as i32;
        let top_left_y       = ( camera_y as f32 - num_game_tiles_y/2.0 ).floor() as i32;
        let bottom_right_x   = ( camera_x as f32 + num_game_tiles_x/2.0 ).ceil() as i32;
        let bottom_right_y   = ( camera_y as f32 + num_game_tiles_y/2.0 ).ceil() as i32;
        let game_width       = ( bottom_right_x - top_left_x ) as u32;
        let game_height      = ( bottom_right_y - top_left_y ) as u32;

        for ( tile, x, y ) in self.game.slice( top_left_x, top_left_y, game_width, game_height ) {
            let draw_x = (window_width  as f32)/2.0 - ( (camera_x as i32 - x as i32) as f32 )*tile_width;
            let draw_y = (window_height as f32)/2.0 - ( (camera_y as i32 - y as i32) as f32 )*tile_height;
            let ( background, foreground ) = tile_to_colour( tile.tile );

            gfx.rectangle(
                    background,
                    [ draw_x, draw_y, tile_width, tile_height ],
            );

            gfx.rectangle(
                    foreground,
                    [ draw_x+tile_width/4.0, draw_y+tile_height/4.0, tile_width/2.0, tile_height/2.0 ],
            );
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

