
use tile::Tile;
use tile::display_tile::tile_to_char;
use map::Map;

use std::cmp;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::io::Result;
use std::vec::Vec;

///
/// The ETB character is 'end of transmission'.
/// It's the end of the stdin to say there is
/// no more content.
///
const CHAR_ETB : char = 27 as char;

fn new_tile_map() -> BTreeMap<char, Tile> {
    let mut map = BTreeMap::new() as BTreeMap<char, Tile>;

    store_tile( &mut map, Tile::Empty        );

    store_tile( &mut map, Tile::Ground       );
    store_tile( &mut map, Tile::Grass        );
    store_tile( &mut map, Tile::GrassThick   );

    store_tile( &mut map, Tile::Hill         );

    store_tile( &mut map, Tile::Rocks        );

    store_tile( &mut map, Tile::Water        );
    store_tile( &mut map, Tile::Wall         );
    store_tile( &mut map, Tile::TreeStump    );

    store_tile( &mut map, Tile::Ice          );

    return map;
}

fn store_tile( map : &mut BTreeMap<char, Tile>, tile : Tile ) {
    let tile_char = tile_to_char( tile );

    map.insert( tile_char, tile );
}

fn char_to_tile( decode_map : & BTreeMap<char, Tile>, c : char ) -> Tile {
    return match decode_map.get( & c ) {
        Some( tile ) => *tile,
        None         => panic!( "Tile not found for char, {}, code {}", c, c as u32 ),
    };
}

pub fn read_to_map( read_in : &mut BufRead ) -> Result<Map<Tile>> {
    let mut buf = Vec::new();
    for line in read_in.lines() {
        buf.push( line? );
    }

    let decode_map      = new_tile_map();
    let (width, height) = get_vec_size( & buf );
    let mut map         = Map::new( width, height, Tile::Empty );

    populate_map( & decode_map, &mut map, & buf );

    return Ok(map);
}

fn get_vec_size( buf : & Vec<String> ) -> ( u32, u32 ) {
    let mut max_width  = 0 as u32;
    let     max_height = buf.len() as u32;

    for line in buf {
        let line_len = line.chars().count() as u32;

        max_width = cmp::max( max_width, line_len );
    }

    return ( max_width, max_height );
}

fn populate_map( decode_map : & BTreeMap<char, Tile>, map : &mut Map<Tile>, buf : & Vec<String> ) {
    let mut y = 0;

    for line in buf {
        let mut x = 0;

        for c in line.chars() {
            if c != CHAR_ETB {
                let tile = char_to_tile( decode_map, c );
                map.set( x, y, tile );

                x += 1;
            }
        }

        y += 1;
    }
}

