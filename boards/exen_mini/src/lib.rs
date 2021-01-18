#![no_std]

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

extern crate atsamd_hal as hal;

pub use hal::common::*;
pub use hal::samd21::*;
pub use hal::target_device as pac;

use hal::prelude::*;
use hal::*;
// use hal::clock::GenericClockController;

#[cfg(feature = "usb")]
pub use hal::usb;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;

#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

use gpio::{Floating, Input, Port};

define_pins!(
    // maps the pins to the numbers printed on the board
    struct Pins,
    target_device: target_device,

    // blue user LED (on board)
    pin led = a6,

    // off board LEDs
    pin tx_led = b8,
    pin rx_led = b9,

    // serial port
    pin tx = a10,
    pin rx = a11,

    // a? -> d?
    // pin d1 = b8,
    // pin d2 = b9,
    pin d3 = a4,
    pin d4 = a5,

    // I2C
    pin sda = a22,
    pin scl = a23,

    // USB
    pin usb_dm = a24,
    pin usb_dp = a25,
);

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusAllocator<UsbBus> {
    use gpio::IntoFunction;

    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_function(port),
        dp.into_function(port),
        usb,
    ))
}