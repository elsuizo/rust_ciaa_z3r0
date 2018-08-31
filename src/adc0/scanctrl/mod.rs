#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCANCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct REPR {
    bits: bool,
}
impl REPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DIFFR {
    bits: bool,
}
impl DIFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ADJR {
    bits: bool,
}
impl ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESR {
    #[doc = "12-bit resolution"]
    _12BIT,
    #[doc = "8-bit resolution"]
    _8BIT,
    #[doc = "6-bit resolution"]
    _6BIT,
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    OVS,
}
impl RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESR::_12BIT => 0,
            RESR::_8BIT => 1,
            RESR::_6BIT => 2,
            RESR::OVS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESR {
        match value {
            0 => RESR::_12BIT,
            1 => RESR::_8BIT,
            2 => RESR::_6BIT,
            3 => RESR::OVS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline]
    pub fn is_12bit(&self) -> bool {
        *self == RESR::_12BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline]
    pub fn is_8bit(&self) -> bool {
        *self == RESR::_8BIT
    }
    #[doc = "Checks if the value of the field is `_6BIT`"]
    #[inline]
    pub fn is_6bit(&self) -> bool {
        *self == RESR::_6BIT
    }
    #[doc = "Checks if the value of the field is `OVS`"]
    #[inline]
    pub fn is_ovs(&self) -> bool {
        *self == RESR::OVS
    }
}
#[doc = r" Value of the field"]
pub struct INPUTMASKR {
    bits: u8,
}
impl INPUTMASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `REF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFR {
    #[doc = "Internal 1.25 V reference"]
    _1V25,
    #[doc = "Internal 2.5 V reference"]
    _2V5,
    #[doc = "VDD"]
    VDD,
    #[doc = "Internal differential 5 V reference"]
    _5VDIFF,
    #[doc = "Single ended external reference from ADCn_CH6"]
    EXTSINGLE,
    #[doc = "Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    _2XEXTDIFF,
    #[doc = "Unbuffered 2xVDD"]
    _2XVDD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFR::_1V25 => 0,
            REFR::_2V5 => 1,
            REFR::VDD => 2,
            REFR::_5VDIFF => 3,
            REFR::EXTSINGLE => 4,
            REFR::_2XEXTDIFF => 5,
            REFR::_2XVDD => 6,
            REFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFR {
        match value {
            0 => REFR::_1V25,
            1 => REFR::_2V5,
            2 => REFR::VDD,
            3 => REFR::_5VDIFF,
            4 => REFR::EXTSINGLE,
            5 => REFR::_2XEXTDIFF,
            6 => REFR::_2XVDD,
            i => REFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline]
    pub fn is_1v25(&self) -> bool {
        *self == REFR::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline]
    pub fn is_2v5(&self) -> bool {
        *self == REFR::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline]
    pub fn is_vdd(&self) -> bool {
        *self == REFR::VDD
    }
    #[doc = "Checks if the value of the field is `_5VDIFF`"]
    #[inline]
    pub fn is_5vdiff(&self) -> bool {
        *self == REFR::_5VDIFF
    }
    #[doc = "Checks if the value of the field is `EXTSINGLE`"]
    #[inline]
    pub fn is_extsingle(&self) -> bool {
        *self == REFR::EXTSINGLE
    }
    #[doc = "Checks if the value of the field is `_2XEXTDIFF`"]
    #[inline]
    pub fn is_2xextdiff(&self) -> bool {
        *self == REFR::_2XEXTDIFF
    }
    #[doc = "Checks if the value of the field is `_2XVDD`"]
    #[inline]
    pub fn is_2xvdd(&self) -> bool {
        *self == REFR::_2XVDD
    }
}
#[doc = "Possible values of the field `AT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATR {
    #[doc = "1 ADC_CLK cycle acquisition time for scan samples"]
    _1CYCLE,
    #[doc = "2 ADC_CLK cycles acquisition time for scan samples"]
    _2CYCLES,
    #[doc = "4 ADC_CLK cycles acquisition time for scan samples"]
    _4CYCLES,
    #[doc = "8 ADC_CLK cycles acquisition time for scan samples"]
    _8CYCLES,
    #[doc = "16 ADC_CLK cycles acquisition time for scan samples"]
    _16CYCLES,
    #[doc = "32 ADC_CLK cycles acquisition time for scan samples"]
    _32CYCLES,
    #[doc = "64 ADC_CLK cycles acquisition time for scan samples"]
    _64CYCLES,
    #[doc = "128 ADC_CLK cycles acquisition time for scan samples"]
    _128CYCLES,
    #[doc = "256 ADC_CLK cycles acquisition time for scan samples"]
    _256CYCLES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ATR::_1CYCLE => 0,
            ATR::_2CYCLES => 1,
            ATR::_4CYCLES => 2,
            ATR::_8CYCLES => 3,
            ATR::_16CYCLES => 4,
            ATR::_32CYCLES => 5,
            ATR::_64CYCLES => 6,
            ATR::_128CYCLES => 7,
            ATR::_256CYCLES => 8,
            ATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ATR {
        match value {
            0 => ATR::_1CYCLE,
            1 => ATR::_2CYCLES,
            2 => ATR::_4CYCLES,
            3 => ATR::_8CYCLES,
            4 => ATR::_16CYCLES,
            5 => ATR::_32CYCLES,
            6 => ATR::_64CYCLES,
            7 => ATR::_128CYCLES,
            8 => ATR::_256CYCLES,
            i => ATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1CYCLE`"]
    #[inline]
    pub fn is_1cycle(&self) -> bool {
        *self == ATR::_1CYCLE
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline]
    pub fn is_2cycles(&self) -> bool {
        *self == ATR::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == ATR::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline]
    pub fn is_8cycles(&self) -> bool {
        *self == ATR::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == ATR::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == ATR::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == ATR::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline]
    pub fn is_128cycles(&self) -> bool {
        *self == ATR::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == ATR::_256CYCLES
    }
}
#[doc = r" Value of the field"]
pub struct PRSENR {
    bits: bool,
}
impl PRSENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `PRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSELR {
    #[doc = "PRS ch 0 triggers scan sequence"]
    PRSCH0,
    #[doc = "PRS ch 1 triggers scan sequence"]
    PRSCH1,
    #[doc = "PRS ch 2 triggers scan sequence"]
    PRSCH2,
    #[doc = "PRS ch 3 triggers scan sequence"]
    PRSCH3,
    #[doc = "PRS ch 4 triggers scan sequence"]
    PRSCH4,
    #[doc = "PRS ch 5 triggers scan sequence"]
    PRSCH5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSELR::PRSCH0 => 0,
            PRSSELR::PRSCH1 => 1,
            PRSSELR::PRSCH2 => 2,
            PRSSELR::PRSCH3 => 3,
            PRSSELR::PRSCH4 => 4,
            PRSSELR::PRSCH5 => 5,
            PRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSELR {
        match value {
            0 => PRSSELR::PRSCH0,
            1 => PRSSELR::PRSCH1,
            2 => PRSSELR::PRSCH2,
            3 => PRSSELR::PRSCH3,
            4 => PRSSELR::PRSCH4,
            5 => PRSSELR::PRSCH5,
            i => PRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSELR::PRSCH5
    }
}
#[doc = r" Proxy"]
pub struct _REPW<'a> {
    w: &'a mut W,
}
impl<'a> _REPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _ADJW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RES`"]
pub enum RESW {
    #[doc = "12-bit resolution"]
    _12BIT,
    #[doc = "8-bit resolution"]
    _8BIT,
    #[doc = "6-bit resolution"]
    _6BIT,
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    OVS,
}
impl RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESW::_12BIT => 0,
            RESW::_8BIT => 1,
            RESW::_6BIT => 2,
            RESW::OVS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESW<'a> {
    w: &'a mut W,
}
impl<'a> _RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "12-bit resolution"]
    #[inline]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RESW::_12BIT)
    }
    #[doc = "8-bit resolution"]
    #[inline]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RESW::_8BIT)
    }
    #[doc = "6-bit resolution"]
    #[inline]
    pub fn _6bit(self) -> &'a mut W {
        self.variant(RESW::_6BIT)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline]
    pub fn ovs(self) -> &'a mut W {
        self.variant(RESW::OVS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INPUTMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTMASKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REF`"]
pub enum REFW {
    #[doc = "Internal 1.25 V reference"]
    _1V25,
    #[doc = "Internal 2.5 V reference"]
    _2V5,
    #[doc = "VDD"]
    VDD,
    #[doc = "Internal differential 5 V reference"]
    _5VDIFF,
    #[doc = "Single ended external reference from ADCn_CH6"]
    EXTSINGLE,
    #[doc = "Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    _2XEXTDIFF,
    #[doc = "Unbuffered 2xVDD"]
    _2XVDD,
}
impl REFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFW::_1V25 => 0,
            REFW::_2V5 => 1,
            REFW::VDD => 2,
            REFW::_5VDIFF => 3,
            REFW::EXTSINGLE => 4,
            REFW::_2XEXTDIFF => 5,
            REFW::_2XVDD => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFW<'a> {
    w: &'a mut W,
}
impl<'a> _REFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal 1.25 V reference"]
    #[inline]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REFW::_1V25)
    }
    #[doc = "Internal 2.5 V reference"]
    #[inline]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REFW::_2V5)
    }
    #[doc = "VDD"]
    #[inline]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFW::VDD)
    }
    #[doc = "Internal differential 5 V reference"]
    #[inline]
    pub fn _5vdiff(self) -> &'a mut W {
        self.variant(REFW::_5VDIFF)
    }
    #[doc = "Single ended external reference from ADCn_CH6"]
    #[inline]
    pub fn extsingle(self) -> &'a mut W {
        self.variant(REFW::EXTSINGLE)
    }
    #[doc = "Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    #[inline]
    pub fn _2xextdiff(self) -> &'a mut W {
        self.variant(REFW::_2XEXTDIFF)
    }
    #[doc = "Unbuffered 2xVDD"]
    #[inline]
    pub fn _2xvdd(self) -> &'a mut W {
        self.variant(REFW::_2XVDD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AT`"]
pub enum ATW {
    #[doc = "1 ADC_CLK cycle acquisition time for scan samples"]
    _1CYCLE,
    #[doc = "2 ADC_CLK cycles acquisition time for scan samples"]
    _2CYCLES,
    #[doc = "4 ADC_CLK cycles acquisition time for scan samples"]
    _4CYCLES,
    #[doc = "8 ADC_CLK cycles acquisition time for scan samples"]
    _8CYCLES,
    #[doc = "16 ADC_CLK cycles acquisition time for scan samples"]
    _16CYCLES,
    #[doc = "32 ADC_CLK cycles acquisition time for scan samples"]
    _32CYCLES,
    #[doc = "64 ADC_CLK cycles acquisition time for scan samples"]
    _64CYCLES,
    #[doc = "128 ADC_CLK cycles acquisition time for scan samples"]
    _128CYCLES,
    #[doc = "256 ADC_CLK cycles acquisition time for scan samples"]
    _256CYCLES,
}
impl ATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ATW::_1CYCLE => 0,
            ATW::_2CYCLES => 1,
            ATW::_4CYCLES => 2,
            ATW::_8CYCLES => 3,
            ATW::_16CYCLES => 4,
            ATW::_32CYCLES => 5,
            ATW::_64CYCLES => 6,
            ATW::_128CYCLES => 7,
            ATW::_256CYCLES => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATW<'a> {
    w: &'a mut W,
}
impl<'a> _ATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 ADC_CLK cycle acquisition time for scan samples"]
    #[inline]
    pub fn _1cycle(self) -> &'a mut W {
        self.variant(ATW::_1CYCLE)
    }
    #[doc = "2 ADC_CLK cycles acquisition time for scan samples"]
    #[inline]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(ATW::_2CYCLES)
    }
    #[doc = "4 ADC_CLK cycles acquisition time for scan samples"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(ATW::_4CYCLES)
    }
    #[doc = "8 ADC_CLK cycles acquisition time for scan samples"]
    #[inline]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(ATW::_8CYCLES)
    }
    #[doc = "16 ADC_CLK cycles acquisition time for scan samples"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(ATW::_16CYCLES)
    }
    #[doc = "32 ADC_CLK cycles acquisition time for scan samples"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(ATW::_32CYCLES)
    }
    #[doc = "64 ADC_CLK cycles acquisition time for scan samples"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(ATW::_64CYCLES)
    }
    #[doc = "128 ADC_CLK cycles acquisition time for scan samples"]
    #[inline]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(ATW::_128CYCLES)
    }
    #[doc = "256 ADC_CLK cycles acquisition time for scan samples"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(ATW::_256CYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRSSEL`"]
pub enum PRSSELW {
    #[doc = "PRS ch 0 triggers scan sequence"]
    PRSCH0,
    #[doc = "PRS ch 1 triggers scan sequence"]
    PRSCH1,
    #[doc = "PRS ch 2 triggers scan sequence"]
    PRSCH2,
    #[doc = "PRS ch 3 triggers scan sequence"]
    PRSCH3,
    #[doc = "PRS ch 4 triggers scan sequence"]
    PRSCH4,
    #[doc = "PRS ch 5 triggers scan sequence"]
    PRSCH5,
}
impl PRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSELW::PRSCH0 => 0,
            PRSSELW::PRSCH1 => 1,
            PRSSELW::PRSCH2 => 2,
            PRSSELW::PRSCH3 => 3,
            PRSSELW::PRSCH4 => 4,
            PRSSELW::PRSCH5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline]
    pub fn rep(&self) -> REPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REPR { bits }
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline]
    pub fn diff(&self) -> DIFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIFFR { bits }
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline]
    pub fn adj(&self) -> ADJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADJR { bits }
    }
    #[doc = "Bits 4:5 - Scan Sequence Resolution Select"]
    #[inline]
    pub fn res(&self) -> RESR {
        RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Scan Sequence Input Mask"]
    #[inline]
    pub fn inputmask(&self) -> INPUTMASKR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INPUTMASKR { bits }
    }
    #[doc = "Bits 16:18 - Scan Sequence Reference Selection"]
    #[inline]
    pub fn ref_(&self) -> REFR {
        REFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Scan Sample Acquisition Time"]
    #[inline]
    pub fn at(&self) -> ATR {
        ATR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Scan Sequence PRS Trigger Enable"]
    #[inline]
    pub fn prsen(&self) -> PRSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSENR { bits }
    }
    #[doc = "Bits 28:30 - Scan Sequence PRS Trigger Select"]
    #[inline]
    pub fn prssel(&self) -> PRSSELR {
        PRSSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline]
    pub fn rep(&mut self) -> _REPW {
        _REPW { w: self }
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline]
    pub fn diff(&mut self) -> _DIFFW {
        _DIFFW { w: self }
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline]
    pub fn adj(&mut self) -> _ADJW {
        _ADJW { w: self }
    }
    #[doc = "Bits 4:5 - Scan Sequence Resolution Select"]
    #[inline]
    pub fn res(&mut self) -> _RESW {
        _RESW { w: self }
    }
    #[doc = "Bits 8:15 - Scan Sequence Input Mask"]
    #[inline]
    pub fn inputmask(&mut self) -> _INPUTMASKW {
        _INPUTMASKW { w: self }
    }
    #[doc = "Bits 16:18 - Scan Sequence Reference Selection"]
    #[inline]
    pub fn ref_(&mut self) -> _REFW {
        _REFW { w: self }
    }
    #[doc = "Bits 20:23 - Scan Sample Acquisition Time"]
    #[inline]
    pub fn at(&mut self) -> _ATW {
        _ATW { w: self }
    }
    #[doc = "Bit 24 - Scan Sequence PRS Trigger Enable"]
    #[inline]
    pub fn prsen(&mut self) -> _PRSENW {
        _PRSENW { w: self }
    }
    #[doc = "Bits 28:30 - Scan Sequence PRS Trigger Select"]
    #[inline]
    pub fn prssel(&mut self) -> _PRSSELW {
        _PRSSELW { w: self }
    }
}
