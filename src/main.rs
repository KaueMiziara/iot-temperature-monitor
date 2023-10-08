use dht11::Dht11;
use esp_idf_hal::{delay::FreeRtos, gpio::*, peripherals::Peripherals};
use esp_idf_sys as _;
use iot_temperature_monitor::dht11_extension::Dht11Ext;

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let dht11_pin = PinDriver::input_output_od(peripherals.pins.gpio4.downgrade()).unwrap();
    let mut dht11 = Dht11::new(dht11_pin);

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
