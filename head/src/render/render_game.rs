use theme::Theme;
use render::gfx::GFX;
use render::camera::Camera;
use render::setup::Setup;

use game::Game;
use game::GameTile;

use util::shapes::Size;
use util::shapes::Point2;
use util::shapes::Rect;

pub struct RenderGame<'a> {
    /// How we get visual setup information.
    theme: &'a Theme,

    /// The game state we are using for rendering.
    game: &'a Game<'a>,

    /// Current size of the Window.
    window_size: Size<u32>,

    /// Used for rendering.
    ///
    /// The size of the tile when drawn to the screen.
    tile_size: Size<f32>,

    /// The camera whilst drawing.
    camera: Camera,
}

impl<'a> RenderGame<'a> {
    pub fn new(setup: &Setup, game: &'a Game, theme: &'a Theme) -> RenderGame<'a> {
        return RenderGame {
            theme: theme,
            game: game,
            tile_size: Size::new(setup.tile_size.width as f32, setup.tile_size.height as f32),
            camera: Camera::new((game.width / 2) as i32, (game.height / 2) as i32),
            window_size: Size::new(setup.window_size.width, setup.window_size.height),
        };
    }

    pub fn on_resize(&mut self, w: u32, h: u32) {
        self.window_size = Size::new(w, h);
    }

    pub fn move_camera(&mut self, x: i32, y: i32) {
        self.camera.add_xy(x, y);
    }

    pub fn render(&self, gfx: &mut GFX) {
        let camera_x = self.camera.x();
        let camera_y = self.camera.y();
        let window_width = self.window_size.width as f32;
        let window_height = self.window_size.height as f32;
        let tile_size = self.tile_size;

        // Work out the area that we are rendering.
        // We want to skip areas outside of the window.
        let num_game_tiles_x = window_width / tile_size.width;
        let num_game_tiles_y = window_height / tile_size.height;
        let top_left_x = ((camera_x as f32) - num_game_tiles_x / 2.0).floor() as i32;
        let top_left_y = ((camera_y as f32) - num_game_tiles_y / 2.0).floor() as i32;
        let bottom_right_x = ((camera_x as f32) + num_game_tiles_x / 2.0).ceil() as i32;
        let bottom_right_y = ((camera_y as f32) + num_game_tiles_y / 2.0).ceil() as i32;
        let game_width = (bottom_right_x - top_left_x) as u32;
        let game_height = (bottom_right_y - top_left_y) as u32;

        for (tile, x, y) in self.game
            .slice(top_left_x, top_left_y, game_width, game_height)
        {
            let pos = Point2::new(
                window_width / 2.0 - ((camera_x - x as i32) as f32) * tile_size.width,
                window_height / 2.0 - ((camera_y - y as i32) as f32) * tile_size.height,
            );

            self.tile(gfx, tile, pos, tile_size);
        }

        let player_pos = Point2::new(
            window_width / 2.0
                - ((camera_x - self.game.player.position.x as i32) as f32) * tile_size.width,
            window_height / 2.0
                - ((camera_y - self.game.player.position.x as i32) as f32) * tile_size.height,
        );
        self.player(gfx, player_pos, tile_size);
    }

    fn player(&self, gfx: &mut GFX, pos: Point2<f32>, size: Size<f32>) {
        let draw_pos = (pos - size / 2.0).to_rect(size);
        let colour = self.theme.get_player_colour();

        gfx.rectangle(colour, draw_pos);
    }

    fn tile(&self, gfx: &mut GFX, tile: GameTile, pos: Point2<f32>, size: Size<f32>) {
        let colour = self.theme.get_game_tile_colour(tile);

        let draw_back = (pos - size / 2.0).to_rect(size);
        let draw_front = (pos - size / 4.0).to_rect(size / 2.0);

        gfx.rectangle(colour.background, draw_back);
        gfx.rectangle(colour.foreground, draw_front);
    }

    pub fn translate_window_to_tile_xy(&self, pos: Point2<f32>) -> Rect<f32> {
        let tile_size = self.tile_size;
        let tile_pos = pos - (pos % tile_size);

        return pos.to_rect(tile_size);
    }

    pub fn translate_window_to_tile_xy_inner(&self, pos: Point2<f32>) -> Rect<f32> {
        let pos = self.translate_window_to_tile_xy(pos);

        return (pos + Point2::new(2.0, 2.0)) - Size::new(4.0, 4.0);
    }
}
