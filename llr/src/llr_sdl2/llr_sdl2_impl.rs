
use LLR;
use LLROptions;

use sdl2;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseWheelDirection;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::pixels::Color;

use llr_sdl2::to_sdl2::*;

use util::colour::RGBA;
use util::shapes::Rect;

/// An SDL2 based LLR.
pub struct LLRSDL2Impl {
    options : LLROptions,

    canvas : WindowCanvas,

    pub events : EventPump,
}

impl LLRSDL2Impl {
    pub fn new( options : LLROptions ) -> LLRSDL2Impl {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(options.title, options.window_size.width, options.window_size.height)
            .position_centered()
            .allow_highdpi()
            .resizable()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let events = sdl_context.event_pump().unwrap();

        LLRSDL2Impl {
            options : options,
            canvas : canvas,
            events : events,
        }
    }
}

impl LLR for LLRSDL2Impl {
    fn clear(&mut self) {
        let black = Color::RGBA(0, 0, 0, 255);
        self.canvas.set_draw_color(black);
        self.canvas.clear();
    }

    fn rectangle(
        &mut self,
        colour: RGBA,
        rect: Rect<f32>,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(colour.to_sdl2());
        self.canvas.fill_rect(rect.to_sdl2())
    }

    fn rectangle_outline(
        &mut self,
        colour: RGBA,
        rect: Rect<f32>,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(colour.to_sdl2());
        self.canvas.draw_rect(rect.to_sdl2())
    }

    fn finished_drawing(&mut self) {
        self.canvas.present();
    }
}
