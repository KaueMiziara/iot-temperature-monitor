use anyhow;
use dht11::Dht11;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use embedded_svc::wifi::{self, AccessPointConfiguration, AuthMethod};
use esp_idf_hal::{delay::FreeRtos, gpio::*, i2c::*, peripherals::Peripherals, prelude::*};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::EspHttpServer,
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi},
};
use esp_idf_sys as _;
use log::*;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use iot_temperature_monitor::dht11_extension::Dht11Ext;

const SSID: &str = "ESP_WIFI";
const PASSWORD: &str = "WIFI_PASS";

const STACK_SIZE: usize = 10240;
const CHANNEL: u8 = 11;

fn main() -> anyhow::Result<()> {
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

    let _server = create_server()?;

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    loop {
        display.clear(BinaryColor::Off).unwrap();
        match dht11.read_data() {
            Ok(data) => {
                println!("temp: {}ºC, humidity: {}%", data.temperature, data.humidity);

                Text::with_baseline(
                    format!("Temperature: {} C", data.temperature).as_str(),
                    Point::zero(),
                    text_style,
                    Baseline::Top,
                )
                .draw(&mut display)
                .unwrap();

                Text::with_baseline(
                    format!("Humidity: {}%", data.humidity).as_str(),
                    Point::new(0, 16),
                    text_style,
                    Baseline::Top,
                )
                .draw(&mut display)
                .unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        display.flush().unwrap();
        FreeRtos::delay_ms(1500);
    }
}

fn create_server() -> anyhow::Result<EspHttpServer> {
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    let wifi_configuration = wifi::Configuration::AccessPoint(AccessPointConfiguration {
        ssid: SSID.into(),
        ssid_hidden: true,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.into(),
        channel: CHANNEL,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;
    wifi.start()?;
    wifi.wait_netif_up()?;

    info!(
        "Created Wi-Fi with WIFI_SSID `{}` and WIFI_PASS `{}`",
        SSID, PASSWORD
    );

    let server_configuration = esp_idf_svc::http::server::Configuration {
        stack_size: STACK_SIZE,
        ..Default::default()
    };

    core::mem::forget(wifi);

    Ok(EspHttpServer::new(&server_configuration)?)
}
