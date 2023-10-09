use dht11::Dht11;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use embedded_svc::wifi::AuthMethod;
use esp_idf_hal::{delay::FreeRtos, gpio::*, i2c::*, peripherals::Peripherals, prelude::*};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};
use esp_idf_sys as _;
use iot_temperature_monitor::dht11_extension::Dht11Ext;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use embedded_svc::wifi::{ClientConfiguration, Configuration};

use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_svc::wifi::AsyncWifi;

use futures::executor::block_on;

use log::info;

const SSID: &str = "SSID";
const PASSWORD: &str = "PASSWORD";

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let dht11_pin = PinDriver::input_output_od(peripherals.pins.gpio4.downgrade()).unwrap();
    let mut dht11 = Dht11::new(dht11_pin);

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let config = I2cConfig::new().baudrate(100u32.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let interface = I2CDisplayInterface::new(i2c);

    let sys_loop = EspSystemEventLoop::take()?;
    let timer_service = EspTaskTimerService::new()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
        timer_service,
    )?;

    block_on(connect_wifi(&mut wifi))?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    info!("Shutting down in 5s...");

    std::thread::sleep(core::time::Duration::from_secs(5));

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

async fn connect_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.into(),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start().await?;
    info!("Wifi started");

    wifi.connect().await?;
    info!("Wifi connected");

    Ok(())
}
