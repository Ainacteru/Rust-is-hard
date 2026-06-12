use core::panic::PanicInfo;

use atsamd_hal::{clock::GenericClockController, delay::Delay, ehal::delay::DelayNs, pac::{CorePeripherals, Peripherals}, prelude::_atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin};
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
        error!("panic! {}", info);
        // red.toggle();
        // delay.delay_ms(500u32);
    }
}