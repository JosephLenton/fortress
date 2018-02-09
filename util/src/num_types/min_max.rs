
pub trait MinMax {
    fn min_t( self, other : Self ) -> Self;
    fn max_t( self, other : Self ) -> Self;
}

// i types.

impl MinMax for i8 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}


impl MinMax for i16 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

impl MinMax for i32 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

impl MinMax for i64 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

// u types.

impl MinMax for u8 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

impl MinMax for u16 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

impl MinMax for u32 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

impl MinMax for u64 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

// f types.

impl MinMax for f32 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

impl MinMax for f64 {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

// others.

impl MinMax for usize {
    #[inline]
    fn min_t( self, other : Self ) -> Self {
        self.min( other )
    }

    #[inline]
    fn max_t( self, other : Self ) -> Self {
        self.max( other )
    }
}

