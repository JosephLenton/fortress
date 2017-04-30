
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

        window_size : Size::new( 800, 600 ),
        tile_size   : Size::new(  24,  24 ),
    };

    render::game::run( setup, game );
}

