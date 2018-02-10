
/// I need `saturating_add` as a type.
/// However it is not a part of any trait.
///
/// So I make a trait, and hook it up for each of the types I'm interested in.
///
/// Done and dusted.
pub trait SaturatingAddT {
    fn saturating_add_t( self, n : Self ) -> Self ;
}

// u numbers.

impl SaturatingAddT for u8 {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

impl SaturatingAddT for u16 {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

impl SaturatingAddT for u32 {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

impl SaturatingAddT for u64 {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

// i numbers.

impl SaturatingAddT for i8 {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

impl SaturatingAddT for i16 {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

impl SaturatingAddT for i32 {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

impl SaturatingAddT for i64 {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

// u/i size
impl SaturatingAddT for isize {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

impl SaturatingAddT for usize {
    #[inline]
    fn saturating_add_t( self, n : Self ) -> Self {
        self.saturating_add(n)
    }
}

