use theme::Theme;

use llr::LLREvent;
use llr::LLRKey;
use llr::LLROptions;
use llr::LLRSDL2;
use llr::LLR;

use render::cursor::Cursor;
use render::render_game::RenderGame;

use game::Game;

use util::shapes::Point;

pub fn run<'a>(
    setup: LLROptions,
    game: &'a Game,
) {
    let theme = Theme::new();
    let mut llr = LLRSDL2::new(setup);
    let mut rgame = RenderGame::new(&game, &theme);
    let mut cursor =
        Cursor::new(setup.window_size.width as f32 / 2.0, setup.window_size.height as f32 / 2.0);

    rgame.render(&mut llr);

    'main: loop {
        match llr.poll() {
            Some(ev) => {
                match ev {
                    LLREvent::Quit => {
                        break 'main;
                    },

                    // User Input
                    //
                    LLREvent::KeyPress(LLRKey::Left) => {
                        rgame.move_camera(-1, 0);
                    },

                    LLREvent::KeyPress(LLRKey::Right) => {
                        rgame.move_camera(1, 0);
                    },

                    LLREvent::KeyPress(LLRKey::Up) => {
                        rgame.move_camera(0, -1);
                    },

                    LLREvent::KeyPress(LLRKey::Down) => {
                        rgame.move_camera(0, 1);
                    },

                    // do nothing
                    _ => {},
                }
            },

            _ => {},
        }

        // Re-render everything after each event.
        rgame.render(&mut llr)
    }

    // Keep this.
    // It's useful to see a message to tell us we really did quit.
    println!("goodbye!");
}
