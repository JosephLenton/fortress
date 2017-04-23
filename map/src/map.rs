
extern crate tiles;
extern crate rand;

use tiles::tiles::Tile;

use map::rand::{SeedableRng, StdRng, Rng};
use std::vec::Vec;

pub type Map = Vec<Vec<Tile>>;

pub struct MapOptions {
    pub width  : usize,
    pub height : usize,
    pub seed   : Option<usize>
}

pub fn new_map( options : MapOptions ) -> Map {
    let mut map = new_filled_map( &options, tiles::tiles::GROUND );
    let mut rng = new_map_rng( &options );

    for x in 0 .. options.width {
        for y in 0 .. options.height {
            map[x][y] = random_tile( &mut rng );
        }
    }

    return map;
}

fn new_filled_map( options : &MapOptions, tile : Tile ) -> Map {
    let mut map = Vec::with_capacity( options.width );

    for x in 0 .. options.width {
        let mut row = Vec::with_capacity( options.height );

        for y in 0 .. options.height {
            row.push( tile );
        }

        map.push( row );
    }

    return map;
}

fn new_map_rng( options : &MapOptions ) -> StdRng {
    let mut rng = match StdRng::new() {
        Ok(rng) => rng,
        Err(e)  => panic!("Could not initialise map rng, {}", e)
    };

    match options.seed {
        Some(seed) => { rng.reseed(&[ seed ]); }
        None       => {    /* do nothing */    }
    }

    return rng;
}

fn random_tile( rng : &mut rand::StdRng ) -> Tile {
    let r = rng.gen_range( 0, 10 );

    return match r {
        0 ... 3 => tiles::tiles::GRASS,
        6       => tiles::tiles::ROCKS,
              _ => tiles::tiles::GROUND
    }
}

