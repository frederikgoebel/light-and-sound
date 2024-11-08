#![no_main]

use std::{thread, time::Duration};

use esp_idf_svc::hal::{gpio::PinDriver, prelude::Peripherals};
use ws2812_esp32_rmt_driver::driver::Ws2812Esp32RmtDriver;
#[no_mangle]
fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut ws2812 =
        Ws2812Esp32RmtDriver::new(peripherals.rmt.channel0, peripherals.pins.gpio36).unwrap();

    let pin = PinDriver::input(peripherals.pins.gpio21).unwrap();

    let mut energy: f32 = 0.0;
    loop {
        if pin.is_low() {
            energy -= 0.007;
            energy = energy.clamp(0.0, 1.0);
        } else {
            energy += 0.02;
            energy = energy.clamp(0.0, 1.0);
        }
        let v = (energy.powf(2.2) * 255.0) as u8;
        let pixels = std::iter::repeat([v, v, v, v]).take(18).flatten();
        ws2812.write_blocking(pixels.into_iter()).unwrap();
        thread::sleep(Duration::from_millis(10));
    }
}
