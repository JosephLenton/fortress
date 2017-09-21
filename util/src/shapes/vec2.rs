
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vec2<N: Add+Sub+Copy> {
    pub x : N,
    pub y : N,
}

impl<N: Add+Sub+Copy> Vec2<N> {
    pub fn new( x : N, y : N ) -> Vec2<N> {
        return Vec2 {
            x : x,
            y : y,
        }
    }
}

impl<N> PartialEq for Vec2<N>
    where N: Add+Sub+Copy + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl<N> Add for Vec2<N>
    where N:Add<Output=N>+Sub<Output=N>+Copy
{
    type Output = Vec2<N>;

    fn add( self, other: Vec2<N> ) -> Vec2<<N as Add>::Output> {
        return Vec2 {
            x : (self.x + other.x),
            y : (self.y + other.y),
        }
    }
}

impl<N> Sub for Vec2<N>
    where N:Add<Output=N>+Sub<Output=N>+Copy
{
    type Output = Vec2<N>;

    fn sub( self, other: Vec2<N> ) -> Vec2<<N as Sub>::Output> {
        return Vec2 {
            x : (self.x - other.x),
            y : (self.y - other.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(
            Vec2{ x: 1, y: 5 },
            Vec2{ x: 1, y: 5 }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            Vec2{ x: 1, y: 5 },
            Vec2::new( 1, 5 )
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Vec2 { x:  1, y:  5 } + Vec2{ x: 93, y: 28 },
            Vec2 { x: 94, y: 33 }
        );

        assert_eq!(
            Vec2 { x:  50, y: 10 } + Vec2{ x: 100, y: 5 },
            Vec2 { x: 150, y: 15 }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec2{ x: 60, y: 30 } - Vec2{ x: 1, y: 5 },
            Vec2{ x:59, y: 25 }
        );

        assert_eq!(
            Vec2 { x: 50, y: 10 } - Vec2{ x: 100, y: 5 },
            Vec2 { x: -50, y: 5 }
        );
    }
}

