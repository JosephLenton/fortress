use calendar::WorldCalendar;

pub struct WorldSetup<'a> {
    ///
    /// A description of the calendar for this world.
    ///
    pub calendar: &'a WorldCalendar,
}
