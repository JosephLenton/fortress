use LLR;
use LLROptions;
use LLRPixel;

use std;

use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use super::to_sdl2::*;

use util::shapes::Point2;
use util::shapes::Size;

/// An SDL2 based LLR.
pub struct LLRSDL2 {
    options: LLROptions,

    tile_size: Size<f32>,

    canvas: WindowCanvas,

    events: EventPump,
}

impl LLRSDL2 {
    /// Creates a new SDL2 based LLR.
    ///
    /// A window will appear for the user as a result of calling this.
    pub fn new(options: LLROptions) -> LLRSDL2 {
        let window_size = options.window_size.to::<u32>();
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(options.title, window_size.width, window_size.height)
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
            tile_size: options.tile_size.to::<f32>(),
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
        pos: Point2<u16>,
    ) -> Result<(), String> {
        let tile_size = self.options.tile_size.to::<f32>();
        let draw_pos = pos.to::<f32>() * tile_size;
        let outer = draw_pos.combine(tile_size);
        let inner = (draw_pos + tile_size / 4.0).combine(tile_size / 2.0);

        self.canvas.set_draw_color(pixel.background.to_sdl2());
        self.canvas.fill_rect(outer.to_sdl2());
        self.canvas.set_draw_color(pixel.foreground.to_sdl2());
        self.canvas.fill_rect(inner.to_sdl2())
    }

    fn finished_drawing(&mut self) {
        self.canvas.present();
    }

    fn size(&self) -> Size<u16> {
        let (s_width, s_height) = self.canvas.window().size();

        let num_x =
            (s_width / self.options.tile_size.width as u32).max(std::u16::MAX as u32) as u16;
        let num_y =
            (s_height / self.options.tile_size.height as u32).max(std::u16::MAX as u32) as u16;

        Size::new(num_x, num_y)
    }
}
