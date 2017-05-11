
#[macro_use]
extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

extern crate tiles;

mod game;
mod render;
mod util;

use tiles::load;
use std::io;
use game::game::Game;
use render::setup::Setup;
use util::size::Size;

fn main() {
    let std_in  = io::stdin();
    let std_out = io::stdout();

    let map   = load::read_to_map( &mut std_in.lock() );
    let game  = Game::new( map );
    let setup = Setup {
        title : "Fortress",

        window_size : Size { width: 800, height: 600 },
        tile_size   : Size { width:  24, height:  24 },
    };

    render::main::run( setup, game );
}

