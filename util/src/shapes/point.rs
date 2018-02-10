use num_types::FromClamped;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

use num_types::Num;

use super::Rect;
use super::Size;

/// A point in 2D space.
#[derive(Debug, Copy, Clone)]
pub struct Point<N: Add + Sub + Mul + Div + Rem + Copy + AddAssign + DivAssign + MulAssign + SubAssign> {
    /// It's x location.
    pub x: N,

    /// It's y location.
    pub y: N,
}

impl<N: Add + Sub + Mul + Div + Rem + Copy + AddAssign + DivAssign + MulAssign + SubAssign> Point<N> {
    /// Trivial constructor.
    ///
    /// You can make it by hand, or you can use this constructor.
    /// It's all the same to me.
    pub const fn new(
        x: N,
        y: N,
    ) -> Self {
        Point {
            x: x,
            y: y,
        }
    }

    /// Creates a new rectangle and returns it.
    /// The rectangle is centred around this point.
    pub fn combine(
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

    /// Sets a new x position.
    pub fn set_x(
        &mut self,
        x : N,
    ) {
        (*self).x = x;
    }

    /// Sets a new y position.
    pub fn set_y(
        &mut self,
        y : N,
    ) {
        (*self).y = y;
    }

    /// Moves the point along by the x axis.
    pub fn move_x(
        &mut self,
        move_x : N,
    ) {
        (*self).x += move_x;
    }

    /// Moves the point along by the y axis.
    pub fn move_y(
        &mut self,
        move_y : N,
    ) {
        (*self).y += move_y;
    }
}

impl<N: Num<N>> From<(N, N)> for Point<N> {
    fn from((x, y):(N, N)) -> Self {
        Self::new( x, y )
    }
}

impl<N: Num<N>> PartialEq for Point<N> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<N: Num<N>> Add<Self> for Point<N> {
    type Output = Self;

    fn add(
        self,
        other: Self,
    ) -> Self {
        Point {
            x: (self.x + other.x),
            y: (self.y + other.y),
        }
    }
}

impl<N: Num<N>> Add<Size<N>> for Point<N> {
    type Output = Self;

    fn add(
        self,
        other: Size<N>,
    ) -> Self {
        Point {
            x: (self.x + other.width),
            y: (self.y + other.height),
        }
    }
}

impl<N: Num<N>> Sub for Point<N> {
    type Output = Self;

    fn sub(
        self,
        other: Self,
    ) -> Self {
        Point {
            x: (self.x - other.x),
            y: (self.y - other.y),
        }
    }
}

impl<N: Num<N>> Sub<Size<N>> for Point<N> {
    type Output = Self;

    fn sub(
        self,
        other: Size<N>,
    ) -> Self {
        Point {
            x: (self.x - other.width),
            y: (self.y - other.height),
        }
    }
}

impl<N: Num<N>> Mul<N> for Point<N> {
    type Output = Self;

    fn mul(
        self,
        other: N,
    ) -> Self {
        Point {
            x: (self.x * other),
            y: (self.y * other),
        }
    }
}

impl<N: Num<N>> Mul<Size<N>> for Point<N> {
    type Output = Self;

    fn mul(
        self,
        other: Size<N>,
    ) -> Self {
        Point {
            x: (self.x * other.width),
            y: (self.y * other.height),
        }
    }
}

impl<N: Num<N>> Div<N> for Point<N> {
    type Output = Self;

    fn div(
        self,
        other: N,
    ) -> Self {
        Point {
            x: (self.x / other),
            y: (self.y / other),
        }
    }
}

impl<N: Num<N>> Div<Size<N>> for Point<N> {
    type Output = Self;

    fn div(
        self,
        other: Size<N>,
    ) -> Self {
        Point {
            x: (self.x / other.width),
            y: (self.y / other.height),
        }
    }
}

impl<N: Num<N>> Rem<N> for Point<N> {
    type Output = Self;

    fn rem(
        self,
        other: N,
    ) -> Self {
        Point {
            x: (self.x % other),
            y: (self.y % other),
        }
    }
}

impl<N: Num<N>> Rem<Self> for Point<N> {
    type Output = Self;

    fn rem(
        self,
        other: Self,
    ) -> Self {
        Point {
            x: (self.x % other.x),
            y: (self.y % other.y),
        }
    }
}

impl<N: Num<N>> Rem<Size<N>> for Point<N> {
    type Output = Self;

    fn rem(
        self,
        other: Size<N>,
    ) -> Self {
        Point {
            x: (self.x % other.width),
            y: (self.y % other.height),
        }
    }
}

/// For converting Point to Size.
impl<N: Num<N>> Into<Size<N>> for Point<N> {
    fn into(self) -> Size<N> {
        Size {
            width: self.x,
            height: self.y,
        }
    }
}

/// This is to allow creating a new Point, with a new type, from the type given.
/// i.e. `Point::new(1 as u8, 1 as u8)::to::<u32>()`
impl<U: Num<U>> Point<U> {
    pub fn to<F: Num<F> + From<U>>(&self) -> Point<F> {
        Point {
            x: F::from(self.x),
            y: F::from(self.y),
        }
    }
}

/// Converts to a new type. If the current values don't fit in the new type,
/// then it'll be clamped between min and max.
/// i.e. `Point::new(1 as i16, 1 as i16)::to::<u16>()`
impl<A: Num<A>> Point<A> {
    pub fn to_clamped<B: Num<B> + FromClamped<A>>(&self) -> Point<B> {
        Point {
            x: B::from_clamped(self.x),
            y: B::from_clamped(self.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(
            Point {
                x: 1,
                y: 5,
            },
            Point {
                x: 1,
                y: 5,
            }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            Point {
                x: 1,
                y: 5,
            },
            Point::new(1, 5)
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Point {
                x: 1,
                y: 5,
            } + Point {
                x: 93,
                y: 28,
            },
            Point {
                x: 94,
                y: 33,
            }
        );

        assert_eq!(
            Point {
                x: 50,
                y: 10,
            } + Point {
                x: 100,
                y: 5,
            },
            Point {
                x: 150,
                y: 15,
            }
        );
    }

    #[test]
    fn add_size() {
        assert_eq!(
            Point {
                x: 1,
                y: 5,
            } + Size {
                width: 93,
                height: 28,
            },
            Point {
                x: 94,
                y: 33,
            }
        );

        assert_eq!(
            Point {
                x: 50,
                y: 10,
            } + Size {
                width: 100,
                height: 5,
            },
            Point {
                x: 150,
                y: 15,
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Point {
                x: 60,
                y: 30,
            } - Point {
                x: 1,
                y: 5,
            },
            Point {
                x: 59,
                y: 25,
            }
        );

        assert_eq!(
            Point {
                x: 50,
                y: 10,
            } - Point {
                x: 100,
                y: 5,
            },
            Point {
                x: -50,
                y: 5,
            }
        );
    }

    #[test]
    fn sub_size() {
        assert_eq!(
            Point {
                x: 60,
                y: 30,
            } - Size {
                width: 1,
                height: 5,
            },
            Point {
                x: 59,
                y: 25,
            }
        );

        assert_eq!(
            Point {
                x: 50,
                y: 10,
            } - Size {
                width: 100,
                height: 5,
            },
            Point {
                x: -50,
                y: 5,
            }
        );
    }

    #[test]
    pub fn test_from_u8_to_u32() {
        let point_u8 = Point::new(4 as u8, 5 as u8);
        let point_u32 = Point::new(4 as u32, 5 as u32);
        let point_u8_as_u32 = point_u8.to::<u32>();

        assert_eq!(point_u32, point_u8_as_u32);
    }
}
