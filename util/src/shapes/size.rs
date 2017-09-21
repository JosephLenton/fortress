
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Size<N: Add+Sub+Copy> {
    pub width : N,
    pub height : N,
}

impl<N: Add+Sub+Copy> Size<N> {
    pub fn new( width : N, height : N ) -> Size<N> {
        return Size {
            width : width,
            height : height,
        }
    }
}

impl<N> PartialEq for Size<N>
    where N: Add+Sub+Copy + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        return self.width == other.width && self.height == other.height;
    }
}

impl<N> Add for Size<N>
    where N:Add<Output=N>+Sub<Output=N>+Copy
{
    type Output = Size<N>;

    fn add( self, other: Size<N> ) -> Size<<N as Add>::Output> {
        return Size {
            width : (self.width + other.width),
            height : (self.height + other.height),
        }
    }
}

impl<N> Sub for Size<N>
    where N:Add<Output=N>+Sub<Output=N>+Copy
{
    type Output = Size<N>;

    fn sub( self, other: Size<N> ) -> Size<<N as Sub>::Output> {
        return Size {
            width : (self.width - other.width),
            height : (self.height - other.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(
            Size{ width: 1, height: 5 },
            Size{ width: 1, height: 5 }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            Size{ width: 1, height: 5 },
            Size::new( 1, 5 )
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Size { width:  1, height:  5 } + Size{ width: 93, height: 28 },
            Size { width: 94, height: 33 }
        );

        assert_eq!(
            Size { width:  50, height: 10 } + Size{ width: 100, height: 5 },
            Size { width: 150, height: 15 }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Size{ width: 60, height: 30 } - Size{ width: 1, height: 5 },
            Size{ width:59, height: 25 }
        );

        assert_eq!(
            Size { width: 50, height: 10 } - Size{ width: 100, height: 5 },
            Size { width: -50, height: 5 }
        );
    }
}

