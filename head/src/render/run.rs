
use sdl2;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseWheelDirection;

use fortress::colour::Colour;
use render::setup::Setup;
use render::cursor::Cursor;
use render::render_game::RenderGame;
use render::gfx::GFX;

use game::model::Game;

pub fn run(
        setup : Setup,
        game  : Game,
) {
    let mut rgame = RenderGame::new( & setup, & game );
    let mut cursor = Cursor::new( setup.window_size.width / 2, setup.window_size.height / 2 );

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
        & rgame,
        & cursor,
        & mut canvas,
    );

    'main: loop {
        for event in events.wait_iter() {
            match event {
                Event::Quit { .. } | Event::AppTerminating { .. } => {
                    break 'main;
                }

                Event::Window {win_event:WindowEvent::SizeChanged(width, height), ..} | Event::Window {win_event:WindowEvent::Resized(width, height), ..} => {
                    rgame.on_resize( width as u32, height as u32 );
                }

                //
                // User Input
                //

                Event::KeyUp {keycode: Some(Keycode::Left), ..} => {
                    println!("left up");
                }

                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    println!("left down");
                    rgame.move_camera( -1,  0 );
                }

                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    rgame.move_camera(  1,  0 );
                }

                Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                    rgame.move_camera(  0, -1 );
                }

                Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                    rgame.move_camera(  0,  1 );
                }
                Event::MouseButtonDown {x, y, mouse_btn, ..} => {
                    // todo
                }

                Event::MouseMotion { x, y, ..} => {
                    cursor.xy( x as u32, y as u32 );
                }

                Event::MouseWheel {y, direction:MouseWheelDirection::Normal, ..} => {
                    rgame.on_mouse_scroll( y );
                }

                Event::MouseWheel {y, direction:MouseWheelDirection::Flipped, ..} => {
                    rgame.on_mouse_scroll( - y );
                }

                _ => {}
            }

            // Re-render everything after each event.
            render(
                & rgame,
                & cursor,
                & mut canvas,
            )
        }
    }

    println!("goodbye!");
}

fn render(
        rgame   : & RenderGame,
        cursor  : & Cursor,
        gfx     : & mut GFX,
) {
    gfx.clear();

    rgame.render(
        gfx,
    );

    render_cursor(
        rgame,
        cursor,
        gfx,
    );

    gfx.finished_drawing();
}

fn render_cursor(
        rgame   : & RenderGame,
        cursor  : & Cursor,
        gfx     : & mut GFX,
) {
    let screen_tile = rgame.translate_window_to_tile_xy( cursor.to_xy() );

    gfx.rectangle_outline( Colour::LightGreen, screen_tile );
}

