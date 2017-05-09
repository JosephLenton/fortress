
extern crate piston_window;

use self::piston_window::*;
use self::piston_window::Context;
use self::piston_window::G2d;

use render::setup::Setup;
use render::window_state::WindowState;
use render::render_game::RenderGame;
use game::game::Game;
use render::colour;

pub fn run(
        setup : Setup,
        game  : Game,
) {
    let mut state  = WindowState::new( & setup );
    let mut rgame  = RenderGame::new( & setup, & game );

    let mut window : PistonWindow = WindowSettings::new( setup.title, [ setup.window_size.width, setup.window_size.height ] )
        .build()
        .unwrap();

    'render: loop {
        match window.next() {
            None | Some(Input::Close(_)) => {
                println!("close");
                break 'render;
            }

            Some(ev) => {
                match ev {
                    Input::Resize( width, height ) => {
                        println!("resize {} {}", width, height);
                        state.on_resize( width, height );
                    }

                    Input::Move(motion) => {
                        match motion {
                            Motion::MouseCursor(x, y) => {
                                // todo, handle cursor handling
                            }

                            Motion::MouseScroll(x, y) => {
                                rgame.on_mouse_scroll( y );
                            }

                            _ => {
                                /* do nothing */
                            }
                        }
                    }

                    Input::Render(_) => {
                        // todo, this is rendering constantly. Active rendering.
                        // We don't want active rendering. We only want to render on input.
                        window.draw_2d( &ev, |context, mut g2d| {
                            render(
                                & state,
                                & rgame,
                                & context,
                                & mut g2d,
                            );
                        });
                    }

                    _ => { /* do nothing */ }
                }
            }
        }
    }
}

fn render(
        state   : & WindowState,
        rgame   : & RenderGame,
        context : & Context,
        g2d     : & mut G2d,
) {
    clear( colour::BLACK, g2d );

    rgame.render(
        & state,
        & context,
        g2d,
    )
}

