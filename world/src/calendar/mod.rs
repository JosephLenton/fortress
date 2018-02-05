/// This module deals with time and calendars within the world.
/// It does *not* aim to provide a strict definition of time.
/// i.e. it doesn't state how many months, or how many hours in a day.
///
/// Instead it provides the building blocks to be able to define this yourself.
/// 
mod world_calendar;
mod world_time;

pub use self::world_calendar::WorldCalendar;
pub use self::world_time::WorldTime;
