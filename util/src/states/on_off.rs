///
/// It is useful for have an On / Off enum for use instead of boolean values.
/// So your code self documents it better. i.e. `set_logging( OnOff::On )`,
/// instead of `set_logging( true )`.
///
#[derive(Debug, PartialEq, Eq)]
pub enum OnOff {
    /// It's on!
    On,

    /// To represent being off.
    Off,
}
