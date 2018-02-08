use std::convert::From;

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use num_types::Num;

use super::Point;
use super::Size;

/// A rectangle.
///
/// It has a position, and a size. It's generic parameter allows
/// you to use any numerical type for it. Integers, floats, etc.
///
#[derive(Copy, Clone, Debug)]
pub struct Rect<N: Add + Sub + Mul + Div + Rem + Copy> {
    /// It's x location.
    pub x: N,

    /// It's y location.
    pub y: N,

    /// The width of this rectangle.
    /// It can have positive or negative size.
    pub width: N,

    /// The height of this rectangle.
    /// It can have positive or negative size.
    pub height: N,
}

impl<N: Add + Sub + Mul + Div + Rem + Copy + From<u8>> Rect<N> {
    /// Trivial constructor.
    ///
    /// Creates a new Rect with the size given.
    pub const fn new(
        x: N,
        y: N,
        w: N,
        h: N,
    ) -> Rect<N> {
        Rect {
            x: x,
            y: y,

            width: w,
            height: h,
        }
    }
}

impl<N: Num<N> + From<u8>> Rect<N> {
    /// Divides the rectangles size by the amount given.
    /// This is done around the centre of the rectangle.
    ///
    /// So this affects both the x/y values, as well as the width and height.
    pub fn divide_around_centre(
        &self,
        divider: N,
    ) -> Rect<N> {
        Rect {
            x: self.x + (self.width / divider) / N::from(2),
            y: self.y + (self.height / divider) / N::from(2),
            width: self.width / divider,
            height: self.height / divider,
        }
    }
}

impl<N: Num<N>> Add<Point<N>> for Rect<N> {
    type Output = Self;

    fn add(
        self,
        other: Point<N>,
    ) -> Self {
        Rect {
            x: (self.x + other.x),
            y: (self.y + other.y),
            width: self.width,
            height: self.height,
        }
    }
}

impl<N: Num<N>> Add<Size<N>> for Rect<N> {
    type Output = Self;

    fn add(
        self,
        other: Size<N>,
    ) -> Self {
        Rect {
            x: self.x,
            y: self.y,
            width: (self.width + other.width),
            height: (self.height + other.height),
        }
    }
}

impl<N: Num<N>> Sub<Point<N>> for Rect<N> {
    type Output = Self;

    fn sub(
        self,
        other: Point<N>,
    ) -> Self {
        Rect {
            x: (self.x - other.x),
            y: (self.y - other.y),
            width: self.width,
            height: self.height,
        }
    }
}

impl<N: Num<N>> Sub<Size<N>> for Rect<N> {
    type Output = Self;

    fn sub(
        self,
        other: Size<N>,
    ) -> Self {
        Rect {
            x: self.x,
            y: self.y,
            width: (self.width - other.width),
            height: (self.height - other.height),
        }
    }
}

impl<N: Num<N>> PartialEq for Rect<N> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width
            && self.height == other.height
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn divide_around_centre() {
        assert_eq!(Rect::new(1, 50, 2, 60), Rect::new(0, 20, 4, 120).divide_around_centre(2),);
    }
}
