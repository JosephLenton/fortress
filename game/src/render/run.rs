
use sdl2;
use sdl2::pixels;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseWheelDirection;

use render::setup::Setup;
use render::window_state::WindowState;
use render::render_game::RenderGame;
use render::colour;
use render::gfx::GFX;

use game::game::Game;

pub fn run(
        setup : Setup,
        game  : Game,
) {
    let mut state = WindowState::new( & setup );
    let mut rgame = RenderGame::new( & setup, & game );

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let mut window = video_subsys.window( setup.title, setup.window_size.width, setup.window_size.height )
        .position_centered()
        .allow_highdpi()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump()
        .unwrap();

    render(
        & state,
        & rgame,
        & mut canvas,
    );

    'main: loop {
        for event in events.wait_iter() {
            match event {
                Event::Quit { .. } | Event::AppTerminating { .. } => {
                    break 'main;
                }

                Event::Window {win_event:WindowEvent::SizeChanged(width, height), ..} | Event::Window {win_event:WindowEvent::Resized(width, height), ..} => {
                    state.on_resize( width as u32, height as u32 );

                    render(
                        & state,
                        & rgame,
                        & mut canvas,
                    );
                }

                //
                // User Input
                //

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    // todo
                }

                Event::MouseButtonDown {x, y, mouse_btn, ..} => {
                    // todo
                }

                Event::MouseWheel {y, direction:MouseWheelDirection::Normal, ..} => {
                    rgame.on_mouse_scroll( y );
                }

                Event::MouseWheel {y, direction:MouseWheelDirection::Flipped, ..} => {
                    rgame.on_mouse_scroll( - y );
                }

                _ => {}
            }
        }
    }

    println!("goodbye!");
}

fn render(
        state   : & WindowState,
        rgame   : & RenderGame,
        gfx     : & mut GFX,
) {
    gfx.clear();

    rgame.render(
        state,
        gfx,
    );

    gfx.finished_drawing();
}

