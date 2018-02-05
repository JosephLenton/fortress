use theme::Theme;

use llr::temp::Event;
use llr::temp::Keycode;
use llr::temp::MouseWheelDirection;
use llr::temp::WindowEvent;

use llr::LLR;
use llr::LLROptions;
use llr::LLRSDL2;
use render::cursor::Cursor;
use render::render_game::RenderGame;

use game::Game;

use util::shapes::Point2;

pub fn run<'a>(
    setup: LLROptions,
    game: &'a Game,
) {
    let theme = Theme::new();
    let mut llr = LLRSDL2::new(setup);
    let mut rgame = RenderGame::new(&game, &theme, setup.tile_size, setup.window_size);
    let mut cursor =
        Cursor::new(setup.window_size.width as f32 / 2.0, setup.window_size.height as f32 / 2.0);

    rgame.render(&mut llr);

    'main: loop {
        match llr.poll() {
            Event::Quit {
                ..
            }
            | Event::AppTerminating {
                ..
            } => {
                break 'main;
            },

            Event::Window {
                win_event: WindowEvent::SizeChanged(width, height),
                ..
            }
            | Event::Window {
                win_event: WindowEvent::Resized(width, height),
                ..
            } => {
                rgame.on_resize(width as u32, height as u32);
            },

            // User Input
            // 
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                rgame.move_camera(-1, 0);
            },

            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                rgame.move_camera(1, 0);
            },

            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                rgame.move_camera(0, -1);
            },

            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                rgame.move_camera(0, 1);
            },
            Event::MouseButtonDown {
                x,
                y,
                mouse_btn,
                ..
            } => {
                // do nothing
            },

            Event::MouseMotion {
                x,
                y,
                ..
            } => {
                cursor.xy(Point2::new(x as f32, y as f32));
            },

            Event::MouseWheel {
                y,
                direction: MouseWheelDirection::Normal,
                ..
            } => {
                // do nothing
            },

            Event::MouseWheel {
                y,
                direction: MouseWheelDirection::Flipped,
                ..
            } => {
                // do nothing
            },

            _ => {},
        }

        // Re-render everything after each event.
        rgame.render(&mut llr)
    }

    println!("goodbye!");
}
