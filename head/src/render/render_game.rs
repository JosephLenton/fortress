use render::camera::Camera;
use theme::Theme;

use game::Game;
use game::GameTile;

use util::shapes::Point2;

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
            // camera: Camera::new(Point2::new(game.player.position.x as i32,
            // game.player.position.y as i32)),
            camera: Camera::new(Point2::new(0, 0)),
        }
    }

    pub fn move_camera(
        &mut self,
        x: i32,
        y: i32,
    ) {
        self.camera.move_position(Point2::new(x, y));
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
        let area = (camera_pos - llr_size / 2).combine(llr_size);

        for (tile, tile_pos) in
            self.game.slice(area.x, area.y, area.width as u32, area.height as u32)
        {
            let pos =
                Point2::new(tile_pos.x as i32 - camera_pos.x, tile_pos.y as i32 - camera_pos.y);

            if 0 <= pos.x && 0 <= pos.y {
                let draw_pos = Point2::new(pos.x as u16, pos.y as u16);

                self.tile(llr, tile, draw_pos);
            }
        }

        let player_pos = Point2::new(
            self.game.player.position.x as i32 - camera_pos.x,
            self.game.player.position.y as i32 - camera_pos.y,
        );
        if 0 <= player_pos.x && 0 <= player_pos.y {
            let draw_pos = Point2::new(player_pos.x as u16, player_pos.y as u16);

            self.player(llr, draw_pos);
        }
    }

    fn player(
        &mut self,
        llr: &mut LLR,
        draw_pos: Point2<u16>,
    ) {
        let colour = self.theme.get_player();

        llr.pixel(colour, draw_pos);
    }

    fn tile(
        &mut self,
        llr: &mut LLR,
        tile: GameTile,
        draw_pos: Point2<u16>,
    ) {
        let colour = self.theme.get_game_tile(tile);

        llr.pixel(colour, draw_pos);
    }
}
