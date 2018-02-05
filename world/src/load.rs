use map::Map;
use tiles::Tile;
use util::shapes::Size;

use std::cmp;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::io::Result;

/// The ETB character is 'end of transmission'.
/// It's the end of the stdin to say there is
/// no more content.
/// 
const CHAR_ETB: char = 27 as char;

fn new_tile_map() -> BTreeMap<char, Tile> {
    let mut map = BTreeMap::new();

    store_tile(&mut map, Tile::Empty);

    store_tile(&mut map, Tile::Ground);
    store_tile(&mut map, Tile::Grass);
    store_tile(&mut map, Tile::GrassThick);

    store_tile(&mut map, Tile::Hill);

    store_tile(&mut map, Tile::Rocks);

    store_tile(&mut map, Tile::Water);
    store_tile(&mut map, Tile::Wall);
    store_tile(&mut map, Tile::TreeStump);

    store_tile(&mut map, Tile::Ice);

    map
}

/// Given a buffer, this will return you a map of tiles.
/// Every tile can be represented by a character.
/// This converts from character to tile.
pub fn read_to_map(read_in: &mut BufRead) -> Result<Map<Tile>> {
    let mut buf = Vec::new();
    for line in read_in.lines() {
        buf.push(line?);
    }

    let decode_map = new_tile_map();
    let size = get_vec_size(&buf);
    let mut map = Map::new(size.width, size.height, Tile::Empty);

    populate_map(&decode_map, &mut map, &buf);

    Ok(map)
}

fn store_tile(
    map: &mut BTreeMap<char, Tile>,
    tile: Tile,
) {
    map.insert(char::from(tile), tile);
}

fn char_to_tile(
    decode_map: &BTreeMap<char, Tile>,
    c: char,
) -> Tile {
    match decode_map.get(&c) {
        Some(tile) => *tile,
        None => panic!("Tile not found for char, {}, code {}", c, c as u32),
    }
}

fn get_vec_size(buf: &[String]) -> Size<u32> {
    let mut max_width = 0;
    let max_height = buf.len() as u32;

    for line in buf {
        let line_len = line.chars().count() as u32;

        max_width = cmp::max(max_width, line_len);
    }

    Size {
        width: max_width,
        height: max_height,
    }
}

fn populate_map(
    decode_map: &BTreeMap<char, Tile>,
    map: &mut Map<Tile>,
    buf: &[String],
) {
    let mut y = 0;

    for line in buf {
        let mut x = 0;

        for c in line.chars() {
            if c != CHAR_ETB {
                let tile = char_to_tile(decode_map, c);
                map.set(x, y, tile);

                x += 1;
            }
        }

        y += 1;
    }
}
