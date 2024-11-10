#![no_main]

use std::thread;
use std::time::Duration;

use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use palette::IntoColor;
use ws2812_esp32_rmt_driver::driver::Ws2812Esp32RmtDriver;

#[no_mangle]
fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut led =
        Ws2812Esp32RmtDriver::new(peripherals.rmt.channel0, peripherals.pins.gpio9).unwrap();

    let audio_gate = PinDriver::input(peripherals.pins.gpio6).unwrap();

    let mut color = palette::Okhsv::new(0.0, 1.0, 1.0);

    let mut energy: f32 = 0.0;
    loop {
        if audio_gate.is_low() {
            energy -= 0.007;
            energy = energy.clamp(0.0, 1.0);
        } else {
            energy += 0.02;
            energy = energy.clamp(0.0, 1.0);
            color.hue += 0.7;
        }
        color.value = energy;
        let srgb: palette::Srgb = color.into_color();
        let mut r = (srgb.red * 255.0) as u8;
        let mut g = (srgb.green * 255.0) as u8;
        let mut b = (srgb.blue * 255.0) as u8;
        if energy == 0.0 {
            r = 0;
            g = 0;
            b = 0;
        }
        let pixels = std::iter::repeat([r, g, b, 0]).take(18).flatten();
        led.write_blocking(pixels.into_iter()).unwrap();

        thread::sleep(Duration::from_millis(5));
    }
}
