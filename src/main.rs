#![no_std]
#![no_main]

use atsamd_hal::{ bsp_peripherals, clock::GenericClockController, delay::Delay, pac::{CorePeripherals, Interrupt, NVIC, Peripherals}, prelude::_atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin};
use cortex_m_rt::entry;
use defmt::info;
use panic_halt as _;
use embassy_usb_logger as _;

use testing::{Pins, ehal::delay::DelayNs};

use testing::usb::Usb;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port);

    let usb = Usb::new(&mut clocks, &mut peripherals.pm, pins.usb_dm, pins.usb_dp, peripherals.usb);

    enable_interrupts();
    let mut led = pins.led.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        led.toggle();
        delay.delay_ms(500u32);
    }
}

fn enable_interrupts() {
    unsafe {
        NVIC::unmask(Interrupt::USB);
    }
}

