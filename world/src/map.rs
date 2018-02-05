use std::iter::Iterator;

/// Holds the data for a Map.
pub struct Map<T: Copy> {
    /// The width of the map.
    pub width: u32,

    /// The hgitht of the map.
    pub height: u32,

    /// The raw data inside of the map.
    data: Vec<T>,
}

impl<T: Copy> Map<T> {
    /// Creates a new map with the width and height given.
    /// This map is filled with the default value.
    pub fn new(
        width: u32,
        height: u32,
        default: T,
    ) -> Map<T> {
        Map {
            width: width,
            height: height,
            data: vec![default; (width * height) as usize],
        }
    }

    /// Returns the tile at the position given.
    pub fn get(
        &self,
        x: u32,
        y: u32,
    ) -> T {
        let index = map_index(x, y, self.width, self.height);

        self.data[index]
    }

    /// Sets a tile at the position given.
    pub fn set(
        &mut self,
        x: u32,
        y: u32,
        tile: T,
    ) -> () {
        let index = map_index(x, y, self.width, self.height);

        self.data[index] = tile;
    }

    /// Maps this data against the function given.
    /// This permanently changes the contents of this map.
    pub fn fill<F>(
        &mut self,
        mut map_f: F,
    ) where
        F: FnMut(&T, u32, u32) -> T,
    {
        let mut i = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                self.data[i] = map_f(&self.data[i], x, y);

                i += 1;
            }
        }
    }

    /// Maps the data in this map against the function given.
    /// However the result is returned in a new map.
    pub fn map<F, T2: Copy>(
        &self,
        map_f: F,
    ) -> Map<T2>
    where
        F: FnMut(&T) -> T2,
    {
        Map {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(map_f).collect(),
        }
    }

    /// Returns a slice which encompasses the entire map.
    pub fn slice_all(&self) -> MapIterator<T> {
        self.slice(0, 0, self.width, self.height)
    }

    /// Returns a slice of this map.
    pub fn slice(
        &self,
        mut x: i32,
        mut y: i32,
        w: u32,
        h: u32,
    ) -> MapIterator<T> {
        let width = self.width as i32;
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

        MapIterator {
            data: &self.data,

            x: x as u32,
            y: y as u32,

            w: self.width,
            h: self.height,

            sx: x as u32,
            sw: sw as u32,
            sh: sh as u32,
        }
    }
}

/// An iterator for the `Map`.
pub struct MapIterator<'a, T: 'a> {
    /// The raw data we are iterating over.
    data: &'a Vec<T>,

    /// The x position of where we are iterating.
    x: u32,

    /// The y position of where we are iterating.
    y: u32,

    /// Full size of the data map.
    w: u32,
    h: u32,

    /// sx stands for 'slice x'.
    /// It's the x position of the top left corner of the map.
    sx: u32,

    /// sw is the 'slice width'.
    sw: u32,

    /// sh is the 'slice height'.
    sh: u32,
}

impl<'a, T: Copy> Iterator for MapIterator<'a, T> {
    type Item = MapIteratorItem<T>;

    fn next(&mut self) -> Option<MapIteratorItem<T>> {
        if self.y >= self.sh {
            return None;
        }

        let i = map_index(self.x, self.y, self.w, self.h);
        let result = Some((self.data[i], self.x, self.y));

        if self.x < self.sw {
            self.x += 1;

            if self.x == self.sw {
                self.x = self.sx;
                self.y += 1;
            }
        } else {
            self.x = self.sx;
            self.y += 1;
        }

        result
    }
}

type MapIteratorItem<T> = (T, u32, u32);

fn map_index(
    x: u32,
    y: u32,
    _width: u32,
    height: u32,
) -> usize {
    (y * height + x) as usize
}
