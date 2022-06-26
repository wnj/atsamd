#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

use hal::clock::GenericClockController;
use hal::sercom::{i2c, uart, Sercom0, Sercom3};
use hal::time::{Hertz};

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

/// Definitions related to pins and pin aliases
pub mod pins {
    use super::hal;

    hal::bsp_pins!(
        PB08 {
            /// Analog Pin 1
            name: a1
        }
        PB09 {
            /// Analog Pin 2
            name: a2
        }
        PA04 {
            /// Analog Pin 3
            name: a3
        }
        PA05 {
            /// Analog Pin 4
            name: a4
        }
        PA06 {
            /// Digital pin number 13, which is also attached to
            /// the blue LED.  PWM capable.
            name: a8
            aliases: {
                PushPullOutput: BlueLed
            }
        }
        PA11 {
            /// Pin 0, UART RX.  Also analog input (A6)
            name: d0
            aliases: {
                AlternateC: UartRx
            }
        }
        PA10 {
            /// Pin 1, UART TX.  Also analog input (A7)
            name: d1
            aliases: {
                AlternateC: UartTx
            }
        }
        PA22 {
            /// The I2C data line
            name: sda
            aliases: {
                AlternateC: Sda
            }
        }
        PA23 {
            /// The I2C clock line
            name: scl
            aliases: {
                AlternateC: Scl
            }
        }
        PA24 {
            /// The USB D- pad
            name: usb_dm
            aliases: {
                AlternateG: UsbDm
            }
        }
        PA25 {
            /// The USB D+ pad
            name: usb_dp
            aliases: {
                AlternateG: UsbDp
            }
        }
    );
}
pub use pins::*;

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]urations.
pub type I2cPads = i2c::Pads<Sercom3, Sda, Scl>;

/// I2C master for the labelled I2C peripheral
///
/// This type implements [`Read`](ehal::blocking::i2c::Read),
/// [`Write`](ehal::blocking::i2c::Write) and
/// [`WriteRead`](ehal::blocking::i2c::WriteRead).
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: Sercom3,
    pm: &mut pac::PM,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom3_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(pm, sercom, pads, freq).baud(baud).enable()
}

/// UART pads
pub type UartPads = uart::Pads<Sercom0, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, sercom0, pads, clock.freq())
        .baud(baud, uart::BaudMode::Fractional(uart::Oversampling::Bits16))
        .enable()
}

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}
