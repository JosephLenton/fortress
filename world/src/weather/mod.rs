/// A module for the weather modelling.
/// This is the traits for modelling, and utility code.
/// 
mod temperature;
mod weather_report;

pub use self::temperature::Temperature;
pub use self::weather_report::WeatherReport;
