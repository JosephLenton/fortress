use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

use super::Point2;
use super::Size;

///
/// A rectangle.
///
/// It has a position, and a size. It's generic parameter allows
/// you to use any numerical type for it. Integers, floats, etc.
///
#[derive(Copy, Clone)]
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

impl<N: Add + Sub + Div + Mul + Rem + Copy> Rect<N> {
    ///
    /// Trivial constructor.
    ///
    /// Creates a new Rect with the size given.
    ///
    pub fn new(x: N, y: N, w: N, h: N) -> Rect<N> {
        Rect {
            x: x,
            y: y,

            width: w,
            height: h,
        }
    }
}

impl<N> Add<Point2<N>> for Rect<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Div<Output = N>
        + Mul<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn add(self, other: Point2<N>) -> Self {
        Rect {
            x: (self.x + other.x),
            y: (self.y + other.y),
            width: self.width,
            height: self.height,
        }
    }
}

impl<N> Add<Size<N>> for Rect<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Div<Output = N>
        + Mul<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn add(self, other: Size<N>) -> Self {
        Rect {
            x: self.x,
            y: self.y,
            width: (self.width + other.width),
            height: (self.height + other.height),
        }
    }
}

impl<N> Sub<Point2<N>> for Rect<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Div<Output = N>
        + Mul<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn sub(self, other: Point2<N>) -> Self {
        Rect {
            x: (self.x - other.x),
            y: (self.y - other.y),
            width: self.width,
            height: self.height,
        }
    }
}

impl<N> Sub<Size<N>> for Rect<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Div<Output = N>
        + Mul<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn sub(self, other: Size<N>) -> Self {
        Rect {
            x: self.x,
            y: self.y,
            width: (self.width - other.width),
            height: (self.height - other.height),
        }
    }
}
