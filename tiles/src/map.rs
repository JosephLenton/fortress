
use tile::Tile;

pub struct Map {
    pub width  : usize,
    pub height : usize,
        data   : MapData,
}

type MapData = Vec<Vec<Tile>>;

impl Map {
    pub fn new( width : usize, height : usize, default_tile : Tile ) -> Map {
        return Map {
            width  : width,
            height : height,
            data   : new_map_data( width, height, default_tile )
        }
    }

    pub fn get( &self, x : usize, y : usize ) -> Tile {
        return self.data[y][x]
    }

    pub fn set( & mut self, x : usize, y : usize, tile : Tile ) {
        self.data[y][x] = tile
    }

    pub fn map<F>( &mut self, mut map_f : F )
        where F: FnMut( Tile, usize, usize ) -> Tile
    {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                self.data[y][x] = map_f( self.data[y][x], x, y );
            }
        }
    }

    pub fn each<F>( &self, mut each_f : F )
        where F: FnMut( Tile, usize, usize )
    {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                each_f( self.data[y][x], x, y );
            }
        }
    }
}

fn new_map_data( width : usize, height : usize, default_tile : Tile ) -> MapData {
    let mut map = Vec::with_capacity( width );

    for _ in 0 .. height {
        let mut row = Vec::with_capacity( height );

        for _ in 0 .. width {
            row.push( default_tile );
        }

        map.push( row );
    }

    return map;
}

