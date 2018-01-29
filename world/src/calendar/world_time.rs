///
/// This represents time in our world.
///
#[derive(Copy, Clone)]
pub struct WorldTime {
    /// The second.
    /// Value from 0 to 60 (or whatever bounds the world uses).
    pub second: u8,

    /// The current minute.
    /// Value from 0 to 60 (or whatever world uses).
    pub minute: u8,

    /// The current hour.
    pub hour: u8,

    /// The current day of the current month.
    /// i.e. 0 to 30.
    pub day: u8,

    /// The current month.
    /// i.e. 0 to 12.
    pub month: u8,

    /// The current year.
    pub year: u8,
}
