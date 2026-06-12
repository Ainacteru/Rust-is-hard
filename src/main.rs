#![no_std]
#![no_main]


use atsamd_hal::{ bind_interrupts, clock::GenericClockController, delay::Delay, ehal_async::delay::DelayNs, fugit::{ExtU32, RateExtU32}, pac::{CorePeripherals, Interrupt, NVIC, Peripherals}, prelude::{_atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin, InterruptDrivenTimer}, timer::TimerCounter};
use defmt::{self as _, info};

use testing::{Pins, i2c_master, pac::{Sercom3, tc3}};

use testing::usb::Usb;
use testing::timer;

use bmp390::Bmp390;
use uom::si::thermodynamic_temperature::degree_celsius;

bind_interrupts!(struct Irqs {
    SERCOM3 => atsamd_hal::sercom::i2c::InterruptHandler<Sercom3>;
    TC4 => atsamd_hal::timer::InterruptHandler<atsamd_hal::pac::Tc4>;
    TC5 => atsamd_hal::timer::InterruptHandler<atsamd_hal::pac::Tc5>;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let glck0 = &clocks.gclk0();

    let pins = Pins::new(peripherals.port);

    Usb::set_up(&mut clocks, &mut peripherals.pm, pins.usb_dm, pins.usb_dp, peripherals.usb);
    timer::set_up(&mut clocks, peripherals.tc3, &mut peripherals.pm);

    enable_interrupts();
    let mut led = pins.led.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let tc4_tc5_clock = clocks.tc4_tc5(&glck0).unwrap();
    let tc4 = TimerCounter::tc4_(&tc4_tc5_clock, peripherals.tc4, &mut peripherals.pm);
    let del = tc4.into_future(Irqs);

    let tc5 = TimerCounter::tc5_(&tc4_tc5_clock, peripherals.tc5, &mut peripherals.pm);
    let mut loop_delay = tc5.into_future(Irqs);
    
    defmt::info!("init i2c");
    let i2c = i2c_master(&mut clocks, 400u32.kHz(), peripherals.sercom3, &mut peripherals.pm, pins.sda, pins.scl).into_future(Irqs);

    let config = bmp390::Configuration::default();
    defmt::info!("init sensor");
    let mut sensor = match Bmp390::try_new(i2c, bmp390::Address::Up, del, &config).await {
        Ok(s) => s,
        Err(e) => {
            defmt::error!("BMP390 init failed: {:?}", defmt::Debug2Format(&e));
            loop {}
        }
    };
    
    loop {
        defmt::info!("loop tick");
        match sensor.measure().await {
            Ok(mes) => info!("measurement: {}", &mes.temperature.get::<degree_celsius>()),
            Err(e) => defmt::error!("measure failed: {:?}", defmt::Debug2Format(&e)),
        }

        led.toggle();
        loop_delay.delay_ms(500).await;
    }
}



fn enable_interrupts() {
    unsafe {
        NVIC::unmask(Interrupt::USB);
        NVIC::unmask(Interrupt::TC3);
        NVIC::unmask(Interrupt::TC4);
        NVIC::unmask(Interrupt::TC5);
        NVIC::unmask(Interrupt::SERCOM3);
    }
}

