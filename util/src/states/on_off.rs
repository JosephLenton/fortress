use std::convert::From;

/// It is useful for have an On / Off enum for use instead of boolean values.
/// So your code self documents it better. i.e. `set_logging( OnOff::On )`,
/// instead of `set_logging( true )`.
/// 
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OnOff {
    /// It's on!
    On,

    /// To represent being off.
    Off,
}

impl OnOff {
    /// Helper for being able to write `some_cond.is_on()`.
    /// Returns true if this is the `OnOff::On` enum value.
    pub fn is_on(self) -> bool {
        self == OnOff::On
    }

    /// Helper for being able to write `some_cond.is_off()`.
    /// Returns true if this is the `OnOff::Off` enum value.
    pub fn is_off(self) -> bool {
        self == OnOff::Off
    }
}

/// This allows you to convert `OnOff` directly into a bool.
impl From<OnOff> for bool {
    fn from(on_off: OnOff) -> bool {
        on_off.is_on()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_off() {
        assert!(OnOff::On.is_on());
        assert!(OnOff::Off.is_off());
    }

    #[test]
    fn on_off_relationship() {
        assert_ne!(OnOff::On.is_on(), OnOff::On.is_off());
        assert_ne!(OnOff::On.is_on(), OnOff::Off.is_on());
        assert_ne!(OnOff::Off.is_on(), OnOff::Off.is_off());
    }

    #[test]
    fn on_off_as_bools() {
        assert_eq!(bool::from(OnOff::On), true);
        assert_eq!(bool::from(OnOff::Off), false);
    }
}
