
use std::iter::Iterator;

pub struct Map<T> {
    pub width  : usize,
    pub height : usize,
        data   : MapData<T>,
}

type MapData<T> = Vec<T>;

impl<T: Copy> Map<T> {
    pub fn new( width : usize, height : usize, default : T ) -> Map<T> {
        return Map {
            width  : width,
            height : height,
            data   : new_map_data( width, height, default ),
        }
    }

    pub fn get( &self, x : usize, y : usize ) -> T {
        let index = map_index( x, y, self.width, self.height );

        return self.data[ index ];
    }

    pub fn set( & mut self, x : usize, y : usize, tile : T ) {
        let index = map_index( x, y, self.width, self.height );

        self.data[ index ] = tile;
    }

    pub fn map<F>( &mut self, mut map_f : F )
        where F : FnMut( T, usize, usize ) -> T
    {
        let mut i = 0;

        for y in 0 .. self.height {
            for x in 0 .. self.width {
                self.data[i] = map_f( self.data[i], x, y );

                i += 1;
            }
        }
    }

    pub fn transform<F, T2:Copy>( &self, map_f : F ) -> Map<T2>
        where F : FnMut( T ) -> T2
    {
        return Map {
            width  : self.width,
            height : self.height,
            data   : copy_map_data( & self.data, self.width, self.height, map_f ),
        }
    }

    pub fn slice_all( &self ) -> MapIterator<T> {
        return self.slice( 0, 0, self.width as u32, self.height as u32 );
    }

    pub fn slice( &self, mut x:i32, mut y:i32, w:u32, h:u32 ) -> MapIterator<T> {
        let width  = self.width  as i32;
        let height = self.height as i32;

        let mut sw = (w as i32) + x;
        let mut sh = (h as i32) + y;

        if x < 0 {
            x = 0;
        }

        if y < 0 {
            y = 0;
        }

        if sw < 0 {
            sw = 0;

        } else if sw > width {
            sw = width;

        }

        if sh < 0 {
            sh = 0;

        } else if sh > height {
            sh = height;

        }

        return MapIterator {
            data : & self.data,

            x : x as usize,
            y : y as usize,

            w : self.width,
            h : self.height,

            sx : x  as usize,
            sw : sw as usize,
            sh : sh as usize,
        }
    }
}

pub struct MapIterator<'a, T: 'a> {
    data : &'a MapData<T>,

    /// 
    /// The x position of where we are iterating.
    ///
    x : usize,

    /// 
    /// The y position of where we are iterating.
    ///
    y : usize,

    ///
    /// Full size of the data map.
    /// 
    w : usize,
    h : usize,

    /// 
    /// sx stands for 'slice x'.
    /// It's the x position of the top left corner of the map.
    /// 
    sx : usize,

    ///
    /// sw is the 'slice width'.
    /// 
    sw : usize,

    ///
    /// sh is the 'slice height'.
    /// 
    sh : usize,
}

impl<'a, T:Copy> Iterator for MapIterator<'a, T> {
    type Item = MapIteratorItem<T>;

    fn next( &mut self ) -> Option<MapIteratorItem<T>> {
        if self.y >= self.sh {
            return None;
        }

        let i = map_index( self.x, self.y, self.w, self.h );
        let result = Some(( self.data[i], self.x, self.y ));

        if self.x < self.sw {
            self.x += 1;

            if self.x == self.sw {
                self.x  = self.sx;
                self.y += 1;
            }

        } else {
            self.x  = self.sx;
            self.y += 1;

        }

        return result;
    }
}

type MapIteratorItem<T> =
    ( T, usize, usize );

fn new_map_data<T: Copy>( width : usize, height : usize, default : T ) -> MapData<T> {
    let size    = width * height;
    let mut map = Vec::with_capacity( size );

    for _ in 0 .. size {
        map.push( default );
    }

    return map;
}

fn copy_map_data<F, T1:Copy, T2:Copy>(
        src     : & MapData<T1>,
        width   : usize,
        height  : usize,
    mut map_f   : F,
) -> MapData<T2> 
    where F : FnMut( T1 ) -> T2
{
    let size    = width * height;
    let mut map = Vec::with_capacity( size );

    for i in 0 .. size {
        let old_tile = src[ i ];
        let new_tile = map_f( old_tile );

        map.push( new_tile );
    }

    return map;
}

#[inline]
fn map_index( x:usize, y:usize, _width:usize, height:usize ) -> usize {
    return y*height + x;
}

