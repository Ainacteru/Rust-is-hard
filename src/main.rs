#![no_std]
#![no_main]

use atsamd_hal::bind_interrupts;
use cortex_m_rt::entry;
use defmt::info;
use panic_halt as _;
use embassy_usb_logger as _;

mod usb;

#[entry]
fn main() -> ! {
    info!("starting...");
    loop {
        
    }
}

