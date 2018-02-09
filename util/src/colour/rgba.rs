use super::HSVA;

/// An RGBA colour.
///
#[derive(Debug, Copy, Clone)]
pub struct RGBA {
    /// The red component.
    /// 0 to 255.
    ///
    pub red: u8,

    /// The green component.
    /// 0 to 255.
    ///
    pub green: u8,

    /// The green component.
    /// 0 to 255.
    ///
    pub blue: u8,

    /// The alpha component.
    /// 0 to 255.
    ///
    pub alpha: u8,
}

impl RGBA {
    /// Trivial constructor.
    ///
    pub fn new_rgb(
        r: u8,
        g: u8,
        b: u8,
    ) -> RGBA {
        RGBA {
            red: r,
            blue: b,
            green: g,
            alpha: 255,
        }
    }

    /// Trivial constructor.
    ///
    pub fn new(
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> RGBA {
        RGBA {
            red: r,
            blue: b,
            green: g,
            alpha: a,
        }
    }

    /// Returns the HSVA equivalent of the RGBA object.
    ///
    pub fn to_hsva(&self) -> HSVA {
        if self.red == self.green && self.red == self.blue {
            return HSVA {
                hue: 0.0,
                saturation: 0.0,
                value: f32::from(self.red) / 255.0,
                alpha: self.alpha,
            };
        }

        let r = f32::from(self.red) / 255.0;
        let b = f32::from(self.blue) / 255.0;
        let g = f32::from(self.green) / 255.0;

        let value;
        let diff;
        let min_rgb = r.min(g.min(b));
        let mut hue;

        if r >= g && r >= b {
            value = r;
            diff = value - min_rgb;

            hue = (g - b) / diff;

            if g < b {
                hue += 6.0;
            }
        } else if g >= r && g >= b {
            value = g;
            diff = value - min_rgb;
            hue = (b - r) / diff + 2.0;
        } else {
            value = b;
            diff = value - min_rgb;
            hue = (r - g) / diff + 4.0;
        }

        hue /= 6.0;

        let saturation = diff / value;

        HSVA {
            hue: hue,
            saturation: saturation,
            value: value,
            alpha: self.alpha,
        }
    }

    /// Clones this colour, with the new opacity.
    ///
    pub fn opacity(
        &self,
        alpha: u8,
    ) -> RGBA {
        RGBA {
            red: self.red,
            blue: self.blue,
            green: self.green,
            alpha: alpha,
        }
    }

    /// Returns a copy lightened by the amount given.
    ///
    pub fn lighten(
        &self,
        amount: f32,
    ) -> RGBA {
        self.to_hsva().lighten(amount).to_rgba()
    }

    /// Returns a copy darkened by the amount given.
    ///
    pub fn darken(
        &self,
        amount: f32,
    ) -> RGBA {
        self.to_hsva().darken(amount).to_rgba()
    }
}

impl PartialEq for RGBA {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
            && self.alpha == other.alpha
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let rgba = RGBA {
            red: 255,
            green: 200,
            blue: 99,
            alpha: 200,
        };

        assert_eq!(rgba, rgba.to_hsva().to_rgba());
    }
}
