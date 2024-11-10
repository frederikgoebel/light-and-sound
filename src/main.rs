#![no_main]

use std::thread;
use std::time::Duration;

use color_spaces::OkLab;
use color_spaces::OkLch;
use color_spaces::SRGB;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use ws2812_esp32_rmt_driver::driver::Ws2812Esp32RmtDriver;

mod color_spaces;

#[no_mangle]
fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut led =
        Ws2812Esp32RmtDriver::new(peripherals.rmt.channel0, peripherals.pins.gpio9).unwrap();

    let audio_gate = PinDriver::input(peripherals.pins.gpio6).unwrap();

    let mut color = OkLch {
        l: 1.0,
        c: 0.34,
        h: 0.0,
    };

    let mut energy: f32 = 0.0;
    loop {
        if audio_gate.is_low() {
            energy -= 0.007;
            energy = energy.clamp(0.0, 1.0);
        } else {
            energy += 0.02;
            energy = energy.clamp(0.0, 1.0);
            color.h += 1.0;
            color.h %= 360.0;
        }
        color.l = energy;
        let oklab: OkLab = color.into();
        let srgb: SRGB = oklab.into();
        let mut r = (srgb.r * 255.0) as u8;
        let mut g = (srgb.g * 255.0) as u8;
        let mut b = (srgb.b * 255.0) as u8;
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
