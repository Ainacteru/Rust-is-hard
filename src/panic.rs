use core::{panic::PanicInfo, ptr::{write, write_volatile}};

use atsamd_hal::{clock::GenericClockController, delay::Delay, ehal::delay::DelayNs, pac::{CorePeripherals, Peripherals}, prelude::_atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin};
use cortex_m_rt::pre_init;
use defmt::error;

use crate::RgbRed;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // let info = info.message();
    // let info = info.as_str().unwrap();

    // let mut peripherals = unsafe { Peripherals::steal() };
    // let core = unsafe { CorePeripherals::steal() };
    // let mut clocks = GenericClockController::with_external_32kosc(
    //     peripherals.gclk,
    //     &mut peripherals.pm,
    //     &mut peripherals.sysctrl,
    //     &mut peripherals.nvmctrl,
    // );

   

    // let mut delay = Delay::new(core.SYST, &mut clocks);
    // let pins = crate::Pins::new(peripherals.port);
    // let mut red:RgbRed = pins.rgb_red.into();

    loop {
        defmt::error!("panic! {}", defmt::Display2Format(info));

        cortex_m::asm::delay(5_000_000);
        
    }
}