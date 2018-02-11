use shapes::Point;
use shapes::Rect;
use shapes::Size;
use std::iter::Iterator;
use std::ops::Index;
use std::ops::IndexMut;

/// Holds the data for a Matrix.
///
/// A Matrix has a fixed size and cannot be resized.
pub struct Matrix<V: Copy> {
    /// The width and height of this matrix.
    size: Size<u16>,

    /// The raw data inside.
    /// It's size matches the size of this matrix.
    data: Vec<V>,
}

impl<V: Copy> Matrix<V> {
    /// Creates a new matrix of the size given.
    ///
    /// It is filled with the default value.
    pub fn new(
        size: Size<u16>,
        default: V,
    ) -> Self {
        Matrix {
            size: size,
            data: vec![default; size.to_clamped::<usize>().area()],
        }
    }

    /// Returns the tile at the position given.
    pub fn get(
        &self,
        pos: Point<u16>,
    ) -> V {
        let index = map_index(pos, self.size);

        self.data[index]
    }

    /// Sets a tile at the position given.
    pub fn set(
        &mut self,
        pos: Point<u16>,
        value: V,
    ) -> () {
        let index = map_index(pos, self.size);

        self.data[index] = value;
    }

    /// Returns the size of this Matrix.
    pub fn size(&self) -> Size<u16> {
        self.size
    }

    /// Returns a slice which encompasses the entire map.
    pub fn iter(&self) -> MatrixIterator<V> {
        self.iter_of(Point::new(0, 0).combine(self.size))
    }

    /// Allows you to iterate over a sub section of this map.
    pub fn iter_of(
        &self,
        area: Rect<u16>,
    ) -> MatrixIterator<V> {
        let data_rect = Point::new(0, 0).combine(self.size);

        let iterate_area = match area.overlap_of(data_rect) {
            Some(iterate_area) => iterate_area,
            None => Rect::new(0, 0, 0, 0),
        };

        MatrixIterator {
            data: &self.data,
            data_size: self.size,

            iterate_area: iterate_area,
            pos: area.point(),
        }
    }
}

impl<V: Copy> Index<Point<u16>> for Matrix<V> {
    type Output = V;

    fn index(
        &self,
        pos: Point<u16>,
    ) -> &V {
        let index = map_index(pos, self.size);

        &self.data[index]
    }
}

impl<V: Copy> IndexMut<Point<u16>> for Matrix<V> {
    fn index_mut(
        &mut self,
        pos: Point<u16>,
    ) -> &mut V {
        let index = map_index(pos, self.size);

        &mut self.data[index]
    }
}

/// An iterator for the `Matrix`.
pub struct MatrixIterator<'a, V: 'a> {
    /// The raw data we are iterating over.
    data: &'a Vec<V>,

    /// The size of the data when 2D.
    /// Needed for translating index.
    data_size: Size<u16>,

    /// The size of the area we are iterating over.
    iterate_area: Rect<u16>,

    /// Current index in the `Matrix`.
    pos: Point<u16>,
}

impl<'a, V: Copy> Iterator for MatrixIterator<'a, V> {
    type Item = (V, Point<u16>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.y >= self.iterate_area.height {
            return None;
        }

        let i = map_index(self.pos, self.data_size);
        let data = self.data[i];

        let result = Some((data, self.pos));

        // Increment across the x axis.
        if self.pos.x < self.iterate_area.width - 1 {
            self.pos.move_x(1);

        // We've wrapped over the x position.
        } else {
            self.pos.set_x(self.iterate_area.x);
            self.pos.move_y(1);
        }

        result
    }
}

fn map_index(
    pos: Point<u16>,
    size: Size<u16>,
) -> usize {
    if pos.x >= size.width || pos.y >= size.height {
        panic!("Matrix index out of bounds pos: {} size: {}", pos, size);
    }

    ((pos.y * size.width) + pos.x) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn index() {
        let mut matrix = Matrix::new(Size::new(10, 10), 0);
        let index = Point::new(2, 3);

        assert_eq!(matrix[index], 0);
        matrix[index] = 1;
        assert_eq!(matrix[index], 1);
    }

    #[test]
    fn test_index() {
        let matrix_size = Size::new(10, 10);
        let mut matrix = Matrix::new(matrix_size, 1);

        for x in 0..matrix_size.width {
            for y in 0..matrix_size.height {
                matrix[Point::new(x, y)] = x;
            }
        }

        matrix.iter().for_each(|(x, pos)| -> () {
            assert_eq!(x, pos.x);
        });

        for x in 0..matrix_size.width {
            for y in 0..matrix_size.height {
                matrix[Point::new(x, y)] = y;
            }
        }

        matrix.iter().for_each(|(y, pos)| -> () {
            assert_eq!(y, pos.y);
        });
    }

    #[test]
    fn iterate_over_all() {
        let matrix_size = Size::new(10, 10);
        let matrix = Matrix::new(matrix_size, 1);
        let mut count = 0;
        let mut pos_count_x = 0;

        matrix.iter().for_each(|(n, pos)| -> () {
            count += n;
            pos_count_x += pos.x;
        });

        assert_eq!(count, matrix_size.area());
        assert_eq!(pos_count_x, 10 * (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9));
    }

    #[test]
    fn iterate_over_larger_area() {
        let matrix_size = Size::new(10, 10);
        let matrix = Matrix::new(matrix_size, 1);
        let mut count = 0;
        let mut pos_count_x = 0;

        matrix.iter_of(Rect::new(0, 0, 20, 20)).for_each(|(n, pos)| -> () {
            count += n;
            pos_count_x += pos.x;
        });

        assert_eq!(count, matrix_size.area());
        assert_eq!(pos_count_x, matrix_size.height * (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9));
    }

    #[test]
    fn iterate_over_partial_overlap() {
        let matrix_size = Size::new(10, 10);
        let matrix = Matrix::new(matrix_size, 1);
        let mut count = 0;
        let mut pos_count_x = 0;

        matrix.iter_of(Rect::new(5, 5, 20, 20)).for_each(|(n, pos)| -> () {
            count += n;
            pos_count_x += pos.x;
        });

        assert_eq!(count, 5 * 5);
        assert_eq!(pos_count_x, 5 * (5 + 6 + 7 + 8 + 9));
    }

    #[test]
    fn set_and_then_iterate_over_all() {
        let matrix_size = Size::new(10, 10);
        let mut matrix = Matrix::new(matrix_size, 0);

        for x in 0..matrix_size.width {
            for y in 0..matrix_size.height {
                matrix[Point::new(x, y)] = 1;
            }
        }

        let mut count = 0;
        let mut pos_count_x = 0;

        matrix.iter().for_each(|(n, pos)| -> () {
            count += n;
            pos_count_x += pos.x;
        });

        assert_eq!(count, matrix_size.area());
        assert_eq!(pos_count_x, 10 * (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9));
    }
}
