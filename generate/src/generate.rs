use world::tiles::Tile;
use world::map::Map;

use rand::StdRng;
use rand::SeedableRng;
use rand::Rng;

pub struct MapOptions {
    pub width: u32,
    pub height: u32,
    pub seed: Option<usize>,
}

pub fn new_map(options: MapOptions) -> Map<Tile> {
    let mut map = Map::new(options.width, options.height, Tile::Grass);
    let mut rng = new_map_rng(&options);

    add_ground_vegetation(&mut map, &mut rng);
    add_buildings(&mut map, &mut rng);

    return map;
}

fn new_map_rng(options: &MapOptions) -> StdRng {
    let mut rng = match StdRng::new() {
        Ok(rng) => rng,
        Err(e) => panic!("Could not initialise map rng, {}", e),
    };

    match options.seed {
        Some(seed) => {
            rng.reseed(&[seed]);
        }
        None => { /* do nothing */ }
    }

    return rng;
}

fn add_ground_vegetation(mut map: &mut Map<Tile>, mut rng: &mut StdRng) {
    map.map(|_, _, _| random_tile(&mut rng));
}

fn add_buildings(mut map: &mut Map<Tile>, mut rng: &mut StdRng) {
    // todo,
    // change this to some proper building building code
    for x in 20..30 {
        map.set(x, 15, Tile::Wall);
    }

    for y in 15..30 {
        map.set(20, y, Tile::Wall);
    }
}

fn random_tile(rng: &mut StdRng) -> Tile {
    let r = rng.gen_range(0, 20);

    return match r {
        0...3 => Tile::GrassThick,
        4 => Tile::TreeStump,
        6 => Tile::Rocks,
        _ => Tile::Grass,
    };
}
