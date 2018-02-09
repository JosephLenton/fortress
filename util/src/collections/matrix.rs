use std::iter::Iterator;
use shapes::Point;
use shapes::Size;
use shapes::Rect;

/// Holds the data for a Matrix.
///
/// A Matrix has a fixed size and cannot be resized.
pub struct Matrix<T: Copy> {
    /// The width and height of this matrix.
    size : Size<u16>,

    /// The raw data inside.
    /// It's size matches the size of this matrix.
    data: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of the size given.
    ///
    /// It is filled with the default value.
    pub fn new(
        size : Size<u16>,
        default: T,
    ) -> Self {
        Matrix {
            size : size,
            data: vec![default; size.to_clamped::<usize>().area()],
        }
    }

    /// Returns the tile at the position given.
    pub fn get(
        &self,
        pos : Point<u16>,
    ) -> T {
        let index = map_index(pos, self.size);

        self.data[index]
    }

    /// Sets a tile at the position given.
    pub fn set(
        &mut self,
        pos : Point<u16>,
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
        self.iter_of(Point::new(0, 0).combine(self.size))
    }

    /// Allows you to iterate over a sub section of this map.
    pub fn iter_of(
        &self,
        area : Rect<u16>,
    ) -> MatrixIterator<T> {
        let data_rect = Point::new(0, 0).combine(self.size);
        let iterate_area = area.clamp_within( data_rect );

        MatrixIterator {
            data: &self.data,

            iterate_area: iterate_area,
            pos: area.point(),
        }
    }
}

/// An iterator for the `Matrix`.
pub struct MatrixIterator<'a, T: 'a> {
    /// The raw data we are iterating over.
    data: &'a Vec<T>,

    /// The size of the area we are iterating over.
    iterate_area: Rect<u16>,

    /// Current index in the `Matrix`.
    pos: Point<u16>,
}

impl<'a, T: Copy> Iterator for MatrixIterator<'a, T> {
    type Item = MapIteratorItem<T>;

    fn next(&mut self) -> Option<MapIteratorItem<T>> {
        if self.pos.y >= self.iterate_area.height {
            return None;
        }

        let i = map_index(self.pos, self.iterate_area.size());
        let data = self.data[i];

        let result = Some((data, self.pos));

        // Increment across the x axis.
        if self.pos.x < self.iterate_area.width-1 {
            self.pos.move_x( 1 );

        // We've wrapped over the X position.
        } else {
            self.pos.set_x( self.iterate_area.x );
            self.pos.move_y( 1 );
        }

        result
    }
}

type MapIteratorItem<T> = (T, Point<u16>);

fn map_index(
    pos : Point<u16>,
    size : Size<u16>,
) -> usize {
    (pos.y * size.height + pos.x) as usize
}
