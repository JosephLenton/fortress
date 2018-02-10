use super::Point;
use num_types::FromClamped;
use num_types::Num;
use std::convert::From;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

/// A Size is an area of space with no location.
/// The size of a box, the size of a window, the size of a player.
/// But they don't have a location.
#[derive(Default, Debug, Copy, Clone)]
pub struct Size<N: Add + Sub + Mul + Div + Rem + Copy + AddAssign + DivAssign + MulAssign + SubAssign> {
    /// The width of the area.
    pub width: N,

    /// The height of the area.
    pub height: N,
}

impl<N: Add + Sub + Mul<Output = N> + Div + Rem + Copy + AddAssign + DivAssign + MulAssign + SubAssign> Size<N> {
    /// Trivial constructor.
    ///
    /// Creates a new Size with the width and height given.
    pub const fn new(
        width: N,
        height: N,
    ) -> Self {
        Size {
            width: width,
            height: height,
        }
    }

    /// Converts this to a Point.
    pub const fn to_point2(&self) -> Point<N> {
        Point {
            x: self.width,
            y: self.height,
        }
    }

    /// The area of the size.
    /// Returns width * height.
    pub fn area(&self) -> N {
        self.width * self.height
    }
}

impl<N: Num<N> + PartialOrd> Size<N> {
    /// Returns true if the point is within the size.
    /// The check is exclusive of the size. i.e. `x < width`.
    pub fn contains( &self, point : Point<N> ) -> bool {
        point.x < self.width && point.y < self.height
    }
}

impl Size<u8> {
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

impl Size<u16> {
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

impl Size<i16> {
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

impl Size<u32> {
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

impl Size<i32> {
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

impl Size<u64> {
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

impl Size<i64> {
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

impl Size<f32> {
    pub const fn zero() -> Self {
        Size {
            width: 0.0,
            height: 0.0,
        }
    }
}

impl Size<f64> {
    pub const fn zero() -> Self {
        Size {
            width: 0.0,
            height: 0.0,
        }
    }
}

impl Size<usize> {
    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

impl<N: Num<N>> Add<Size<N>> for Size<N> {
    type Output = Self;

    fn add(
        self,
        other: Self,
    ) -> Self {
        Size {
            width: (self.width + other.width),
            height: (self.height + other.height),
        }
    }
}

impl<N: Num<N>> Sub<Size<N>> for Size<N> {
    type Output = Self;

    fn sub(
        self,
        other: Self,
    ) -> Self {
        Size {
            width: (self.width - other.width),
            height: (self.height - other.height),
        }
    }
}

impl<N: Num<N>> Mul<N> for Size<N> {
    type Output = Self;

    fn mul(
        self,
        other: N,
    ) -> Self {
        Size {
            width: (self.width * other),
            height: (self.height * other),
        }
    }
}

impl<N: Num<N>> Mul<Size<N>> for Size<N> {
    type Output = Self;

    fn mul(
        self,
        other: Size<N>,
    ) -> Self {
        Size {
            width: (self.width * other.width),
            height: (self.height * other.height),
        }
    }
}

impl<N: Num<N>> Div<N> for Size<N> {
    type Output = Self;

    fn div(
        self,
        other: N,
    ) -> Self {
        Size {
            width: (self.width / other),
            height: (self.height / other),
        }
    }
}

impl<N: Num<N>> Div<Size<N>> for Size<N> {
    type Output = Self;

    fn div(
        self,
        other: Size<N>,
    ) -> Self {
        Size {
            width: (self.width / other.width),
            height: (self.height / other.height),
        }
    }
}

impl<N: Num<N>> Rem<N> for Size<N> {
    type Output = Self;

    fn rem(
        self,
        other: N,
    ) -> Self {
        Size {
            width: (self.width % other),
            height: (self.height % other),
        }
    }
}

impl<N: Num<N>> PartialEq for Size<N> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.width == other.width && self.height == other.height
    }
}

/// This is to allow creating a new Size, with a new type, from the type given.
/// i.e. `Size::new(1 as u8, 1 as u8)::to::<f32>()`
impl<A: Num<A>> Size<A> {
    pub fn to<B: Num<B> + From<A>>(&self) -> Size<B> {
        Size {
            width: B::from(self.width),
            height: B::from(self.height),
        }
    }
}

/// Converts to a new type. If the current values don't fit in the new type,
/// then it'll be clamped between min and max.
/// i.e. `Size::new(1 as i16, 1 as i16)::to::<u16>()`
impl<A: Num<A>> Size<A> {
    pub fn to_clamped<B: Num<B> + FromClamped<A>>(&self) -> Size<B> {
        Size {
            width: B::from_clamped(self.width),
            height: B::from_clamped(self.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            Size::default(),
            Size {
                width: 0,
                height: 0,
            },
        );
    }

    #[test]
    fn create() {
        assert_eq!(
            Size {
                width: 1,
                height: 5,
            },
            Size {
                width: 1,
                height: 5,
            }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            Size {
                width: 1,
                height: 5,
            },
            Size::new(1, 5)
        );
    }

    #[test]
    fn to_point2() {
        assert_eq!(
            Point {
                x: 1,
                y: 5,
            },
            Size::new(1, 5).to_point2()
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Size {
                width: 1,
                height: 5,
            } + Size {
                width: 93,
                height: 28,
            },
            Size {
                width: 94,
                height: 33,
            }
        );

        assert_eq!(
            Size {
                width: 50,
                height: 10,
            } + Size {
                width: 100,
                height: 5,
            },
            Size {
                width: 150,
                height: 15,
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Size {
                width: 60,
                height: 30,
            } - Size {
                width: 1,
                height: 5,
            },
            Size {
                width: 59,
                height: 25,
            }
        );

        assert_eq!(
            Size {
                width: 50,
                height: 10,
            } - Size {
                width: 100,
                height: 5,
            },
            Size {
                width: -50,
                height: 5,
            }
        );
    }

    #[test]
    pub fn test_from_u8_to_u32() {
        let size_u8 = Size::new(4 as u8, 5 as u8);
        let size_u32 = Size::new(4 as u32, 5 as u32);
        let size_u8_as_u32 = size_u8.to::<u32>();

        assert_eq!(size_u32, size_u8_as_u32);
    }
}
