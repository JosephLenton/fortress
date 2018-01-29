/// A temperature value.
///
/// Represents celcius, kelvin, and fahrenheit.
#[derive(Copy, Clone)]
pub struct Temperature {
    /// The real temperature value, in celcius.
    celcius: f32,
}

impl Temperature {
    /// Creates a temperature from teh celcius value given.
    ///
    /// @param c Celcius value.
    pub fn from_celcius(c: f32) -> Temperature {
        Temperature { celcius: c }
    }

    /// Creates a temperature from the fahrenheit value given.
    ///
    /// @param f Fahrenheit value.
    pub fn from_fahrenheit(f: f32) -> Temperature {
        Temperature {
            celcius: Temperature::fahrenheit_to_celcius(f),
        }
    }

    /// Creates a temperature from the kelvin value given.
    ///
    /// @param k Kelvin value.
    pub fn from_kelvin(k: f32) -> Temperature {
        Temperature {
            celcius: Temperature::kelvin_to_celcius(k),
        }
    }

    /// Returns this temperature value as Celcius.
    pub fn as_celcius(&self) -> f32 {
        self.celcius
    }

    /// Returns this temperature value as Fahrenheit.
    pub fn as_fahrenheit(&self) -> f32 {
        Temperature::celcius_to_fahrenheit(self.celcius)
    }

    /// Returns this temperature value as Kelvin.
    pub fn as_kelvin(&self) -> f32 {
        Temperature::celcius_to_kelvin(self.celcius)
    }

    /// Give it fahrenheit, get back celcius. Easy peasy.
    pub fn fahrenheit_to_celcius(f: f32) -> f32 {
        (f - 32.0) * (5.0 / 9.0)
    }

    /// Give it celcius, and get back crappy fahrenheit.
    pub fn celcius_to_fahrenheit(c: f32) -> f32 {
        (c / (5.0 / 9.0)) + 32.0
    }

    /// Give it kelvin, and get back celcius.
    pub fn kelvin_to_celcius(k: f32) -> f32 {
        k - 273.15
    }

    /// Give it celcius, and get back kelvin.
    pub fn celcius_to_kelvin(c: f32) -> f32 {
        c + 273.15
    }

    /// Give it kelvin, and get back fahrenheit.
    pub fn kelvin_to_fahrenheit(k: f32) -> f32 {
        Temperature::celcius_to_fahrenheit(Temperature::kelvin_to_celcius(k))
    }

    /// Give it fahrenheit, and get back kelvin.
    pub fn fahrenheit_to_kelvin(f: f32) -> f32 {
        Temperature::celcius_to_kelvin(Temperature::fahrenheit_to_celcius(f))
    }
}
