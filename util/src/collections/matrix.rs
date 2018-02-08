use std::iter::Iterator;
use util::shapes::Point2;

/// Holds the data for a Matrix.
///
/// A Matrix has a fixed size and cannot be resized.
pub struct Matrix<T: Copy> {
    /// The width and height of this matrix.
    size : Size<u32>,

    /// The raw data inside.
    /// It's size matches the size of this matrix.
    data: Vec<T>,
}

impl<T: Copy> Map<T> {
    /// Creates a new matrix of the size given.
    ///
    /// It is filled with the default value.
    pub fn new(
        size : Size<u16>,
        default: T,
    ) -> Map<T> {
        Map {
            size : size,
            data: vec![default; size::to_clamped::<usize>().area()],
        }
    }

    /// Returns the tile at the position given.
    pub fn get(
        &self,
        pos : Point2<u16>,
    ) -> T {
        let index = map_index(pos, self.size);

        self.data[index]
    }

    /// Sets a tile at the position given.
    pub fn set(
        &mut self,
        pos : Point2<u16>,
        tile: T,
    ) -> () {
        let index = map_index(pos, self.size);

        self.data[index] = tile;
    }

    /// Returns the size of this Matrix.
    pub fn size(&self) -> Size<u16> {
        self.size
    }

    /// Returns a slice which encompasses the entire map.
    pub fn iter(&self) -> MatrixIterator<T> {
        self.iter_of(Point2::new(0, 0), self.size)
    }

    /// Allows you to iterate over a sub section of this map.
    pub fn iter_of(
        &self,
        area : Rect<u16>,
        pos : Point2<u16>,
        size : Size<u16>,
    ) -> MatrixIterator<T> {
        let data_rect = Point2::new(0, 0).to_rect(self.size);
        let iterate_area = area.clamp_within( data_rect )

        MatrixIterator {
            data: &self.data,
            data_size: self.size,

            iterate_area: iterate_area,
        }
    }
}

/// An iterator for the `Matrix`.
pub struct MatrixIterator<'a, T: 'a> {
    /// The raw data we are iterating over.
    data: &'a Vec<T>,

    /// The size of the raw data area.
    data_size: Size<u16>,

    /// The size of the area we are iterating over.
    iterate_area: Rect<u16>,
}

impl<'a, T: Copy> Iterator for MatrixIterator<'a, T> {
    type Item = MapIteratorItem<T>;

    fn next(&mut self) -> Option<MapIteratorItem<T>> {
        if self.y >= self.sh {
            return None;
        }

        let i = map_index(self.x, self.y, self.w, self.h);
        let data = self.data[i];
        let pos = Point2::new(self.x, self.y);

        let result = Some((data, pos));

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

type MapIteratorItem<T> = (T, Point2<u32>);

fn map_index(
    pos : Point2<u16>,
    size : Size<u16>,
) -> usize {
    (pos.y * size.height + pos.x) as usize
}
