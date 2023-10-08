use dht11::Dht11;
use esp_idf_hal::{delay::FreeRtos, gpio::*, i2c::*, peripherals::Peripherals, prelude::*};
use esp_idf_sys as _;
use iot_temperature_monitor::dht11_extension::Dht11Ext;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let dht11_pin = PinDriver::input_output_od(peripherals.pins.gpio4.downgrade()).unwrap();
    let mut dht11 = Dht11::new(dht11_pin);

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let config = I2cConfig::new().baudrate(100u32.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let interface = I2CDisplayInterface::new(i2c);

    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    display.init().unwrap();
    display.clear().unwrap();

    loop {
        match dht11.read_data() {
            Ok(data) => {
                println!("temp: {}ºC, humidity: {}%", data.temperature, data.humidity);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        FreeRtos::delay_ms(2000);
    }
}
