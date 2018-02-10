use LLR;
use LLROptions;
use LLRPixel;
use LLREvent;
use LLRKey;

use std;

use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use super::to_sdl2::*;

use util::shapes::Point;
use util::shapes::Size;

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
            canvas: canvas,
            events: events,
        }
    }
}

impl LLR for LLRSDL2 {
    fn clear(&mut self) {
        self.canvas.set_draw_color( self.options.clear_colour.to_sdl2() );
        self.canvas.clear();
    }

    fn pixel(
        &mut self,
        pixel: LLRPixel,
        pos: Point<u16>,
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

    /// Blocks indefinitely until a user event has occurred.
    /// When the event happens this will return.
    fn poll(&mut self) -> Option<LLREvent> {
        match self.events.wait_event() {
            Event::Quit { .. } | Event::AppTerminating { .. } => {
                Some(LLREvent::Quit)
            },

            Event::Window { win_event, .. } => {
                match win_event {
                    WindowEvent::Resized(_w, _h) | WindowEvent::SizeChanged(_w, _h) => {
                        Some(LLREvent::Resize)
                    },
                    _ => {
                        None
                    },
                }
            },

            Event::KeyDown {
                keycode: Some(sdl_key),
                ..
            } => {
                match translate_sdl_key( sdl_key ) {
                    Some( key ) => {
                        Some( LLREvent::KeyPress( key ))
                    },
                    None => {
                        None
                    },
                }
            },

            _ => {
                None
            },
        }
    }
}

fn translate_sdl_key( sdl_key : Keycode ) -> Option<LLRKey> {
    match sdl_key {
        Keycode::Up => {
            Some(LLRKey::Up)
        },
        Keycode::Down => {
            Some(LLRKey::Down)
        },
        Keycode::Left => {
            Some(LLRKey::Left)
        },
        Keycode::Right => {
            Some(LLRKey::Right)
        },
        _ => {
            None
        },
    }
}

