use calendar::WorldTime;
use weather::Temperature;

/// A description of the current weather.
pub struct WeatherReport {
    /// The current temperature, in celsius.
    pub temperature: Temperature,
}

impl WeatherReport {
    /// Generates a new report for the time given.
    /// If you give the same time again, then you get the same report.
    pub fn generate_report(time: WorldTime) -> WeatherReport {
        WeatherReport {
            temperature: WeatherReport::generate_temperature(time),
        }
    }

    /// Generates the temperature for the time given.
    fn generate_temperature(time: WorldTime) -> Temperature {
        let celcius = (20 - (16 - i32::from(time.hour)).abs()) as f32;

        Temperature::from_celcius(celcius)
    }
}
