use super::Num;
use std::mem::size_of;

/// It's useful to convert from a higher precision numerical type,
/// to a lower one. i.e. u64 to u32.
///
/// When this happens you can lose data. There are multiple ways to deal with
/// this. One approach is to clamp within the target range.
///
/// This isn't just about moving to the same type with less bits. It's for any
/// change where you end up with less data. Such as moving between signed and
/// unsigned. i.e. i32 -> u32, and u32 -> i32.
pub trait FromClamped<N: Num<N>> {
    /// Returns the value in the new type, but clamped.
    fn from_clamped(n: N) -> Self;
}

// i8
impl FromClamped<i16> for i8 {
    fn from_clamped(n: i16) -> Self {
        n.max(<i8>::min_value() as i16).min(<i8>::max_value() as i16) as Self
    }
}

impl FromClamped<i32> for i8 {
    fn from_clamped(n: i32) -> Self {
        n.max(<i8>::min_value() as i32).min(<i8>::max_value() as i32) as Self
    }
}

impl FromClamped<i64> for i8 {
    fn from_clamped(n: i64) -> Self {
        n.max(<i8>::min_value() as i64).min(<i8>::max_value() as i64) as Self
    }
}

impl FromClamped<u8> for i8 {
    fn from_clamped(n: u8) -> Self {
        n.min(<i8>::max_value() as u8) as Self
    }
}

impl FromClamped<u16> for i8 {
    fn from_clamped(n: u16) -> Self {
        n.min(<i8>::max_value() as u16) as Self
    }
}

impl FromClamped<u32> for i8 {
    fn from_clamped(n: u32) -> Self {
        n.min(<i8>::max_value() as u32) as Self
    }
}

impl FromClamped<u64> for i8 {
    fn from_clamped(n: u64) -> Self {
        n.min(<i8>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for i8 {
    fn from_clamped(n: f32) -> Self {
        n.min(<i8>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for i8 {
    fn from_clamped(n: f64) -> Self {
        n.min(<i8>::max_value() as f64) as Self
    }
}

// u8
impl FromClamped<i8> for u8 {
    fn from_clamped(n: i8) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<i16> for u8 {
    fn from_clamped(n: i16) -> Self {
        n.max(0).min(<u8>::max_value() as i16) as Self
    }
}

impl FromClamped<i32> for u8 {
    fn from_clamped(n: i32) -> Self {
        n.max(0).min(<u8>::max_value() as i32) as Self
    }
}

impl FromClamped<i64> for u8 {
    fn from_clamped(n: i64) -> Self {
        n.max(0).min(<u8>::max_value() as i64) as Self
    }
}

impl FromClamped<u16> for u8 {
    fn from_clamped(n: u16) -> Self {
        n.min(<u8>::max_value() as u16) as Self
    }
}

impl FromClamped<u32> for u8 {
    fn from_clamped(n: u32) -> Self {
        n.min(<u8>::max_value() as u32) as Self
    }
}

impl FromClamped<u64> for u8 {
    fn from_clamped(n: u64) -> Self {
        n.min(<u8>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for u8 {
    fn from_clamped(n: f32) -> Self {
        n.min(<u8>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for u8 {
    fn from_clamped(n: f64) -> Self {
        n.min(<u8>::max_value() as f64) as Self
    }
}

// i16
impl FromClamped<i32> for i16 {
    fn from_clamped(n: i32) -> Self {
        n.max(<i16>::min_value() as i32).min(<i16>::max_value() as i32) as Self
    }
}

impl FromClamped<i64> for i16 {
    fn from_clamped(n: i64) -> Self {
        n.max(<i16>::min_value() as i64).min(<i16>::max_value() as i64) as Self
    }
}

impl FromClamped<u16> for i16 {
    fn from_clamped(n: u16) -> Self {
        n.min(<i16>::max_value() as u16) as Self
    }
}

impl FromClamped<u32> for i16 {
    fn from_clamped(n: u32) -> Self {
        n.min(<i16>::max_value() as u32) as Self
    }
}

impl FromClamped<u64> for i16 {
    fn from_clamped(n: u64) -> Self {
        n.min(<i16>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for i16 {
    fn from_clamped(n: f32) -> Self {
        n.min(<i16>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for i16 {
    fn from_clamped(n: f64) -> Self {
        n.min(<i16>::max_value() as f64) as Self
    }
}

// u16
impl FromClamped<i16> for u16 {
    fn from_clamped(n: i16) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<i32> for u16 {
    fn from_clamped(n: i32) -> Self {
        n.max(0).min(<u16>::max_value() as i32) as Self
    }
}

impl FromClamped<i64> for u16 {
    fn from_clamped(n: i64) -> Self {
        n.max(0).min(<u16>::max_value() as i64) as Self
    }
}

impl FromClamped<u32> for u16 {
    fn from_clamped(n: u32) -> Self {
        n.min(<u16>::max_value() as u32) as Self
    }
}

impl FromClamped<u64> for u16 {
    fn from_clamped(n: u64) -> Self {
        n.min(<u16>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for u16 {
    fn from_clamped(n: f32) -> Self {
        n.min(<u16>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for u16 {
    fn from_clamped(n: f64) -> Self {
        n.min(<u16>::max_value() as f64) as Self
    }
}

// i32
impl FromClamped<i64> for i32 {
    fn from_clamped(n: i64) -> Self {
        n.max(<i32>::min_value() as i64).min(<i32>::max_value() as i64) as Self
    }
}

impl FromClamped<u32> for i32 {
    fn from_clamped(n: u32) -> Self {
        n.min(<i32>::max_value() as u32) as Self
    }
}

impl FromClamped<u64> for i32 {
    fn from_clamped(n: u64) -> Self {
        n.min(<i32>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for i32 {
    fn from_clamped(n: f32) -> Self {
        n.min(<i32>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for i32 {
    fn from_clamped(n: f64) -> Self {
        n.min(<i32>::max_value() as f64) as Self
    }
}

// u32
impl FromClamped<i32> for u32 {
    fn from_clamped(n: i32) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<i64> for u32 {
    fn from_clamped(n: i64) -> Self {
        n.max(0).min(<u32>::max_value() as i64) as Self
    }
}

impl FromClamped<u64> for u32 {
    fn from_clamped(n: u64) -> Self {
        n.min(<u32>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for u32 {
    fn from_clamped(n: f32) -> Self {
        n.min(<u32>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for u32 {
    fn from_clamped(n: f64) -> Self {
        n.min(<u32>::max_value() as f64) as Self
    }
}

// i64
impl FromClamped<u64> for i64 {
    fn from_clamped(n: u64) -> Self {
        n.min(<i64>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for i64 {
    fn from_clamped(n: f32) -> Self {
        n.min(<i64>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for i64 {
    fn from_clamped(n: f64) -> Self {
        n.min(<i64>::max_value() as f64) as Self
    }
}

// u64
impl FromClamped<i64> for u64 {
    fn from_clamped(n: i64) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<f32> for u64 {
    fn from_clamped(n: f32) -> Self {
        n.min(<u64>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for u64 {
    fn from_clamped(n: f64) -> Self {
        n.min(<u64>::max_value() as f64) as Self
    }
}

// usize
impl FromClamped<i8> for usize {
    fn from_clamped(n: i8) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<i16> for usize {
    fn from_clamped(n: i16) -> Self {
        if size_of::<u16>() <= size_of::<usize>() {
            n.max(0) as Self
        } else {
            n.max(0).min(<usize>::max_value() as i16) as Self
        }
    }
}

impl FromClamped<i32> for usize {
    fn from_clamped(n: i32) -> Self {
        if size_of::<u16>() <= size_of::<usize>() {
            n.max(0) as Self
        } else {
            n.max(0).min(<usize>::max_value() as i32) as Self
        }
    }
}

impl FromClamped<i64> for usize {
    fn from_clamped(n: i64) -> Self {
        if size_of::<u16>() <= size_of::<usize>() {
            n.max(0) as Self
        } else {
            n.max(0).min(<usize>::max_value() as i64) as Self
        }
    }
}

impl FromClamped<u16> for usize {
    fn from_clamped(n: u16) -> Self {
        if size_of::<u16>() <= size_of::<usize>() {
            n as Self
        } else {
            n.min(<usize>::max_value() as u16) as Self
        }
    }
}

impl FromClamped<u32> for usize {
    fn from_clamped(n: u32) -> Self {
        if size_of::<u32>() <= size_of::<usize>() {
            n as Self
        } else {
            n.min(<usize>::max_value() as u32) as Self
        }
    }
}

impl FromClamped<u64> for usize {
    fn from_clamped(n: u64) -> Self {
        if size_of::<u64>() <= size_of::<usize>() {
            n as Self
        } else {
            n.min(<usize>::max_value() as u64) as Self
        }
    }
}

impl FromClamped<f32> for usize {
    fn from_clamped(n: f32) -> Self {
        n.min(<usize>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for usize {
    fn from_clamped(n: f64) -> Self {
        n.min(<usize>::max_value() as f64) as Self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u8_to_i8() {
        assert_eq!(<i8>::from_clamped(<u8>::max_value()), <i8>::max_value());
    }

    #[test]
    fn i8_to_u8() {
        assert_eq!(<u8>::from_clamped(<i8>::min_value()), 0);
        assert_eq!(<u8>::from_clamped(<i8>::max_value()), <i8>::max_value() as u8);
    }
}
