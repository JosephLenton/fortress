use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

/// A common number trait.
///
/// A number is a collection of the traits below.
/// But you cannot get them all together in Rust.
/// So we have had to build our own.
///
/// The Points, Rects, and Shapes, all use these traits.
pub trait Num<N>
    : Add<Output = N>
    + Sub<Output = N>
    + Mul<Output = N>
    + Div<Output = N>
    + Rem<Output = N>
    + Copy
    + PartialEq {
    type Output = Self;
}

/// You might be wondering 'what does this do?'
/// So am I!
///
/// I think it's saying *'the trait above really does exists, and is
/// implemented, for `U`'*. This applies for cases where `U` matches the `Num`
/// trait.
impl<N, U> Num<N> for U
where
    U: Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Rem<Output = N>
        + Copy
        + PartialEq,
{
    type Output = Self;
}
