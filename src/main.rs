use esp_idf_sys as _;
use log::*;

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");
}
