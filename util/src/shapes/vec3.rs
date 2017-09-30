
//!
//! Vec3 is a position in 3 dimensional space.
//!
//! It's a 3D version of the Vec2. It offers some interaction with the Vec2 as
//! well.
//!

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

use super::Size;
use super::Rect;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vec3<N: Add+Sub+Mul+Div+Rem+Copy> {
    pub x : N,
    pub y : N,
    pub z : N,
}

impl<N: Add+Sub+Mul+Div+Rem+Copy> Vec3<N> {

    ///
    /// Trivial constructor.
    ///
    /// You can make it by hand, or you can use this constructor.
    /// It's all the same to me.
    ///
    pub fn new( x : N, y : N, z : N ) -> Vec3<N> {
        return Vec3 {
            x : x,
            y : y,
            z : z,
        }
    }

    ///
    /// Gets the X and Y values of this location.
    ///
    pub fn to_xy( self, size : Size<N> ) -> Vec2<N> {
        return Vec2<N> {
            x : self.x,
            y : self.y,
        }
    }

}

impl<N> PartialEq for Vec3<N>
    where N: Add+Sub+Div+Mul+Rem+Copy + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl<N> Add<Self> for Vec3<N>
    where N:Add<Output=N>+Sub<Output=N>+Div<Output=N>+Mul<Output=N>+Rem<Output=N>+Copy
{
    type Output = Self;

    fn add( self, other: Self ) -> Self {
        return Vec3 {
            x : (self.x + other.x),
            y : (self.y + other.y),
            z : (self.z + other.z),
        }
    }
}

impl<N> Add<Size<N>> for Vec3<N>
    where N:Add<Output=N>+Sub<Output=N>+Div<Output=N>+Mul<Output=N>+Rem<Output=N>+Copy
{
    type Output = Self;

    fn add( self, other: Vec2<N> ) -> Self {
        return Vec3 {
            x : (self.x + other.x),
            y : (self.y + other.y),
            z : self.z,
        }
    }
}

impl<N> Sub for Vec3<N>
    where N:Add<Output=N>+Sub<Output=N>+Div<Output=N>+Mul<Output=N>+Rem<Output=N>+Copy
{
    type Output = Self;

    fn sub( self, other: Self ) -> Self {
        return Vec3 {
            x : (self.x - other.x),
            y : (self.y - other.y),
            z : (self.z - other.z),
        }
    }
}

impl<N> Sub<Size<N>> for Vec3<N>
    where N:Add<Output=N>+Sub<Output=N>+Div<Output=N>+Mul<Output=N>+Rem<Output=N>+Copy
{
    type Output = Self;

    fn sub( self, other: Vec2<N> ) -> Self {
        return Vec3 {
            x : (self.x - other.x),
            y : (self.y - other.y),
            z : (self.z - other.z),
        }
    }
}

impl<N> Rem<Self> for Vec3<N>
    where N:Add<Output=N>+Sub<Output=N>+Mul<Output=N>+Div<Output=N>+Rem<Output=N>+Copy
{
    type Output = Self;

    fn rem( self, other: Self ) -> Self {
        return Vec3 {
            x : (self.x - other.x),
            y : (self.y - other.y),
            z : (self.z - other.z),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(
            Vec3{ x: 1, y: 5, z : 3 },
            Vec3{ x: 1, y: 5, z : 3 }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            Vec3{ x: 1, y: 5, z : 3 },
            Vec3::new( 1, 5,  3 )
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Vec3 { x:  1, y:  5, z : 10 } + Vec3 { x: 93, y: 28, z : -5 },
            Vec3 { x: 94, y: 33, z : 5 }
        );

        assert_eq!(
            Vec3 { x:  50, y: 10, z : -3 } + Vec3 { x: 100, y: 5, z : 3 },
            Vec3 { x: 150, y: 15, z : 0 }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec3{ x: 60, y: 30, z : 90 } - Vec3 { x: 1, y: 5, z : 10 },
            Vec3{ x:59, y: 25, z : 80 }
        );

        assert_eq!(
            Vec3 { x: 50, y: 10, z : -150 } - Vec3 { x: 100, y: 5, z : -250 },
            Vec3 { x: -50, y: 5, z : 100 }
        );
    }
}

