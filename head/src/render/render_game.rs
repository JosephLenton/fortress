use render::camera::Camera;
use theme::Theme;

use game::Game;
use game::GameTile;

use util::shapes::Point;

use llr::LLR;

pub struct RenderGame<'a> {
    /// How we get visual setup information.
    theme: &'a Theme,

    /// The game state we are using for rendering.
    game: &'a Game<'a>,

    /// The camera whilst drawing.
    camera: Camera,
}

impl<'a> RenderGame<'a> {
    pub fn new(
        game: &'a Game,
        theme: &'a Theme,
    ) -> RenderGame<'a> {
        RenderGame {
            theme: theme,
            game: game,
            //camera: Camera::new(game.player.position.to_clamped::<i32>()),
            camera: Camera::new(Point::new(0, 0)),
        }
    }

    pub fn move_camera(
        &mut self,
        x: i32,
        y: i32,
    ) {
        self.camera.move_position(Point::new(x, y));
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
        let llr_size = llr.size().to::<i32>();
        let top_left = camera_pos - llr_size/2;
        let area = top_left.combine(llr_size);

        for (tile, tile_pos) in
            self.game.slice(area.x, area.y, area.width as u32, area.height as u32)
        {
            let pos = tile_pos.to_clamped::<i32>() - top_left;
            if 0 <= pos.x && 0 <= pos.y {
                self.tile(llr, tile, pos.to_clamped::<u16>());
            }
        }

        let player_pos = self.game.player.position.to_clamped::<i32>() - top_left;
        if 0 <= player_pos.x && 0 <= player_pos.y {
            self.player(llr, player_pos.to_clamped::<u16>());
        }
    }

    fn player(
        &mut self,
        llr: &mut LLR,
        draw_pos: Point<u16>,
    ) {
        let colour = self.theme.get_player();

        llr.pixel(colour, draw_pos);
    }

    fn tile(
        &mut self,
        llr: &mut LLR,
        tile: GameTile,
        draw_pos: Point<u16>,
    ) {
        let colour = self.theme.get_game_tile(tile);

        llr.pixel(colour, draw_pos);
    }
}
