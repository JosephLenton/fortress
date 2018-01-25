use calendar::WorldCalendar;

/// This struct contains all the information for creating a world.
/// What that world does with it, well, that's up to the world.
/// But this is the information.
pub struct WorldSetup<'a> {
    /// A description of the calendar for this world.
    pub calendar: &'a WorldCalendar,
}
