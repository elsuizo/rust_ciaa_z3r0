#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INPUTSEL {
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
#[doc = "Possible values of the field `POSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POSSELR {
    #[doc = "Channel 0 as positive input."]
    CH0,
    #[doc = "Channel 1 as positive input."]
    CH1,
    #[doc = "Channel 2 as positive input."]
    CH2,
    #[doc = "Channel 3 as positive input."]
    CH3,
    #[doc = "Channel 4 as positive input."]
    CH4,
    #[doc = "Channel 5 as positive input."]
    CH5,
    #[doc = "Channel 6 as positive input."]
    CH6,
    #[doc = "Channel 7 as positive input."]
    CH7,
}
impl POSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POSSELR::CH0 => 0,
            POSSELR::CH1 => 1,
            POSSELR::CH2 => 2,
            POSSELR::CH3 => 3,
            POSSELR::CH4 => 4,
            POSSELR::CH5 => 5,
            POSSELR::CH6 => 6,
            POSSELR::CH7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POSSELR {
        match value {
            0 => POSSELR::CH0,
            1 => POSSELR::CH1,
            2 => POSSELR::CH2,
            3 => POSSELR::CH3,
            4 => POSSELR::CH4,
            5 => POSSELR::CH5,
            6 => POSSELR::CH6,
            7 => POSSELR::CH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline]
    pub fn is_ch0(&self) -> bool {
        *self == POSSELR::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline]
    pub fn is_ch1(&self) -> bool {
        *self == POSSELR::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline]
    pub fn is_ch2(&self) -> bool {
        *self == POSSELR::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline]
    pub fn is_ch3(&self) -> bool {
        *self == POSSELR::CH3
    }
    #[doc = "Checks if the value of the field is `CH4`"]
    #[inline]
    pub fn is_ch4(&self) -> bool {
        *self == POSSELR::CH4
    }
    #[doc = "Checks if the value of the field is `CH5`"]
    #[inline]
    pub fn is_ch5(&self) -> bool {
        *self == POSSELR::CH5
    }
    #[doc = "Checks if the value of the field is `CH6`"]
    #[inline]
    pub fn is_ch6(&self) -> bool {
        *self == POSSELR::CH6
    }
    #[doc = "Checks if the value of the field is `CH7`"]
    #[inline]
    pub fn is_ch7(&self) -> bool {
        *self == POSSELR::CH7
    }
}
#[doc = "Possible values of the field `NEGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEGSELR {
    #[doc = "Channel 0 as negative input."]
    CH0,
    #[doc = "Channel 1 as negative input."]
    CH1,
    #[doc = "Channel 2 as negative input."]
    CH2,
    #[doc = "Channel 3 as negative input."]
    CH3,
    #[doc = "Channel 4 as negative input."]
    CH4,
    #[doc = "Channel 5 as negative input."]
    CH5,
    #[doc = "Channel 6 as negative input."]
    CH6,
    #[doc = "Channel 7 as negative input."]
    CH7,
    #[doc = "1.25 V as negative input."]
    _1V25,
    #[doc = "2.5 V as negative input."]
    _2V5,
    #[doc = "Scaled VDD as negative input."]
    VDD,
    #[doc = "Capacitive sense mode."]
    CAPSENSE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NEGSELR::CH0 => 0,
            NEGSELR::CH1 => 1,
            NEGSELR::CH2 => 2,
            NEGSELR::CH3 => 3,
            NEGSELR::CH4 => 4,
            NEGSELR::CH5 => 5,
            NEGSELR::CH6 => 6,
            NEGSELR::CH7 => 7,
            NEGSELR::_1V25 => 8,
            NEGSELR::_2V5 => 9,
            NEGSELR::VDD => 10,
            NEGSELR::CAPSENSE => 11,
            NEGSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NEGSELR {
        match value {
            0 => NEGSELR::CH0,
            1 => NEGSELR::CH1,
            2 => NEGSELR::CH2,
            3 => NEGSELR::CH3,
            4 => NEGSELR::CH4,
            5 => NEGSELR::CH5,
            6 => NEGSELR::CH6,
            7 => NEGSELR::CH7,
            8 => NEGSELR::_1V25,
            9 => NEGSELR::_2V5,
            10 => NEGSELR::VDD,
            11 => NEGSELR::CAPSENSE,
            i => NEGSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline]
    pub fn is_ch0(&self) -> bool {
        *self == NEGSELR::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline]
    pub fn is_ch1(&self) -> bool {
        *self == NEGSELR::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline]
    pub fn is_ch2(&self) -> bool {
        *self == NEGSELR::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline]
    pub fn is_ch3(&self) -> bool {
        *self == NEGSELR::CH3
    }
    #[doc = "Checks if the value of the field is `CH4`"]
    #[inline]
    pub fn is_ch4(&self) -> bool {
        *self == NEGSELR::CH4
    }
    #[doc = "Checks if the value of the field is `CH5`"]
    #[inline]
    pub fn is_ch5(&self) -> bool {
        *self == NEGSELR::CH5
    }
    #[doc = "Checks if the value of the field is `CH6`"]
    #[inline]
    pub fn is_ch6(&self) -> bool {
        *self == NEGSELR::CH6
    }
    #[doc = "Checks if the value of the field is `CH7`"]
    #[inline]
    pub fn is_ch7(&self) -> bool {
        *self == NEGSELR::CH7
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline]
    pub fn is_1v25(&self) -> bool {
        *self == NEGSELR::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline]
    pub fn is_2v5(&self) -> bool {
        *self == NEGSELR::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline]
    pub fn is_vdd(&self) -> bool {
        *self == NEGSELR::VDD
    }
    #[doc = "Checks if the value of the field is `CAPSENSE`"]
    #[inline]
    pub fn is_capsense(&self) -> bool {
        *self == NEGSELR::CAPSENSE
    }
}
#[doc = r" Value of the field"]
pub struct VDDLEVELR {
    bits: u8,
}
impl VDDLEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPREFR {
    bits: bool,
}
impl LPREFR {
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
pub struct CSRESENR {
    bits: bool,
}
impl CSRESENR {
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
#[doc = "Possible values of the field `CSRESSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRESSELR {
    #[doc = "Internal capacitive sense resistor value 0."]
    RES0,
    #[doc = "Internal capacitive sense resistor value 1."]
    RES1,
    #[doc = "Internal capacitive sense resistor value 2."]
    RES2,
    #[doc = "Internal capacitive sense resistor value 3."]
    RES3,
}
impl CSRESSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSRESSELR::RES0 => 0,
            CSRESSELR::RES1 => 1,
            CSRESSELR::RES2 => 2,
            CSRESSELR::RES3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSRESSELR {
        match value {
            0 => CSRESSELR::RES0,
            1 => CSRESSELR::RES1,
            2 => CSRESSELR::RES2,
            3 => CSRESSELR::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline]
    pub fn is_res0(&self) -> bool {
        *self == CSRESSELR::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline]
    pub fn is_res1(&self) -> bool {
        *self == CSRESSELR::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline]
    pub fn is_res2(&self) -> bool {
        *self == CSRESSELR::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline]
    pub fn is_res3(&self) -> bool {
        *self == CSRESSELR::RES3
    }
}
#[doc = "Values that can be written to the field `POSSEL`"]
pub enum POSSELW {
    #[doc = "Channel 0 as positive input."]
    CH0,
    #[doc = "Channel 1 as positive input."]
    CH1,
    #[doc = "Channel 2 as positive input."]
    CH2,
    #[doc = "Channel 3 as positive input."]
    CH3,
    #[doc = "Channel 4 as positive input."]
    CH4,
    #[doc = "Channel 5 as positive input."]
    CH5,
    #[doc = "Channel 6 as positive input."]
    CH6,
    #[doc = "Channel 7 as positive input."]
    CH7,
}
impl POSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POSSELW::CH0 => 0,
            POSSELW::CH1 => 1,
            POSSELW::CH2 => 2,
            POSSELW::CH3 => 3,
            POSSELW::CH4 => 4,
            POSSELW::CH5 => 5,
            POSSELW::CH6 => 6,
            POSSELW::CH7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _POSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POSSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Channel 0 as positive input."]
    #[inline]
    pub fn ch0(self) -> &'a mut W {
        self.variant(POSSELW::CH0)
    }
    #[doc = "Channel 1 as positive input."]
    #[inline]
    pub fn ch1(self) -> &'a mut W {
        self.variant(POSSELW::CH1)
    }
    #[doc = "Channel 2 as positive input."]
    #[inline]
    pub fn ch2(self) -> &'a mut W {
        self.variant(POSSELW::CH2)
    }
    #[doc = "Channel 3 as positive input."]
    #[inline]
    pub fn ch3(self) -> &'a mut W {
        self.variant(POSSELW::CH3)
    }
    #[doc = "Channel 4 as positive input."]
    #[inline]
    pub fn ch4(self) -> &'a mut W {
        self.variant(POSSELW::CH4)
    }
    #[doc = "Channel 5 as positive input."]
    #[inline]
    pub fn ch5(self) -> &'a mut W {
        self.variant(POSSELW::CH5)
    }
    #[doc = "Channel 6 as positive input."]
    #[inline]
    pub fn ch6(self) -> &'a mut W {
        self.variant(POSSELW::CH6)
    }
    #[doc = "Channel 7 as positive input."]
    #[inline]
    pub fn ch7(self) -> &'a mut W {
        self.variant(POSSELW::CH7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NEGSEL`"]
pub enum NEGSELW {
    #[doc = "Channel 0 as negative input."]
    CH0,
    #[doc = "Channel 1 as negative input."]
    CH1,
    #[doc = "Channel 2 as negative input."]
    CH2,
    #[doc = "Channel 3 as negative input."]
    CH3,
    #[doc = "Channel 4 as negative input."]
    CH4,
    #[doc = "Channel 5 as negative input."]
    CH5,
    #[doc = "Channel 6 as negative input."]
    CH6,
    #[doc = "Channel 7 as negative input."]
    CH7,
    #[doc = "1.25 V as negative input."]
    _1V25,
    #[doc = "2.5 V as negative input."]
    _2V5,
    #[doc = "Scaled VDD as negative input."]
    VDD,
    #[doc = "Capacitive sense mode."]
    CAPSENSE,
}
impl NEGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NEGSELW::CH0 => 0,
            NEGSELW::CH1 => 1,
            NEGSELW::CH2 => 2,
            NEGSELW::CH3 => 3,
            NEGSELW::CH4 => 4,
            NEGSELW::CH5 => 5,
            NEGSELW::CH6 => 6,
            NEGSELW::CH7 => 7,
            NEGSELW::_1V25 => 8,
            NEGSELW::_2V5 => 9,
            NEGSELW::VDD => 10,
            NEGSELW::CAPSENSE => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _NEGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NEGSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Channel 0 as negative input."]
    #[inline]
    pub fn ch0(self) -> &'a mut W {
        self.variant(NEGSELW::CH0)
    }
    #[doc = "Channel 1 as negative input."]
    #[inline]
    pub fn ch1(self) -> &'a mut W {
        self.variant(NEGSELW::CH1)
    }
    #[doc = "Channel 2 as negative input."]
    #[inline]
    pub fn ch2(self) -> &'a mut W {
        self.variant(NEGSELW::CH2)
    }
    #[doc = "Channel 3 as negative input."]
    #[inline]
    pub fn ch3(self) -> &'a mut W {
        self.variant(NEGSELW::CH3)
    }
    #[doc = "Channel 4 as negative input."]
    #[inline]
    pub fn ch4(self) -> &'a mut W {
        self.variant(NEGSELW::CH4)
    }
    #[doc = "Channel 5 as negative input."]
    #[inline]
    pub fn ch5(self) -> &'a mut W {
        self.variant(NEGSELW::CH5)
    }
    #[doc = "Channel 6 as negative input."]
    #[inline]
    pub fn ch6(self) -> &'a mut W {
        self.variant(NEGSELW::CH6)
    }
    #[doc = "Channel 7 as negative input."]
    #[inline]
    pub fn ch7(self) -> &'a mut W {
        self.variant(NEGSELW::CH7)
    }
    #[doc = "1.25 V as negative input."]
    #[inline]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(NEGSELW::_1V25)
    }
    #[doc = "2.5 V as negative input."]
    #[inline]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(NEGSELW::_2V5)
    }
    #[doc = "Scaled VDD as negative input."]
    #[inline]
    pub fn vdd(self) -> &'a mut W {
        self.variant(NEGSELW::VDD)
    }
    #[doc = "Capacitive sense mode."]
    #[inline]
    pub fn capsense(self) -> &'a mut W {
        self.variant(NEGSELW::CAPSENSE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VDDLEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDLEVELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPREFW<'a> {
    w: &'a mut W,
}
impl<'a> _LPREFW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSRESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRESENW<'a> {
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
#[doc = "Values that can be written to the field `CSRESSEL`"]
pub enum CSRESSELW {
    #[doc = "Internal capacitive sense resistor value 0."]
    RES0,
    #[doc = "Internal capacitive sense resistor value 1."]
    RES1,
    #[doc = "Internal capacitive sense resistor value 2."]
    RES2,
    #[doc = "Internal capacitive sense resistor value 3."]
    RES3,
}
impl CSRESSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSRESSELW::RES0 => 0,
            CSRESSELW::RES1 => 1,
            CSRESSELW::RES2 => 2,
            CSRESSELW::RES3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSRESSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRESSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSRESSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Internal capacitive sense resistor value 0."]
    #[inline]
    pub fn res0(self) -> &'a mut W {
        self.variant(CSRESSELW::RES0)
    }
    #[doc = "Internal capacitive sense resistor value 1."]
    #[inline]
    pub fn res1(self) -> &'a mut W {
        self.variant(CSRESSELW::RES1)
    }
    #[doc = "Internal capacitive sense resistor value 2."]
    #[inline]
    pub fn res2(self) -> &'a mut W {
        self.variant(CSRESSELW::RES2)
    }
    #[doc = "Internal capacitive sense resistor value 3."]
    #[inline]
    pub fn res3(self) -> &'a mut W {
        self.variant(CSRESSELW::RES3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline]
    pub fn possel(&self) -> POSSELR {
        POSSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline]
    pub fn negsel(&self) -> NEGSELR {
        NEGSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline]
    pub fn vddlevel(&self) -> VDDLEVELR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VDDLEVELR { bits }
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline]
    pub fn lpref(&self) -> LPREFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPREFR { bits }
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline]
    pub fn csresen(&self) -> CSRESENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSRESENR { bits }
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline]
    pub fn csressel(&self) -> CSRESSELR {
        CSRESSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65664 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline]
    pub fn possel(&mut self) -> _POSSELW {
        _POSSELW { w: self }
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline]
    pub fn negsel(&mut self) -> _NEGSELW {
        _NEGSELW { w: self }
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline]
    pub fn vddlevel(&mut self) -> _VDDLEVELW {
        _VDDLEVELW { w: self }
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline]
    pub fn lpref(&mut self) -> _LPREFW {
        _LPREFW { w: self }
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline]
    pub fn csresen(&mut self) -> _CSRESENW {
        _CSRESENW { w: self }
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline]
    pub fn csressel(&mut self) -> _CSRESSELW {
        _CSRESSELW { w: self }
    }
}
