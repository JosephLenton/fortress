use LLR;
use LLROptions;
use LLRPixel;

use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use super::to_sdl2::*;

use util::shapes::Point2;
use util::shapes::Rect;
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
        let window_size = Size<u32>::from( options.window_size )
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
        pos: Point2<i32>,
    ) -> Result<(), String> {
        pos + ( self.options.tile_size / 2.0 )

        self.canvas.set_draw_color(pixel.background.to_sdl2());
        self.canvas.fill_rect(rect.to_sdl2());
        self.canvas.set_draw_color(pixel.foreground.to_sdl2());
        self.canvas.fill_rect(rect.divide_around_centre(2.0).to_sdl2())
    }

    fn finished_drawing(&mut self) {
        self.canvas.present();
    }

    fn size(&mut self) -> Size<u32> {
        let surface = self.canvas.surface();

        Size::new(
            surface.width() / self.options.tile_size.width,
            surface.height() / self.options.tile_size.height,
        )
    }
}
