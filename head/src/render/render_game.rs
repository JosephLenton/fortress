use render::camera::Camera;
use theme::Theme;

use game::Game;
use game::GameTile;

use util::shapes::Point2;
use util::shapes::Rect;
use util::shapes::Size;

use llr::LLR;

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
    pub fn new(
        game: &'a Game,
        theme: &'a Theme,
        tile_size: Size<u32>,
        window_size: Size<u32>,
    ) -> RenderGame<'a> {
        RenderGame {
            theme: theme,
            game: game,
            tile_size: Size::new(tile_size.width as f32, tile_size.height as f32),
            camera: Camera::new((game.width / 2) as i32, (game.height / 2) as i32),
            window_size: window_size,
        }
    }

    pub fn on_resize(
        &mut self,
        w: u32,
        h: u32,
    ) {
        self.window_size = Size::new(w, h);
    }

    pub fn move_camera(
        &mut self,
        x: i32,
        y: i32,
    ) {
        self.camera.add_xy(x, y);
    }

    pub fn render(
        &mut self,
        llr: &mut LLR,
    ) {
        llr.clear();
        self.render_game(llr);
        llr.finished_drawing();
    }

    pub fn render_game(
        &mut self,
        llr: &mut LLR,
    ) {
        let camera_pos = self.camera.position();
        let llr_size = llr.size();

        // Work out the area that we are rendering.
        // We want to skip areas outside of the window.
        let top_left_x = ((camera_pos.x as f32) - (llr_size.width as f32) / 2.0).floor() as i32;
        let top_left_y = ((camera_pos.y as f32) - (llr_size.height as f32) / 2.0).floor() as i32;
        let llr_width = llr_size.width + 1;
        let llr_height = llr_size.height + 1;

        for (tile, draw_pos) in self.game.slice(top_left_x, top_left_y, llr_width, llr_height) {
            let draw_pos = camera_pos - tile_pos;
            self.tile(llr, tile, pos);
        }

        let player_draw_pos = camera_pos - self.game.player.position;
        self.player(llr, player_pos);
    }

    fn player(
        &mut self,
        llr: &mut LLR,
        draw_pos: Point2<f32>,
    ) {
        let colour = self.theme.get_player();

        llr.pixel(colour, draw_pos);
    }

    fn tile(
        &mut self,
        llr: &mut LLR,
        tile: GameTile,
        draw_pos: Point2<f32>,
    ) {
        let colour = self.theme.get_game_tile(tile);

        llr.pixel(colour, draw_pos);
    }
}
