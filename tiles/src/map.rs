
pub struct Map<T> {
    pub width  : usize,
    pub height : usize,
        data   : MapData<T>,
}

type MapData<T> = Vec<Vec<T>>;

impl<T: Copy> Map<T> {
    pub fn new( width : usize, height : usize, default : T ) -> Map<T> {
        return Map {
            width  : width,
            height : height,
            data   : new_map_data( width, height, default )
        }
    }

    pub fn get( &self, x : usize, y : usize ) -> T {
        return self.data[y][x]
    }

    pub fn set( & mut self, x : usize, y : usize, tile : T ) {
        self.data[y][x] = tile
    }

    pub fn map<F>( &mut self, mut map_f : F )
        where F: FnMut( T, usize, usize ) -> T
    {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                self.data[y][x] = map_f( self.data[y][x], x, y );
            }
        }
    }

    pub fn each<F>( &self, mut each_f : F )
        where F: FnMut( T, usize, usize )
    {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                each_f( self.data[y][x], x, y );
            }
        }
    }
}

fn new_map_data<T: Copy>( width : usize, height : usize, default : T ) -> MapData<T> {
    let mut map = Vec::with_capacity( width );

    for _ in 0 .. height {
        let mut row = Vec::with_capacity( height );

        for _ in 0 .. width {
            row.push( default );
        }

        map.push( row );
    }

    return map;
}

