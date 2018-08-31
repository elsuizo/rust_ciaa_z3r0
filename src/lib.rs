#![doc = "Peripheral access API for EFM32HG322F64 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA();
    fn GPIO_EVEN();
    fn TIMER0();
    fn ACMP0();
    fn ADC0();
    fn I2C0();
    fn GPIO_ODD();
    fn TIMER1();
    fn USART1_RX();
    fn USART1_TX();
    fn LEUART0();
    fn PCNT0();
    fn RTC();
    fn CMU();
    fn VCMP();
    fn MSC();
    fn AES();
    fn USART0_RX();
    fn USART0_TX();
    fn USB();
    fn TIMER2();
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
pub static __INTERRUPTS: [Vector; 21] = [
    Vector { _handler: DMA },
    Vector {
        _handler: GPIO_EVEN,
    },
    Vector { _handler: TIMER0 },
    Vector { _handler: ACMP0 },
    Vector { _handler: ADC0 },
    Vector { _handler: I2C0 },
    Vector { _handler: GPIO_ODD },
    Vector { _handler: TIMER1 },
    Vector {
        _handler: USART1_RX,
    },
    Vector {
        _handler: USART1_TX,
    },
    Vector { _handler: LEUART0 },
    Vector { _handler: PCNT0 },
    Vector { _handler: RTC },
    Vector { _handler: CMU },
    Vector { _handler: VCMP },
    Vector { _handler: MSC },
    Vector { _handler: AES },
    Vector {
        _handler: USART0_RX,
    },
    Vector {
        _handler: USART0_TX,
    },
    Vector { _handler: USB },
    Vector { _handler: TIMER2 },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ($Name:ident, $handler:path,state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ($Name:ident, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - DMA"]
    DMA,
    #[doc = "1 - GPIO_EVEN"]
    GPIO_EVEN,
    #[doc = "2 - TIMER0"]
    TIMER0,
    #[doc = "3 - ACMP0"]
    ACMP0,
    #[doc = "4 - ADC0"]
    ADC0,
    #[doc = "5 - I2C0"]
    I2C0,
    #[doc = "6 - GPIO_ODD"]
    GPIO_ODD,
    #[doc = "7 - TIMER1"]
    TIMER1,
    #[doc = "8 - USART1_RX"]
    USART1_RX,
    #[doc = "9 - USART1_TX"]
    USART1_TX,
    #[doc = "10 - LEUART0"]
    LEUART0,
    #[doc = "11 - PCNT0"]
    PCNT0,
    #[doc = "12 - RTC"]
    RTC,
    #[doc = "13 - CMU"]
    CMU,
    #[doc = "14 - VCMP"]
    VCMP,
    #[doc = "15 - MSC"]
    MSC,
    #[doc = "16 - AES"]
    AES,
    #[doc = "17 - USART0_RX"]
    USART0_RX,
    #[doc = "18 - USART0_TX"]
    USART0_TX,
    #[doc = "19 - USB"]
    USB,
    #[doc = "20 - TIMER2"]
    TIMER2,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DMA => 0,
            Interrupt::GPIO_EVEN => 1,
            Interrupt::TIMER0 => 2,
            Interrupt::ACMP0 => 3,
            Interrupt::ADC0 => 4,
            Interrupt::I2C0 => 5,
            Interrupt::GPIO_ODD => 6,
            Interrupt::TIMER1 => 7,
            Interrupt::USART1_RX => 8,
            Interrupt::USART1_TX => 9,
            Interrupt::LEUART0 => 10,
            Interrupt::PCNT0 => 11,
            Interrupt::RTC => 12,
            Interrupt::CMU => 13,
            Interrupt::VCMP => 14,
            Interrupt::MSC => 15,
            Interrupt::AES => 16,
            Interrupt::USART0_RX => 17,
            Interrupt::USART0_TX => 18,
            Interrupt::USB => 19,
            Interrupt::TIMER2 => 20,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "AES"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aes::RegisterBlock {
        1074659328 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    fn deref(&self) -> &aes::RegisterBlock {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "AES"]
pub mod aes;
#[doc = "DMA"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        1074536448 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA"]
pub mod dma;
#[doc = "USB"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1074544640 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB"]
pub mod usb;
#[doc = "MSC"]
pub struct MSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSC {}
impl MSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const msc::RegisterBlock {
        1074528256 as *const _
    }
}
impl Deref for MSC {
    type Target = msc::RegisterBlock;
    fn deref(&self) -> &msc::RegisterBlock {
        unsafe { &*MSC::ptr() }
    }
}
#[doc = "MSC"]
pub mod msc;
#[doc = "EMU"]
pub struct EMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMU {}
impl EMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emu::RegisterBlock {
        1074552832 as *const _
    }
}
impl Deref for EMU {
    type Target = emu::RegisterBlock;
    fn deref(&self) -> &emu::RegisterBlock {
        unsafe { &*EMU::ptr() }
    }
}
#[doc = "EMU"]
pub mod emu;
#[doc = "RMU"]
pub struct RMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMU {}
impl RMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rmu::RegisterBlock {
        1074569216 as *const _
    }
}
impl Deref for RMU {
    type Target = rmu::RegisterBlock;
    fn deref(&self) -> &rmu::RegisterBlock {
        unsafe { &*RMU::ptr() }
    }
}
#[doc = "RMU"]
pub mod rmu;
#[doc = "CMU"]
pub struct CMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMU {}
impl CMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmu::RegisterBlock {
        1074561024 as *const _
    }
}
impl Deref for CMU {
    type Target = cmu::RegisterBlock;
    fn deref(&self) -> &cmu::RegisterBlock {
        unsafe { &*CMU::ptr() }
    }
}
#[doc = "CMU"]
pub mod cmu;
#[doc = "TIMER0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer1::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;
    fn deref(&self) -> &timer1::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "TIMER1"]
pub mod timer1;
#[doc = "TIMER2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer2::RegisterBlock {
        1073809408 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer2::RegisterBlock;
    fn deref(&self) -> &timer2::RegisterBlock {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "TIMER2"]
pub mod timer2;
#[doc = "ACMP0"]
pub struct ACMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP0 {}
impl ACMP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acmp0::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for ACMP0 {
    type Target = acmp0::RegisterBlock;
    fn deref(&self) -> &acmp0::RegisterBlock {
        unsafe { &*ACMP0::ptr() }
    }
}
#[doc = "ACMP0"]
pub mod acmp0;
#[doc = "USART0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USART0"]
pub mod usart0;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073792000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USART1"]
pub mod usart1;
#[doc = "PRS"]
pub struct PRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRS {}
impl PRS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const prs::RegisterBlock {
        1074577408 as *const _
    }
}
impl Deref for PRS {
    type Target = prs::RegisterBlock;
    fn deref(&self) -> &prs::RegisterBlock {
        unsafe { &*PRS::ptr() }
    }
}
#[doc = "PRS"]
pub mod prs;
#[doc = "IDAC0"]
pub struct IDAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IDAC0 {}
impl IDAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const idac0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for IDAC0 {
    type Target = idac0::RegisterBlock;
    fn deref(&self) -> &idac0::RegisterBlock {
        unsafe { &*IDAC0::ptr() }
    }
}
#[doc = "IDAC0"]
pub mod idac0;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1073766400 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "VCMP"]
pub struct VCMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VCMP {}
impl VCMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vcmp::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for VCMP {
    type Target = vcmp::RegisterBlock;
    fn deref(&self) -> &vcmp::RegisterBlock {
        unsafe { &*VCMP::ptr() }
    }
}
#[doc = "VCMP"]
pub mod vcmp;
#[doc = "ADC0"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "ADC0"]
pub mod adc0;
#[doc = "LEUART0"]
pub struct LEUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEUART0 {}
impl LEUART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const leuart0::RegisterBlock {
        1074282496 as *const _
    }
}
impl Deref for LEUART0 {
    type Target = leuart0::RegisterBlock;
    fn deref(&self) -> &leuart0::RegisterBlock {
        unsafe { &*LEUART0::ptr() }
    }
}
#[doc = "LEUART0"]
pub mod leuart0;
#[doc = "PCNT0"]
pub struct PCNT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT0 {}
impl PCNT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pcnt0::RegisterBlock {
        1074290688 as *const _
    }
}
impl Deref for PCNT0 {
    type Target = pcnt0::RegisterBlock;
    fn deref(&self) -> &pcnt0::RegisterBlock {
        unsafe { &*PCNT0::ptr() }
    }
}
#[doc = "PCNT0"]
pub mod pcnt0;
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C0"]
pub mod i2c0;
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "WDOG"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog::RegisterBlock {
        1074298880 as *const _
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    fn deref(&self) -> &wdog::RegisterBlock {
        unsafe { &*WDOG::ptr() }
    }
}
#[doc = "WDOG"]
pub mod wdog;
#[doc = "MTB"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mtb::RegisterBlock {
        4026793984 as *const _
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    fn deref(&self) -> &mtb::RegisterBlock {
        unsafe { &*MTB::ptr() }
    }
}
#[doc = "MTB"]
pub mod mtb;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "MSC"]
    pub MSC: MSC,
    #[doc = "EMU"]
    pub EMU: EMU,
    #[doc = "RMU"]
    pub RMU: RMU,
    #[doc = "CMU"]
    pub CMU: CMU,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "ACMP0"]
    pub ACMP0: ACMP0,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "PRS"]
    pub PRS: PRS,
    #[doc = "IDAC0"]
    pub IDAC0: IDAC0,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "VCMP"]
    pub VCMP: VCMP,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "LEUART0"]
    pub LEUART0: LEUART0,
    #[doc = "PCNT0"]
    pub PCNT0: PCNT0,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "MTB"]
    pub MTB: MTB,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
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
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AES: AES {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            MSC: MSC {
                _marker: PhantomData,
            },
            EMU: EMU {
                _marker: PhantomData,
            },
            RMU: RMU {
                _marker: PhantomData,
            },
            CMU: CMU {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            ACMP0: ACMP0 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            PRS: PRS {
                _marker: PhantomData,
            },
            IDAC0: IDAC0 {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            VCMP: VCMP {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            LEUART0: LEUART0 {
                _marker: PhantomData,
            },
            PCNT0: PCNT0 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            WDOG: WDOG {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
        }
    }
}
