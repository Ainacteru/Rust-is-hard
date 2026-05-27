use core::cell::RefCell;

use atsamd_hal::{clock::UsbClock, gpio::{AnyPin, PA24, PA25}, pac::{Pm, Usb}, usb::UsbBus};
use cortex_m::interrupt::Mutex;
use embassy_usb::class::uac1::speaker::Volume::Muted;
use usb_device::{LangID, bus::{UsbBus as _, UsbBusAllocator}, device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid}};
use usbd_serial::{SerialPort, USB_CLASS_CDC};


static USB_ALLOCATOR: Mutex<RefCell<Option<UsbBusAllocator<UsbBus>>>> = Mutex::new(RefCell::new(None));

struct Usbb {
    usb_bus: UsbBus,
    serial: SerialPort<'static, UsbBus>,
    usb_device: UsbDevice<'static, UsbBus>,
}

impl Usbb {
    pub fn new(
        _clock: &UsbClock,
        pm: &mut Pm,
        dm_pad: impl AnyPin<Id = PA24>,
        dp_pad: impl AnyPin<Id = PA25>,
        _usb: Usb,
    ) -> Self 
    {
        let mut usb_bus = UsbBus::new(_clock, pm, dm_pad, dp_pad, _usb);
        usb_bus.enable();

        cortex_m::interrupt::free(|cs| {
            let a = USB_ALLOCATOR.borrow(cs).borrow_mut();
            *a = Some(UsbBusAllocator::new(usb_bus));
        

            let mut serial = SerialPort::new(&a.unwrap());

            let mut usb_device = UsbDeviceBuilder::new(&a.unwrap(), UsbVidPid(0x16c0, 0x27dd))
                .strings(&[StringDescriptors::new(LangID::EN)
                    .product("Serial port")]).expect("Failed to set strings")
                .device_class(USB_CLASS_CDC)
                .build();
            

            return Self {
                usb_bus,
                serial,
                usb_device,
            }
        });
    }
}