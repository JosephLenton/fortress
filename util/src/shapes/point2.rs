use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use super::Rect;
use super::Size;

/// A point in 2D space.
/// 
#[derive(Debug, Copy, Clone)]
pub struct Point2<N: Add + Sub + Mul + Div + Rem + Copy> {
    /// It's x location.
    pub x: N,

    /// It's y location.
    pub y: N,
}

impl<N: Add + Sub + Mul + Div + Rem + Copy> Point2<N> {
    /// Trivial constructor.
    ///
    /// You can make it by hand, or you can use this constructor.
    /// It's all the same to me.
    /// 
    pub fn new(
        x: N,
        y: N,
    ) -> Point2<N> {
        Point2 {
            x: x,
            y: y,
        }
    }

    /// Creates a new rectangle and returns it.
    /// The rectangle is centred around this point.
    /// 
    pub fn to_rect(
        self,
        size: Size<N>,
    ) -> Rect<N> {
        Rect {
            x: self.x,
            y: self.y,

            width: size.width,
            height: size.height,
        }
    }
}

impl<N> PartialEq for Point2<N>
where
    N: Add + Sub + Div + Mul + Rem + Copy + PartialEq,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<N> Add<Self> for Point2<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Div<Output = N>
        + Mul<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn add(
        self,
        other: Self,
    ) -> Self {
        Point2 {
            x: (self.x + other.x),
            y: (self.y + other.y),
        }
    }
}

impl<N> Add<Size<N>> for Point2<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Div<Output = N>
        + Mul<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn add(
        self,
        other: Size<N>,
    ) -> Self {
        Point2 {
            x: (self.x + other.width),
            y: (self.y + other.height),
        }
    }
}

impl<N> Sub for Point2<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Div<Output = N>
        + Mul<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn sub(
        self,
        other: Self,
    ) -> Self {
        Point2 {
            x: (self.x - other.x),
            y: (self.y - other.y),
        }
    }
}

impl<N> Sub<Size<N>> for Point2<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Div<Output = N>
        + Mul<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn sub(
        self,
        other: Size<N>,
    ) -> Self {
        Point2 {
            x: (self.x - other.width),
            y: (self.y - other.height),
        }
    }
}

impl<N> Rem<Self> for Point2<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn rem(
        self,
        other: Self,
    ) -> Self {
        Point2 {
            x: (self.x % other.x),
            y: (self.y % other.y),
        }
    }
}

impl<N> Rem<Size<N>> for Point2<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn rem(
        self,
        other: Size<N>,
    ) -> Self {
        Point2 {
            x: (self.x % other.width),
            y: (self.y % other.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(
            Point2 {
                x: 1,
                y: 5,
            },
            Point2 {
                x: 1,
                y: 5,
            }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            Point2 {
                x: 1,
                y: 5,
            },
            Point2::new(1, 5)
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Point2 {
                x: 1,
                y: 5,
            } + Point2 {
                x: 93,
                y: 28,
            },
            Point2 {
                x: 94,
                y: 33,
            }
        );

        assert_eq!(
            Point2 {
                x: 50,
                y: 10,
            } + Point2 {
                x: 100,
                y: 5,
            },
            Point2 {
                x: 150,
                y: 15,
            }
        );
    }

    #[test]
    fn add_size() {
        assert_eq!(
            Point2 {
                x: 1,
                y: 5,
            } + Size {
                width: 93,
                height: 28,
            },
            Point2 {
                x: 94,
                y: 33,
            }
        );

        assert_eq!(
            Point2 {
                x: 50,
                y: 10,
            } + Size {
                width: 100,
                height: 5,
            },
            Point2 {
                x: 150,
                y: 15,
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Point2 {
                x: 60,
                y: 30,
            } - Point2 {
                x: 1,
                y: 5,
            },
            Point2 {
                x: 59,
                y: 25,
            }
        );

        assert_eq!(
            Point2 {
                x: 50,
                y: 10,
            } - Point2 {
                x: 100,
                y: 5,
            },
            Point2 {
                x: -50,
                y: 5,
            }
        );
    }

    #[test]
    fn sub_size() {
        assert_eq!(
            Point2 {
                x: 60,
                y: 30,
            } - Size {
                width: 1,
                height: 5,
            },
            Point2 {
                x: 59,
                y: 25,
            }
        );

        assert_eq!(
            Point2 {
                x: 50,
                y: 10,
            } - Size {
                width: 100,
                height: 5,
            },
            Point2 {
                x: -50,
                y: 5,
            }
        );
    }
}
