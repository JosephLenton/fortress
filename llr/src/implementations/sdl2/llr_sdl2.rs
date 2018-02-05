use LLR;
use LLRPixel;
use LLROptions;

use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use super::to_sdl2::*;

use util::shapes::Rect;

/// An SDL2 based LLR.
pub struct LLRSDL2 {
    options: LLROptions,

    canvas: WindowCanvas,

    events: EventPump,
}

impl LLRSDL2 {
    /// Creates a new SDL2 based LLR.
    ///
    /// A window will appear for the user as a result of calling this.
    pub fn new(options: LLROptions) -> LLRSDL2 {
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

        LLRSDL2 {
            options: options,
            canvas: canvas,
            events: events,
        }
    }

    /// Blocks indefinitely until a user event has occurred.
    /// When the event happens this will return.
    pub fn poll(&mut self) -> Event {
        self.events.wait_event()
    }
}

impl LLR for LLRSDL2 {
    fn clear(&mut self) {
        let black = Color::RGBA(0, 0, 0, 255);
        self.canvas.set_draw_color(black);
        self.canvas.clear();
    }

    fn pixel(
        &mut self,
        pixel: LLRPixel,
        rect: Rect<f32>,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(pixel.background.to_sdl2());
        self.canvas.fill_rect(rect.to_sdl2());
        self.canvas.set_draw_color(pixel.foreground.to_sdl2());
        self.canvas.fill_rect(rect.divide_around_centre(2.0).to_sdl2())
    }

    fn finished_drawing(&mut self) {
        self.canvas.present();
    }
}
