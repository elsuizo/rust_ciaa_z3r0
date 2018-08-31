#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct INACTVALR {
    bits: bool,
}
impl INACTVALR {
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
pub struct HYSTENR {
    bits: bool,
}
impl HYSTENR {
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
#[doc = "Possible values of the field `WARMTIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARMTIMER {
    #[doc = "4 HFPERCLK cycles"]
    _4CYCLES,
    #[doc = "8 HFPERCLK cycles"]
    _8CYCLES,
    #[doc = "16 HFPERCLK cycles"]
    _16CYCLES,
    #[doc = "32 HFPERCLK cycles"]
    _32CYCLES,
    #[doc = "64 HFPERCLK cycles"]
    _64CYCLES,
    #[doc = "128 HFPERCLK cycles"]
    _128CYCLES,
    #[doc = "256 HFPERCLK cycles"]
    _256CYCLES,
    #[doc = "512 HFPERCLK cycles"]
    _512CYCLES,
}
impl WARMTIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WARMTIMER::_4CYCLES => 0,
            WARMTIMER::_8CYCLES => 1,
            WARMTIMER::_16CYCLES => 2,
            WARMTIMER::_32CYCLES => 3,
            WARMTIMER::_64CYCLES => 4,
            WARMTIMER::_128CYCLES => 5,
            WARMTIMER::_256CYCLES => 6,
            WARMTIMER::_512CYCLES => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WARMTIMER {
        match value {
            0 => WARMTIMER::_4CYCLES,
            1 => WARMTIMER::_8CYCLES,
            2 => WARMTIMER::_16CYCLES,
            3 => WARMTIMER::_32CYCLES,
            4 => WARMTIMER::_64CYCLES,
            5 => WARMTIMER::_128CYCLES,
            6 => WARMTIMER::_256CYCLES,
            7 => WARMTIMER::_512CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline]
    pub fn is_4cycles(&self) -> bool {
        *self == WARMTIMER::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline]
    pub fn is_8cycles(&self) -> bool {
        *self == WARMTIMER::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline]
    pub fn is_16cycles(&self) -> bool {
        *self == WARMTIMER::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline]
    pub fn is_32cycles(&self) -> bool {
        *self == WARMTIMER::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline]
    pub fn is_64cycles(&self) -> bool {
        *self == WARMTIMER::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline]
    pub fn is_128cycles(&self) -> bool {
        *self == WARMTIMER::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == WARMTIMER::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_512CYCLES`"]
    #[inline]
    pub fn is_512cycles(&self) -> bool {
        *self == WARMTIMER::_512CYCLES
    }
}
#[doc = r" Value of the field"]
pub struct IRISER {
    bits: bool,
}
impl IRISER {
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
pub struct IFALLR {
    bits: bool,
}
impl IFALLR {
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
pub struct BIASPROGR {
    bits: u8,
}
impl BIASPROGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HALFBIASR {
    bits: bool,
}
impl HALFBIASR {
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
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
pub struct _INACTVALW<'a> {
    w: &'a mut W,
}
impl<'a> _INACTVALW<'a> {
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
#[doc = r" Proxy"]
pub struct _HYSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTENW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WARMTIME`"]
pub enum WARMTIMEW {
    #[doc = "4 HFPERCLK cycles"]
    _4CYCLES,
    #[doc = "8 HFPERCLK cycles"]
    _8CYCLES,
    #[doc = "16 HFPERCLK cycles"]
    _16CYCLES,
    #[doc = "32 HFPERCLK cycles"]
    _32CYCLES,
    #[doc = "64 HFPERCLK cycles"]
    _64CYCLES,
    #[doc = "128 HFPERCLK cycles"]
    _128CYCLES,
    #[doc = "256 HFPERCLK cycles"]
    _256CYCLES,
    #[doc = "512 HFPERCLK cycles"]
    _512CYCLES,
}
impl WARMTIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WARMTIMEW::_4CYCLES => 0,
            WARMTIMEW::_8CYCLES => 1,
            WARMTIMEW::_16CYCLES => 2,
            WARMTIMEW::_32CYCLES => 3,
            WARMTIMEW::_64CYCLES => 4,
            WARMTIMEW::_128CYCLES => 5,
            WARMTIMEW::_256CYCLES => 6,
            WARMTIMEW::_512CYCLES => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WARMTIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _WARMTIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WARMTIMEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 HFPERCLK cycles"]
    #[inline]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(WARMTIMEW::_4CYCLES)
    }
    #[doc = "8 HFPERCLK cycles"]
    #[inline]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(WARMTIMEW::_8CYCLES)
    }
    #[doc = "16 HFPERCLK cycles"]
    #[inline]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(WARMTIMEW::_16CYCLES)
    }
    #[doc = "32 HFPERCLK cycles"]
    #[inline]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(WARMTIMEW::_32CYCLES)
    }
    #[doc = "64 HFPERCLK cycles"]
    #[inline]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(WARMTIMEW::_64CYCLES)
    }
    #[doc = "128 HFPERCLK cycles"]
    #[inline]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(WARMTIMEW::_128CYCLES)
    }
    #[doc = "256 HFPERCLK cycles"]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(WARMTIMEW::_256CYCLES)
    }
    #[doc = "512 HFPERCLK cycles"]
    #[inline]
    pub fn _512cycles(self) -> &'a mut W {
        self.variant(WARMTIMEW::_512CYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRISEW<'a> {
    w: &'a mut W,
}
impl<'a> _IRISEW<'a> {
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
pub struct _IFALLW<'a> {
    w: &'a mut W,
}
impl<'a> _IFALLW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BIASPROGW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASPROGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HALFBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _HALFBIASW<'a> {
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline]
    pub fn inactval(&self) -> INACTVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INACTVALR { bits }
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline]
    pub fn hysten(&self) -> HYSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYSTENR { bits }
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
    #[inline]
    pub fn warmtime(&self) -> WARMTIMER {
        WARMTIMER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline]
    pub fn irise(&self) -> IRISER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRISER { bits }
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline]
    pub fn ifall(&self) -> IFALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IFALLR { bits }
    }
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline]
    pub fn biasprog(&self) -> BIASPROGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BIASPROGR { bits }
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline]
    pub fn halfbias(&self) -> HALFBIASR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALFBIASR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1191182336 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline]
    pub fn inactval(&mut self) -> _INACTVALW {
        _INACTVALW { w: self }
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline]
    pub fn hysten(&mut self) -> _HYSTENW {
        _HYSTENW { w: self }
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
    #[inline]
    pub fn warmtime(&mut self) -> _WARMTIMEW {
        _WARMTIMEW { w: self }
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline]
    pub fn irise(&mut self) -> _IRISEW {
        _IRISEW { w: self }
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline]
    pub fn ifall(&mut self) -> _IFALLW {
        _IFALLW { w: self }
    }
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline]
    pub fn biasprog(&mut self) -> _BIASPROGW {
        _BIASPROGW { w: self }
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline]
    pub fn halfbias(&mut self) -> _HALFBIASW {
        _HALFBIASW { w: self }
    }
}
