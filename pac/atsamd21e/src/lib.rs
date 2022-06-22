#![doc = "Peripheral access API for ATSAMD21E microcontrollers (generated using svd2rust v0.20.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.20.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn PM();
    fn SYSCTRL();
    fn WDT();
    fn RTC();
    fn EIC();
    fn NVMCTRL();
    fn DMAC();
    fn USB();
    fn EVSYS();
    fn SERCOM0();
    fn SERCOM1();
    fn SERCOM2();
    fn SERCOM3();
    fn TCC0();
    fn TCC1();
    fn TCC2();
    fn TC3();
    fn TC4();
    fn TC5();
    fn ADC();
    fn AC();
    fn DAC();
    fn I2S();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 28] = [
    Vector { _handler: PM },
    Vector { _handler: SYSCTRL },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector { _handler: EIC },
    Vector { _handler: NVMCTRL },
    Vector { _handler: DMAC },
    Vector { _handler: USB },
    Vector { _handler: EVSYS },
    Vector { _handler: SERCOM0 },
    Vector { _handler: SERCOM1 },
    Vector { _handler: SERCOM2 },
    Vector { _handler: SERCOM3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TCC0 },
    Vector { _handler: TCC1 },
    Vector { _handler: TCC2 },
    Vector { _handler: TC3 },
    Vector { _handler: TC4 },
    Vector { _handler: TC5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC },
    Vector { _handler: AC },
    Vector { _handler: DAC },
    Vector { _reserved: 0 },
    Vector { _handler: I2S },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - PM"]
    PM = 0,
    #[doc = "1 - SYSCTRL"]
    SYSCTRL = 1,
    #[doc = "2 - WDT"]
    WDT = 2,
    #[doc = "3 - RTC"]
    RTC = 3,
    #[doc = "4 - EIC"]
    EIC = 4,
    #[doc = "5 - NVMCTRL"]
    NVMCTRL = 5,
    #[doc = "6 - DMAC"]
    DMAC = 6,
    #[doc = "7 - USB"]
    USB = 7,
    #[doc = "8 - EVSYS"]
    EVSYS = 8,
    #[doc = "9 - SERCOM0"]
    SERCOM0 = 9,
    #[doc = "10 - SERCOM1"]
    SERCOM1 = 10,
    #[doc = "11 - SERCOM2"]
    SERCOM2 = 11,
    #[doc = "12 - SERCOM3"]
    SERCOM3 = 12,
    #[doc = "15 - TCC0"]
    TCC0 = 15,
    #[doc = "16 - TCC1"]
    TCC1 = 16,
    #[doc = "17 - TCC2"]
    TCC2 = 17,
    #[doc = "18 - TC3"]
    TC3 = 18,
    #[doc = "19 - TC4"]
    TC4 = 19,
    #[doc = "20 - TC5"]
    TC5 = 20,
    #[doc = "23 - ADC"]
    ADC = 23,
    #[doc = "24 - AC"]
    AC = 24,
    #[doc = "25 - DAC"]
    DAC = 25,
    #[doc = "27 - I2S"]
    I2S = 27,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "Analog Comparators"]
pub struct AC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AC {}
impl AC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ac::RegisterBlock = 0x4200_4400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ac::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AC {
    type Target = ac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AC").finish()
    }
}
#[doc = "Analog Comparators"]
pub mod ac;
#[doc = "Analog Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x4200_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "Analog Digital Converter"]
pub mod adc;
#[doc = "Digital Analog Converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dac::RegisterBlock = 0x4200_4800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DAC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC").finish()
    }
}
#[doc = "Digital Analog Converter"]
pub mod dac;
#[doc = "Direct Memory Access Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dmac::RegisterBlock = 0x4100_4800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMAC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC").finish()
    }
}
#[doc = "Direct Memory Access Controller"]
pub mod dmac;
#[doc = "Device Service Unit"]
pub struct DSU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSU {}
impl DSU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dsu::RegisterBlock = 0x4100_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DSU {
    type Target = dsu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DSU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSU").finish()
    }
}
#[doc = "Device Service Unit"]
pub mod dsu;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC {}
impl EIC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const eic::RegisterBlock = 0x4000_1800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eic::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EIC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EIC").finish()
    }
}
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "Event System Interface"]
pub struct EVSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVSYS {}
impl EVSYS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const evsys::RegisterBlock = 0x4200_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const evsys::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EVSYS {
    type Target = evsys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EVSYS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVSYS").finish()
    }
}
#[doc = "Event System Interface"]
pub mod evsys;
#[doc = "Generic Clock Generator"]
pub struct GCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCLK {}
impl GCLK {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gclk::RegisterBlock = 0x4000_0c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gclk::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GCLK {
    type Target = gclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GCLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCLK").finish()
    }
}
#[doc = "Generic Clock Generator"]
pub mod gclk;
#[doc = "HSB Matrix"]
pub struct HMATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMATRIX {}
impl HMATRIX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const hmatrix::RegisterBlock = 0x4100_7000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hmatrix::RegisterBlock {
        Self::PTR
    }
}
impl Deref for HMATRIX {
    type Target = hmatrix::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for HMATRIX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMATRIX").finish()
    }
}
#[doc = "HSB Matrix"]
pub mod hmatrix;
#[doc = "Inter-IC Sound Interface"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2s::RegisterBlock = 0x4200_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2S {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S").finish()
    }
}
#[doc = "Inter-IC Sound Interface"]
pub mod i2s;
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const mtb::RegisterBlock = 0x4100_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mtb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MTB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTB").finish()
    }
}
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub mod mtb;
#[doc = "Non-Volatile Memory Controller"]
pub struct NVMCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMCTRL {}
impl NVMCTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const nvmctrl::RegisterBlock = 0x4100_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for NVMCTRL {
    type Target = nvmctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for NVMCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVMCTRL").finish()
    }
}
#[doc = "Non-Volatile Memory Controller"]
pub mod nvmctrl;
#[doc = "Peripheral Access Controller 0"]
pub struct PAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC0 {}
impl PAC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pac0::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pac0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PAC0 {
    type Target = pac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PAC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAC0").finish()
    }
}
#[doc = "Peripheral Access Controller 0"]
pub mod pac0;
#[doc = "Peripheral Access Controller 1"]
pub struct PAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC1 {}
impl PAC1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pac0::RegisterBlock = 0x4100_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pac0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PAC1 {
    type Target = pac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PAC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAC1").finish()
    }
}
#[doc = "Peripheral Access Controller 1"]
pub use pac0 as pac1;
#[doc = "Peripheral Access Controller 2"]
pub struct PAC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC2 {}
impl PAC2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pac0::RegisterBlock = 0x4200_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pac0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PAC2 {
    type Target = pac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PAC2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAC2").finish()
    }
}
#[doc = "Peripheral Access Controller 2"]
pub use pac0 as pac2;
#[doc = "Power Manager"]
pub struct PM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PM {}
impl PM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pm::RegisterBlock = 0x4000_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PM").finish()
    }
}
#[doc = "Power Manager"]
pub mod pm;
#[doc = "Port Module"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const port::RegisterBlock = 0x4100_4400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PORT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORT").finish()
    }
}
#[doc = "Port Module"]
pub mod port;
#[doc = "Port Module (IOBUS)"]
pub struct PORT_IOBUS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_IOBUS {}
impl PORT_IOBUS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const port::RegisterBlock = 0x6000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PORT_IOBUS {
    type Target = port::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PORT_IOBUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORT_IOBUS").finish()
    }
}
#[doc = "Port Module (IOBUS)"]
pub use port as port_iobus;
#[doc = "Real-Time Counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x4000_1400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "Real-Time Counter"]
pub mod rtc;
#[doc = "Serial Communication Interface 0"]
pub struct SERCOM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM0 {}
impl SERCOM0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sercom0::RegisterBlock = 0x4200_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SERCOM0 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SERCOM0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SERCOM0").finish()
    }
}
#[doc = "Serial Communication Interface 0"]
pub mod sercom0;
#[doc = "Serial Communication Interface 1"]
pub struct SERCOM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM1 {}
impl SERCOM1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sercom0::RegisterBlock = 0x4200_0c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SERCOM1 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SERCOM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SERCOM1").finish()
    }
}
#[doc = "Serial Communication Interface 1"]
pub use sercom0 as sercom1;
#[doc = "Serial Communication Interface 2"]
pub struct SERCOM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM2 {}
impl SERCOM2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sercom0::RegisterBlock = 0x4200_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SERCOM2 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SERCOM2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SERCOM2").finish()
    }
}
#[doc = "Serial Communication Interface 2"]
pub use sercom0 as sercom2;
#[doc = "Serial Communication Interface 3"]
pub struct SERCOM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM3 {}
impl SERCOM3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sercom0::RegisterBlock = 0x4200_1400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SERCOM3 {
    type Target = sercom0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SERCOM3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SERCOM3").finish()
    }
}
#[doc = "Serial Communication Interface 3"]
pub use sercom0 as sercom3;
#[doc = "System Control"]
pub struct SYSCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTRL {}
impl SYSCTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sysctrl::RegisterBlock = 0x4000_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSCTRL {
    type Target = sysctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCTRL").finish()
    }
}
#[doc = "System Control"]
pub mod sysctrl;
#[doc = "Basic Timer Counter 3"]
pub struct TC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC3 {}
impl TC3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tc3::RegisterBlock = 0x4200_2c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TC3 {
    type Target = tc3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TC3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC3").finish()
    }
}
#[doc = "Basic Timer Counter 3"]
pub mod tc3;
#[doc = "Basic Timer Counter 4"]
pub struct TC4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC4 {}
impl TC4 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tc3::RegisterBlock = 0x4200_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TC4 {
    type Target = tc3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TC4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC4").finish()
    }
}
#[doc = "Basic Timer Counter 4"]
pub use tc3 as tc4;
#[doc = "Basic Timer Counter 5"]
pub struct TC5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC5 {}
impl TC5 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tc3::RegisterBlock = 0x4200_3400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TC5 {
    type Target = tc3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TC5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC5").finish()
    }
}
#[doc = "Basic Timer Counter 5"]
pub use tc3 as tc5;
#[doc = "Timer Counter Control 0"]
pub struct TCC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC0 {}
impl TCC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tcc0::RegisterBlock = 0x4200_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TCC0 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TCC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCC0").finish()
    }
}
#[doc = "Timer Counter Control 0"]
pub mod tcc0;
#[doc = "Timer Counter Control 1"]
pub struct TCC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC1 {}
impl TCC1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tcc0::RegisterBlock = 0x4200_2400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TCC1 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TCC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCC1").finish()
    }
}
#[doc = "Timer Counter Control 1"]
pub use tcc0 as tcc1;
#[doc = "Timer Counter Control 2"]
pub struct TCC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC2 {}
impl TCC2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tcc0::RegisterBlock = 0x4200_2800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TCC2 {
    type Target = tcc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TCC2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCC2").finish()
    }
}
#[doc = "Timer Counter Control 2"]
pub use tcc0 as tcc2;
#[doc = "Universal Serial Bus"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usb::RegisterBlock = 0x4100_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB").finish()
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdt::RegisterBlock = 0x4000_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT").finish()
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AC"]
    pub AC: AC,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DSU"]
    pub DSU: DSU,
    #[doc = "EIC"]
    pub EIC: EIC,
    #[doc = "EVSYS"]
    pub EVSYS: EVSYS,
    #[doc = "GCLK"]
    pub GCLK: GCLK,
    #[doc = "HMATRIX"]
    pub HMATRIX: HMATRIX,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "MTB"]
    pub MTB: MTB,
    #[doc = "NVMCTRL"]
    pub NVMCTRL: NVMCTRL,
    #[doc = "PAC0"]
    pub PAC0: PAC0,
    #[doc = "PAC1"]
    pub PAC1: PAC1,
    #[doc = "PAC2"]
    pub PAC2: PAC2,
    #[doc = "PM"]
    pub PM: PM,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "PORT_IOBUS"]
    pub PORT_IOBUS: PORT_IOBUS,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SERCOM0"]
    pub SERCOM0: SERCOM0,
    #[doc = "SERCOM1"]
    pub SERCOM1: SERCOM1,
    #[doc = "SERCOM2"]
    pub SERCOM2: SERCOM2,
    #[doc = "SERCOM3"]
    pub SERCOM3: SERCOM3,
    #[doc = "SYSCTRL"]
    pub SYSCTRL: SYSCTRL,
    #[doc = "TC3"]
    pub TC3: TC3,
    #[doc = "TC4"]
    pub TC4: TC4,
    #[doc = "TC5"]
    pub TC5: TC5,
    #[doc = "TCC0"]
    pub TCC0: TCC0,
    #[doc = "TCC1"]
    pub TCC1: TCC1,
    #[doc = "TCC2"]
    pub TCC2: TCC2,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AC: AC {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DSU: DSU {
                _marker: PhantomData,
            },
            EIC: EIC {
                _marker: PhantomData,
            },
            EVSYS: EVSYS {
                _marker: PhantomData,
            },
            GCLK: GCLK {
                _marker: PhantomData,
            },
            HMATRIX: HMATRIX {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
            NVMCTRL: NVMCTRL {
                _marker: PhantomData,
            },
            PAC0: PAC0 {
                _marker: PhantomData,
            },
            PAC1: PAC1 {
                _marker: PhantomData,
            },
            PAC2: PAC2 {
                _marker: PhantomData,
            },
            PM: PM {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            PORT_IOBUS: PORT_IOBUS {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SERCOM0: SERCOM0 {
                _marker: PhantomData,
            },
            SERCOM1: SERCOM1 {
                _marker: PhantomData,
            },
            SERCOM2: SERCOM2 {
                _marker: PhantomData,
            },
            SERCOM3: SERCOM3 {
                _marker: PhantomData,
            },
            SYSCTRL: SYSCTRL {
                _marker: PhantomData,
            },
            TC3: TC3 {
                _marker: PhantomData,
            },
            TC4: TC4 {
                _marker: PhantomData,
            },
            TC5: TC5 {
                _marker: PhantomData,
            },
            TCC0: TCC0 {
                _marker: PhantomData,
            },
            TCC1: TCC1 {
                _marker: PhantomData,
            },
            TCC2: TCC2 {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
