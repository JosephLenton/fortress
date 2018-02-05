use super::RGBA;

/// A HSV colour, with alpha.
/// 
#[derive(Debug, Copy, Clone)]
pub struct HSVA {
    /// The hue of this colour.
    /// 
    pub hue: f32,

    /// The saturation.
    /// 
    pub saturation: f32,

    /// The value.
    /// 
    pub value: f32,

    /// The alpha.
    ///
    /// A value from 0 to 255.
    /// 
    pub alpha: u8,
}

impl HSVA {
    /// Trivial constructor.
    /// 
    pub fn hsv(
        h: f32,
        s: f32,
        v: f32,
    ) -> HSVA {
        HSVA {
            hue: h,
            saturation: s,
            value: v,
            alpha: 255,
        }
    }

    /// Trivial constructor.
    /// 
    pub fn hsva(
        h: f32,
        s: f32,
        v: f32,
        a: u8,
    ) -> HSVA {
        HSVA {
            hue: h,
            saturation: s,
            value: v,
            alpha: a,
        }
    }

    /// Converts this into RGB and returns that colour.
    /// It has the same alpha channel as this one.
    /// 
    pub fn to_rgba(&self) -> RGBA {
        let r;
        let g;
        let b;

        let i = (self.hue * 6.0).floor() as u32;
        let f = self.hue * 6.0 - i as f32;
        let p = self.value * (1.0 - self.saturation);
        let q = self.value * (1.0 - f * self.saturation);
        let t = self.value * (1.0 - (1.0 - f) * self.saturation);

        match i % 6 {
            0 => {
                r = self.value;
                g = t;
                b = p;
            },
            1 => {
                r = q;
                g = self.value;
                b = p;
            },
            2 => {
                r = p;
                g = self.value;
                b = t;
            },
            3 => {
                r = p;
                g = q;
                b = self.value;
            },
            4 => {
                r = t;
                g = p;
                b = self.value;
            },
            _ => {
                r = self.value;
                g = p;
                b = q;
            },
        }

        RGBA {
            red: (r * 255.0).round() as u8,
            green: (g * 255.0).round() as u8,
            blue: (b * 255.0).round() as u8,
            alpha: self.alpha,
        }
    }

    /// Returns a copy lightened by the amount given.
    /// 
    pub fn lighten(
        &self,
        amount: f32,
    ) -> HSVA {
        HSVA {
            hue: self.hue,
            saturation: (self.saturation + amount).max(1.0),
            value: self.value,
            alpha: self.alpha,
        }
    }

    /// Returns a copy darkened by the amount given.
    /// 
    pub fn darken(
        &self,
        amount: f32,
    ) -> HSVA {
        HSVA {
            hue: self.hue,
            saturation: (self.saturation - amount).max(0.0),
            value: self.value,
            alpha: self.alpha,
        }
    }

    /// Clones this colour, with the new opacity.
    /// 
    pub fn alpha(
        &self,
        alpha: u8,
    ) -> HSVA {
        HSVA {
            hue: self.hue,
            saturation: self.saturation,
            value: self.value,
            alpha: alpha,
        }
    }
}

impl PartialEq for HSVA {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.hue == other.hue && self.saturation == other.saturation && self.value == other.value
            && self.alpha == other.alpha
    }
}
