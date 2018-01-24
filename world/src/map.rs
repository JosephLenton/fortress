use std::iter::Iterator;

pub struct Map<T> {
    pub width: u32,
    pub height: u32,
    data: MapData<T>,
}

type MapData<T> = Vec<T>;

impl<T: Copy> Map<T> {
    pub fn new(width: u32, height: u32, default: T) -> Map<T> {
        return Map {
            width: width,
            height: height,
            data: new_map_data(width, height, default),
        };
    }

    pub fn get(&self, x: u32, y: u32) -> T {
        let index = map_index(x, y, self.width, self.height);

        return self.data[index];
    }

    pub fn set(&mut self, x: u32, y: u32, tile: T) {
        let index = map_index(x, y, self.width, self.height);

        self.data[index] = tile;
    }

    pub fn map<F>(&mut self, mut map_f: F)
    where
        F: FnMut(T, u32, u32) -> T,
    {
        let mut i = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                self.data[i] = map_f(self.data[i], x, y);

                i += 1;
            }
        }
    }

    pub fn transform<F, T2: Copy>(&self, map_f: F) -> Map<T2>
    where
        F: FnMut(T) -> T2,
    {
        return Map {
            width: self.width,
            height: self.height,
            data: copy_map_data(&self.data, self.width, self.height, map_f),
        };
    }

    pub fn slice_all(&self) -> MapIterator<T> {
        return self.slice(0, 0, self.width, self.height);
    }

    pub fn slice(&self, mut x: i32, mut y: i32, w: u32, h: u32) -> MapIterator<T> {
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

        return MapIterator {
            data: &self.data,

            x: x as u32,
            y: y as u32,

            w: self.width,
            h: self.height,

            sx: x as u32,
            sw: sw as u32,
            sh: sh as u32,
        };
    }
}

pub struct MapIterator<'a, T: 'a> {
    data: &'a MapData<T>,

    ///
    /// The x position of where we are iterating.
    ///
    x: u32,

    ///
    /// The y position of where we are iterating.
    ///
    y: u32,

    ///
    /// Full size of the data map.
    ///
    w: u32,
    h: u32,

    ///
    /// sx stands for 'slice x'.
    /// It's the x position of the top left corner of the map.
    ///
    sx: u32,

    ///
    /// sw is the 'slice width'.
    ///
    sw: u32,

    ///
    /// sh is the 'slice height'.
    ///
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

        return result;
    }
}

type MapIteratorItem<T> = (T, u32, u32);

fn new_map_data<T: Copy>(width: u32, height: u32, default: T) -> MapData<T> {
    let size = (width * height) as usize;
    let mut map = Vec::with_capacity(size);

    for _ in 0..size {
        map.push(default);
    }

    return map;
}

fn copy_map_data<F, T1: Copy, T2: Copy>(
    src: &MapData<T1>,
    width: u32,
    height: u32,
    mut map_f: F,
) -> MapData<T2>
where
    F: FnMut(T1) -> T2,
{
    let size = (width * height) as usize;
    let mut map = Vec::with_capacity(size);

    for i in 0..size {
        let old_tile = src[i];
        let new_tile = map_f(old_tile);

        map.push(new_tile);
    }

    return map;
}

#[inline]
fn map_index(x: u32, y: u32, _width: u32, height: u32) -> usize {
    return (y * height + x) as usize;
}
