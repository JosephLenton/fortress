
use vulkano;
use winit;

use winit::WindowBuilder;

use render::vulkano_win;
use render::vulkano_win::VkSurfaceBuild;
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

    let extensions = vulkano_win::required_extensions();
    let instance   = vulkano::instance::Instance::new(None, &extensions, None).expect("failed to create Vulkan instance");

    let events_loop = winit::EventsLoop::new();
    let window      = winit::WindowBuilder::new()
        .with_title( setup.title )
        .with_dimensions( setup.window_size.width, setup.window_size.height )
        .build_vk_surface( &events_loop, &instance )
        .unwrap();

    let mut gfx = GFX::new();

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event : win_event, .. } => {
                match win_event {
                    winit::WindowEvent::Closed => {
                        println!("close");
                        events_loop.interrupt();
                    }

                    winit::WindowEvent::Resized( width, height ) => {
                        println!("resize {} {}", width, height);
                        state.on_resize( width, height );
                    }

                    winit::WindowEvent::Refresh => {
                        render(
                            & state,
                            & rgame,
                            & mut gfx,
                        );
                    }

                    // 
                    // User Input
                    //

                    winit::WindowEvent::MouseWheel( mouse_move, _ ) => {
                        match mouse_move {
                            winit::MouseScrollDelta::LineDelta( _, y )  => rgame.on_mouse_scroll( y ),
                            winit::MouseScrollDelta::PixelDelta( _, y ) => rgame.on_mouse_scroll( y ),
                        };
                    }

                    ev => {}
                }
            }
        }
    })
}

fn render(
        state   : & WindowState,
        rgame   : & RenderGame,
        gfx     : & mut GFX,
) {
    rgame.render(
        state,
        gfx,
    )
}

