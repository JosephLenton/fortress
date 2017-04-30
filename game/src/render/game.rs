
extern crate piston_window;

use self::piston_window::*;
use self::piston_window::G2d;

use util::point::Point;
use render::setup::Setup;
use render::state::State;
use game::game::Game;
use render::colour;

pub fn run(
        setup : Setup,
        game  : Game,
) {
    let     camera = Point::new( game.width / 2, game.height / 2 );
    let mut state  = State::new( & setup, camera );

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
                        println!("{} {}", width, height);
                        state.window_size.size( width, height );
                    }

                    Input::Move(motion) => {
                        match motion {
                            Motion::MouseCursor(x, y) => {
                                state.cursor.set( x as u32, y as u32 );
                            }

                            Motion::MouseScroll(x, y) => {
                                if y > 0.0 {
                                    state.zoom.zoom_in();
                                } else if y < 0.0 {
                                    state.zoom.zoom_out();
                                }
                            }

                            _ => {
                                /* do nothing */
                            }
                        }
                    }

                    Input::Focus(_)
                  | Input::Cursor(_) => {
                        // todo, setup input handling here
                    }

                    Input::Render(_) => {
                        // todo, this is rendering constantly. Active rendering.
                        // We don't want active rendering. We only want to render on input.
                        window.draw_2d( &ev, |context, mut g2d| {
                            render(
                                & setup,
                                & state,
                                & game,
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
        setup   : & Setup,
        state   : & State,
        game    : & Game,
        context : & Context,
        g2d     : & mut G2d,
) {
    clear( colour::BLACK, g2d );

    render_tiles(
            setup,
            state,
            game,
            context,
            g2d,
    );
}

fn render_tiles(
        setup   : & Setup,
        state   : & State,
        game    : & Game,
        context : & Context,
        g2d     : & mut G2d,
) {
    let zoom          = state.zoom.zoom;
    let window_width  = state.window_size.width  as f32;
    let window_height = state.window_size.height as f32;
    let tile_width    = setup.tile_size.width  as f64 * zoom;
    let tile_height   = setup.tile_size.height as f64 * zoom;

    // Work out the area that we are rendering.
    // We want to skip areas outside of the window.
    let num_game_tiles_x =   window_width  as f32 / tile_width  as f32 ;
    let num_game_tiles_y =   window_height as f32 / tile_height as f32 ;
    let top_left_x       = ( state.camera.x as f32 - num_game_tiles_x/2.0 ).floor() as i32;
    let top_left_y       = ( state.camera.y as f32 - num_game_tiles_y/2.0 ).floor() as i32;
    let bottom_right_x   = ( state.camera.x as f32 + num_game_tiles_x/2.0 ).ceil() as i32;
    let bottom_right_y   = ( state.camera.y as f32 + num_game_tiles_y/2.0 ).ceil() as i32;
    let game_width       = ( bottom_right_x - top_left_x ) as u32;
    let game_height      = ( bottom_right_y - top_left_y ) as u32;

    for ( tile, x, y ) in game.slice( top_left_x, top_left_y, game_width, game_height ) {
        let draw_x = (window_width  as f64)/2.0 - ( (state.camera.x as f64 - x as f64) as f64 )*tile_width;
        let draw_y = (window_height as f64)/2.0 - ( (state.camera.y as f64 - y as f64) as f64 )*tile_height;
        let ( background, foreground ) = colour::tile_to_colour( tile.tile );

        rectangle(
                background,
                [ draw_x, draw_y, tile_width, tile_height ],
                context.transform,
                g2d,
        );

        rectangle(
                foreground,
                [ draw_x+tile_width/4.0, draw_y+tile_height/4.0, tile_width/2.0, tile_height/2.0 ],
                context.transform,
                g2d,
        );
    }
}

