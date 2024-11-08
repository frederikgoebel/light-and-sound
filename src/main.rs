#![no_main]

use std::{thread, time::Duration};

use esp_idf_svc::hal::{gpio::PinDriver, prelude::Peripherals};
use ws2812_esp32_rmt_driver::driver::Ws2812Esp32RmtDriver;
#[no_mangle]
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let mut ws2812 =
        Ws2812Esp32RmtDriver::new(peripherals.rmt.channel0, peripherals.pins.gpio36).unwrap();

    let pin = PinDriver::input(peripherals.pins.gpio21).unwrap();

    loop {
        if pin.is_low() {
            log::info!("Low");
            let pixels = std::iter::repeat([0, 0, 0, 0]).take(18).flatten();
            ws2812.write_blocking(pixels.into_iter()).unwrap();
        } else {
            log::info!("High");
            let pixels = std::iter::repeat([255, 255, 255, 255]).take(18).flatten();
            ws2812.write_blocking(pixels.into_iter()).unwrap();
        }
        thread::sleep(Duration::from_millis(10));
    }
}
