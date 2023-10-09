use dht11::Dht11;
use esp_idf_hal::{delay::Ets, gpio::*};

/// Represents the measurement data from a DHT11 sensor.
pub struct SensorData {
    /// The temperature in degrees Celcius.
    pub temperature: f32,
    /// The relative humidity in percentage.
    pub humidity: f32,
}

/// A trait for extending the Dht11 struct's functionalities.
pub trait Dht11Ext {
    /// Reads data from the DHT11 sensor and returns the result as `SensorData`.
    ///
    /// # Returns
    /// - `Ok(SensorData)`: Measurement data including temperature and humidity.
    /// - `Err(String)`: An error message in case of a measurement failure.
    fn read_data(&mut self) -> Result<SensorData, String>;
}

impl Dht11Ext for Dht11<esp_idf_hal::gpio::PinDriver<'_, AnyIOPin, InputOutput>> {
    fn read_data(&mut self) -> Result<SensorData, String> {
        let mut delay = Ets;

        match self.perform_measurement(&mut delay) {
            Ok(measurement) => {
                let temperature = measurement.temperature as f32 / 10.0;
                let humidity = measurement.humidity as f32 / 10.0;
                Ok(SensorData {
                    temperature,
                    humidity,
                })
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
