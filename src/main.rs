use esp_idf_hal::{gpio::*, peripherals::Peripherals};
use esp_idf_sys as _;
use log::*;

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    info!("Hello, world!");
}
