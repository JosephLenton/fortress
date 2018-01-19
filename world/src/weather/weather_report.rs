
use calendar::WorldTime;

pub struct WeatherReport {

    ///
    /// The current temperature, in celsius.
    ///
    temp : i32

}

impl WeatherReport {
    pub fn generate_report( time : WorldTime ) -> WeatherReport {
        WeatherReport {
            temp : WeatherReport::generate_temperature( time ),
        }
    }

    fn generate_temperature( time : WorldTime ) -> i32 {
        20 - ( 16 - time.hour as i32 ).abs()
    }
}

