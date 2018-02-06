use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use super::Num;
use super::Rect;
use super::Size;

/// A point in 2D space.
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

impl<N: Num<N>> PartialEq for Point2<N> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<N: Num<N>> Add<Self> for Point2<N> {
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

impl<N: Num<N>> Add<Size<N>> for Point2<N> {
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

impl<N: Num<N>> Sub for Point2<N> {
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

impl<N: Num<N>> Sub<Size<N>> for Point2<N> {
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

impl<N: Num<N>> Mul<N> for Point2<N> {
    type Output = Self;

    fn mul(
        self,
        other: N,
    ) -> Self {
        Point2 {
            x: (self.x * other),
            y: (self.y * other),
        }
    }
}

impl<N: Num<N>> Div<N> for Point2<N> {
    type Output = Self;

    fn div(
        self,
        other: N,
    ) -> Self {
        Point2 {
            x: (self.x / other),
            y: (self.y / other),
        }
    }
}

impl<N: Num<N>> Rem<N> for Point2<N> {
    type Output = Self;

    fn rem(
        self,
        other: N,
    ) -> Self {
        Point2 {
            x: (self.x % other),
            y: (self.y % other),
        }
    }
}

impl<N: Num<N>> Rem<Self> for Point2<N> {
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

impl<N: Num<N>> Rem<Size<N>> for Point2<N> {
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

/// For converting Point2 to Size.
impl<N: Num<N>> Into<Size<N>> for Point2<N> {
    fn into(self) -> Size<N> {
        Size {
            width: self.x,
            height: self.y,
        }
    }
}

/// This is to allow creating a new Point, with a new type, from the type given.
/// i.e. `Point2::new(1 as u8, 1 as u8)::to::<u32>()`
impl<U: Num<U>> Point2<U> {
    pub fn to<F: Num<F> + From<U>>(&self) -> Point2<F> {
        Point2 {
            x: F::from(self.x),
            y: F::from(self.y),
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

    #[test]
    pub fn test_from_u8_to_u32() {
        let point_u8 = Point2::new(4 as u8, 5 as u8);
        let point_u32 = Point2::new(4 as u32, 5 as u32);
        let point_u8_as_u32 = point_u8.to::<u32>();

        assert_eq!(point_u32, point_u8_as_u32);
    }
}
