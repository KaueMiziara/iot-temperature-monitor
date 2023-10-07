use esp_idf_hal::{gpio::*, peripherals::Peripherals};
use esp_idf_sys as _;
use log::*;
use onewire::OneWire;

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut one = PinDriver::input_output(peripherals.pins.gpio18).unwrap();
    let mut _wire = OneWire::new(&mut one, false);

    info!("Hello, world!");
}
