use dht11::Dht11;
use esp_idf_hal::{
    delay::{Ets, FreeRtos},
    gpio::*,
    peripherals::Peripherals,
};
use esp_idf_sys as _;

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let dht11_pin = PinDriver::input_output_od(peripherals.pins.gpio4.downgrade()).unwrap();
    let mut dht11 = Dht11::new(dht11_pin);

    loop {
        let mut dht11_delay = Ets;

        match dht11.perform_measurement(&mut dht11_delay) {
            Ok(measurement) => println!(
                "temp: {}ºC, humidity: {}%",
                (measurement.temperature as f32 / 10.),
                (measurement.humidity as f32 / 10.)
            ),
            Err(e) => println!("{:?}", e),
        }

        FreeRtos::delay_ms(2000);
    }
}
