
use fortress::tile::tile_colour::tile_to_colour;

use render::gfx::GFX;
use render::camera::Camera;
use render::setup::Setup;

use game::model::Game;

use util::shapes::size::Size;

pub struct RenderGame<'a> {

    ///
    /// The game state we are using for rendering.
    ///
    game : &'a Game,

    ///
    /// Current size of the Window.
    ///
    window_size : Size<u32>,

    ///
    /// Used for rendering.
    ///
    /// The size of the tile when drawn to the screen.
    tile_size : Size<u32>,

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
            game        : game,
            tile_size   : setup.tile_size,
            camera      : Camera::new( (game.width / 2) as i32, (game.height / 2) as i32 ),
            window_size : Size::new(
                setup.window_size.width,
                setup.window_size.height,
            ),
        }
    }

    pub fn on_resize( &mut self, w : u32, h : u32 ) {
        self.window_size = Size::new( w, h );
    }

    pub fn on_mouse_scroll( &mut self, y : i32 ) {
        if y > 0 {
            self.camera.zoom_in();
        } else if y < 0 {
            self.camera.zoom_out();
        }
    }

    pub fn move_camera( &mut self, x : i32, y : i32 ) {
        self.camera.add_xy( x, y );
    }

    pub fn render(
        &self,
        gfx : & mut GFX,
    ) {
        let zoom          = self.camera.zoom();
        let camera_x      = self.camera.x();
        let camera_y      = self.camera.y();
        let window_width  = self.window_size.width  as f32;
        let window_height = self.window_size.height as f32;
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
                    ( draw_x as i32, draw_y as i32, tile_width as i32, tile_height as i32 ),
            );

            gfx.rectangle(
                    foreground,
                    ( (draw_x+tile_width/4.0) as i32, (draw_y+tile_height/4.0) as i32, (tile_width/2.0) as i32, (tile_height/2.0) as i32 ),
            );
        }
    }

    pub fn translate_window_to_tile_xy( &self, (x, y) : (i32, i32) ) -> (i32, i32, i32, i32) {
        let zoom        = self.camera.zoom();
        let tile_width  = ( self.tile_size.width  as f32 * zoom ) as i32;
        let tile_height = ( self.tile_size.height as f32 * zoom ) as i32;

        let tile_x = x - (x % tile_width );
        let tile_y = y - (y % tile_height);

        return ( tile_x, tile_y, tile_width, tile_height );
    }

    pub fn translate_window_to_tile_xy_inner( &self, xy : (i32, i32) ) -> (i32, i32, i32, i32) {
        let (x, y, w, h) = self.translate_window_to_tile_xy( xy );

        return (x + 2, y + 2, w - 4, h - 4);
    }
}

