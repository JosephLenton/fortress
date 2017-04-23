
extern crate tiles;
extern crate rand;

use tiles::tile::Tile;
use tiles::map::Map;

use map::rand::{SeedableRng, StdRng, Rng};

pub struct MapOptions {
    pub width  : usize,
    pub height : usize,
    pub seed   : Option<usize>
}

pub fn new_map( options : MapOptions ) -> Map {
    let mut map = tiles::map::Map::new( options.width, options.height, tiles::tile::GROUND );
    let mut rng = new_map_rng( &options );

    add_base_vegetation( & mut map, & mut rng );
    add_buildings( & mut map, & mut rng );

    return map;
}

fn add_base_vegetation(
    mut map : & mut Map,
    mut rng : & mut StdRng,
) {
    map.map(|_, _, _| random_tile( &mut rng ));
}

fn add_buildings(
    mut map : & mut Map,
    mut rng : & mut StdRng,
) {
    for x in 20 .. 30 {
        map.set(  x, 15, tiles::tile::WALL );
    }

    for y in 15 .. 30 {
        map.set( 20,  y, tiles::tile::WALL );
    }
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

fn random_tile( rng : &mut StdRng ) -> Tile {
    let r = rng.gen_range( 0, 10 );

    return match r {
        0 ... 3 => tiles::tile::GRASS,
        6       => tiles::tile::ROCKS,
              _ => tiles::tile::GROUND
    }
}

