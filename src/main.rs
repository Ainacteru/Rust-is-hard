#![no_std]
#![no_main]

use atsamd_hal::{
    clock::GenericClockController, delay::Delay, ehal::delay::DelayNs, pac::{CorePeripherals, Interrupt, NVIC, Peripherals}, prelude::_atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin
};
use cortex_m_rt::entry;
use defmt::{info};

use samd21_usb_defmt::{Pins, usb::Usb};
use samd21_usb_defmt::timer;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = Pins::new(peripherals.port);


    Usb::set_up(&mut clocks, &mut peripherals.pm, pins.usb_dm, pins.usb_dp, peripherals.usb);
    timer::set_up(&mut clocks, peripherals.tc3, &mut peripherals.pm);

    enable_interrupts();

    let mut led = pins.led.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        info!("hello");
        led.toggle();
        delay.delay_ms(500u32);
    }
}

fn enable_interrupts() {
    unsafe {
        NVIC::unmask(Interrupt::USB);
        NVIC::unmask(Interrupt::TC3);
        NVIC::unmask(Interrupt::SERCOM3);
    }
}
