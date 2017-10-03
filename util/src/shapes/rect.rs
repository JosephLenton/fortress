
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

use super::Point2;
use super::Size;

#[derive(Copy, Clone)]
pub struct Rect<N: Add + Sub + Mul + Div + Rem + Copy> {
    pub x : N,
    pub y : N,

    pub width  : N,
    pub height : N,
}

impl<N: Add+Sub+Div+Mul+Rem + Copy> Rect<N> {
    pub fn new( x : N, y : N, w : N, h : N ) -> Rect<N> {
        return Rect {
            x : x,
            y : y,

            width  : w,
            height : h,
        }
    }
}

impl<N> Add<Point2<N>> for Rect<N>
    where N:Add<Output=N>+Sub<Output=N>+Div<Output=N>+Mul<Output=N>+Rem<Output=N>+Copy
{
    type Output = Self;

    fn add( self, other: Point2<N> ) -> Self {
        return Rect {
            x : (self.x + other.x),
            y : (self.y + other.y),
            width  : self.width,
            height : self.height,
        }
    }
}

impl<N> Add<Size<N>> for Rect<N>
    where N:Add<Output=N>+Sub<Output=N>+Div<Output=N>+Mul<Output=N>+Rem<Output=N>+Copy
{
    type Output = Self;

    fn add( self, other: Size<N> ) -> Self {
        return Rect {
            x : self.x,
            y : self.y,
            width  : (self.width  + other.width),
            height : (self.height + other.height),
        }
    }
}

impl<N> Sub<Point2<N>> for Rect<N>
    where N:Add<Output=N>+Sub<Output=N>+Div<Output=N>+Mul<Output=N>+Rem<Output=N>+Copy
{
    type Output = Self;

    fn sub( self, other: Point2<N> ) -> Self {
        return Rect {
            x : (self.x - other.x),
            y : (self.y - other.y),
            width  : self.width,
            height : self.height,
        }
    }
}

impl<N> Sub<Size<N>> for Rect<N>
    where N:Add<Output=N>+Sub<Output=N>+Div<Output=N>+Mul<Output=N>+Rem<Output=N>+Copy
{
    type Output = Self;

    fn sub( self, other: Size<N> ) -> Self {
        return Rect {
            x : self.x,
            y : self.y,
            width  : (self.width  - other.width),
            height : (self.height - other.height),
        }
    }
}

