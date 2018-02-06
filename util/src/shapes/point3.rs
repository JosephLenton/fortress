/// Point3 is a position in 3 dimensional space.
///
/// It's a 3D version of the Point2. It offers some interaction with the Point2
/// as well.
use super::Num;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use super::Point2;

/// A location in 3D space.
#[derive(Debug, Copy, Clone)]
pub struct Point3<N: Add + Sub + Mul + Div + Rem + Copy> {
    /// It's x location.
    pub x: N,

    /// It's y location.
    pub y: N,

    /// It's z location.
    pub z: N,
}

impl<N: Add + Sub + Mul + Div + Rem + Copy> Point3<N> {
    /// Trivial constructor.
    ///
    /// You can make it by hand, or you can use this constructor.
    /// It's all the same to me.
    pub fn new(
        x: N,
        y: N,
        z: N,
    ) -> Point3<N> {
        Point3 {
            x: x,
            y: y,
            z: z,
        }
    }

    /// Gets the X and Y values of this location.
    pub fn to_xy(self) -> Point2<N> {
        Point2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<N: Num<N>> PartialEq for Point3<N> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<N: Num<N>> Add<Self> for Point3<N> {
    type Output = Self;

    fn add(
        self,
        other: Self,
    ) -> Self {
        Point3 {
            x: (self.x + other.x),
            y: (self.y + other.y),
            z: (self.z + other.z),
        }
    }
}

impl<N: Num<N>> Add<Point2<N>> for Point3<N> {
    type Output = Self;

    fn add(
        self,
        other: Point2<N>,
    ) -> Self {
        Point3 {
            x: (self.x + other.x),
            y: (self.y + other.y),
            z: self.z,
        }
    }
}

impl<N: Num<N>> Sub for Point3<N> {
    type Output = Self;

    fn sub(
        self,
        other: Self,
    ) -> Self {
        Point3 {
            x: (self.x - other.x),
            y: (self.y - other.y),
            z: (self.z - other.z),
        }
    }
}

impl<N: Num<N>> Sub<Point2<N>> for Point3<N> {
    type Output = Self;

    fn sub(
        self,
        other: Point2<N>,
    ) -> Self {
        Point3 {
            x: (self.x - other.x),
            y: (self.y - other.y),
            z: self.z,
        }
    }
}

impl<N: Num<N>> Rem<Self> for Point3<N> {
    type Output = Self;

    fn rem(
        self,
        other: Self,
    ) -> Self {
        Point3 {
            x: (self.x % other.x),
            y: (self.y % other.y),
            z: (self.z % other.z),
        }
    }
}

impl<N: Num<N>> Rem<Point2<N>> for Point3<N> {
    type Output = Self;

    fn rem(
        self,
        other: Point2<N>,
    ) -> Self {
        Point3 {
            x: (self.x % other.x),
            y: (self.y % other.y),
            z: self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(
            Point3 {
                x: 1,
                y: 5,
                z: 3,
            },
            Point3 {
                x: 1,
                y: 5,
                z: 3,
            }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            Point3 {
                x: 1,
                y: 5,
                z: 3,
            },
            Point3::new(1, 5, 3)
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Point3 {
                x: 1,
                y: 5,
                z: 10,
            } + Point3 {
                x: 93,
                y: 28,
                z: -5,
            },
            Point3 {
                x: 94,
                y: 33,
                z: 5,
            }
        );

        assert_eq!(
            Point3 {
                x: 50,
                y: 10,
                z: -3,
            } + Point3 {
                x: 100,
                y: 5,
                z: 3,
            },
            Point3 {
                x: 150,
                y: 15,
                z: 0,
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Point3 {
                x: 60,
                y: 30,
                z: 90,
            } - Point3 {
                x: 1,
                y: 5,
                z: 10,
            },
            Point3 {
                x: 59,
                y: 25,
                z: 80,
            }
        );

        assert_eq!(
            Point3 {
                x: 50,
                y: 10,
                z: -150,
            } - Point3 {
                x: 100,
                y: 5,
                z: -250,
            },
            Point3 {
                x: -50,
                y: 5,
                z: 100,
            }
        );
    }
}
