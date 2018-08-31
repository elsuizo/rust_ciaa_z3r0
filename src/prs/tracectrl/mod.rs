#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRACECTRL {
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
pub struct TSTARTENR {
    bits: bool,
}
impl TSTARTENR {
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
#[doc = "Possible values of the field `TSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTARTR {
    #[doc = "PRS ch 0 is controlling TSTART."]
    PRSCH0,
    #[doc = "PRS ch 1 is controlling TSTART."]
    PRSCH1,
    #[doc = "PRS ch 2 is controlling TSTART."]
    PRSCH2,
    #[doc = "PRS ch 3 is controlling TSTART."]
    PRSCH3,
    #[doc = "PRS ch 4 is controlling TSTART."]
    PRSCH4,
    #[doc = "PRS ch 5 is controlling TSTART."]
    PRSCH5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSTARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTARTR::PRSCH0 => 0,
            TSTARTR::PRSCH1 => 1,
            TSTARTR::PRSCH2 => 2,
            TSTARTR::PRSCH3 => 3,
            TSTARTR::PRSCH4 => 4,
            TSTARTR::PRSCH5 => 5,
            TSTARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTARTR {
        match value {
            0 => TSTARTR::PRSCH0,
            1 => TSTARTR::PRSCH1,
            2 => TSTARTR::PRSCH2,
            3 => TSTARTR::PRSCH3,
            4 => TSTARTR::PRSCH4,
            5 => TSTARTR::PRSCH5,
            i => TSTARTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == TSTARTR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == TSTARTR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == TSTARTR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == TSTARTR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == TSTARTR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == TSTARTR::PRSCH5
    }
}
#[doc = r" Value of the field"]
pub struct TSTOPENR {
    bits: bool,
}
impl TSTOPENR {
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
#[doc = "Possible values of the field `TSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTOPR {
    #[doc = "PRS ch 0 is controlling TSTOP."]
    PRSCH0,
    #[doc = "PRS ch 1 is controlling TSTOP."]
    PRSCH1,
    #[doc = "PRS ch 2 is controlling TSTOP."]
    PRSCH2,
    #[doc = "PRS ch 3 is controlling TSTOP."]
    PRSCH3,
    #[doc = "PRS ch 4 is controlling TSTOP."]
    PRSCH4,
    #[doc = "PRS ch 5 is controlling TSTOP."]
    PRSCH5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTOPR::PRSCH0 => 0,
            TSTOPR::PRSCH1 => 1,
            TSTOPR::PRSCH2 => 2,
            TSTOPR::PRSCH3 => 3,
            TSTOPR::PRSCH4 => 4,
            TSTOPR::PRSCH5 => 5,
            TSTOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTOPR {
        match value {
            0 => TSTOPR::PRSCH0,
            1 => TSTOPR::PRSCH1,
            2 => TSTOPR::PRSCH2,
            3 => TSTOPR::PRSCH3,
            4 => TSTOPR::PRSCH4,
            5 => TSTOPR::PRSCH5,
            i => TSTOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == TSTOPR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == TSTOPR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == TSTOPR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == TSTOPR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == TSTOPR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == TSTOPR::PRSCH5
    }
}
#[doc = r" Proxy"]
pub struct _TSTARTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTARTENW<'a> {
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
#[doc = "Values that can be written to the field `TSTART`"]
pub enum TSTARTW {
    #[doc = "PRS ch 0 is controlling TSTART."]
    PRSCH0,
    #[doc = "PRS ch 1 is controlling TSTART."]
    PRSCH1,
    #[doc = "PRS ch 2 is controlling TSTART."]
    PRSCH2,
    #[doc = "PRS ch 3 is controlling TSTART."]
    PRSCH3,
    #[doc = "PRS ch 4 is controlling TSTART."]
    PRSCH4,
    #[doc = "PRS ch 5 is controlling TSTART."]
    PRSCH5,
}
impl TSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTARTW::PRSCH0 => 0,
            TSTARTW::PRSCH1 => 1,
            TSTARTW::PRSCH2 => 2,
            TSTARTW::PRSCH3 => 3,
            TSTARTW::PRSCH4 => 4,
            TSTARTW::PRSCH5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTARTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS ch 0 is controlling TSTART."]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TSTARTW::PRSCH0)
    }
    #[doc = "PRS ch 1 is controlling TSTART."]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TSTARTW::PRSCH1)
    }
    #[doc = "PRS ch 2 is controlling TSTART."]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TSTARTW::PRSCH2)
    }
    #[doc = "PRS ch 3 is controlling TSTART."]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TSTARTW::PRSCH3)
    }
    #[doc = "PRS ch 4 is controlling TSTART."]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TSTARTW::PRSCH4)
    }
    #[doc = "PRS ch 5 is controlling TSTART."]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TSTARTW::PRSCH5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTOPENW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSTOP`"]
pub enum TSTOPW {
    #[doc = "PRS ch 0 is controlling TSTOP."]
    PRSCH0,
    #[doc = "PRS ch 1 is controlling TSTOP."]
    PRSCH1,
    #[doc = "PRS ch 2 is controlling TSTOP."]
    PRSCH2,
    #[doc = "PRS ch 3 is controlling TSTOP."]
    PRSCH3,
    #[doc = "PRS ch 4 is controlling TSTOP."]
    PRSCH4,
    #[doc = "PRS ch 5 is controlling TSTOP."]
    PRSCH5,
}
impl TSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTOPW::PRSCH0 => 0,
            TSTOPW::PRSCH1 => 1,
            TSTOPW::PRSCH2 => 2,
            TSTOPW::PRSCH3 => 3,
            TSTOPW::PRSCH4 => 4,
            TSTOPW::PRSCH5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTOPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS ch 0 is controlling TSTOP."]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TSTOPW::PRSCH0)
    }
    #[doc = "PRS ch 1 is controlling TSTOP."]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TSTOPW::PRSCH1)
    }
    #[doc = "PRS ch 2 is controlling TSTOP."]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TSTOPW::PRSCH2)
    }
    #[doc = "PRS ch 3 is controlling TSTOP."]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TSTOPW::PRSCH3)
    }
    #[doc = "PRS ch 4 is controlling TSTOP."]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TSTOPW::PRSCH4)
    }
    #[doc = "PRS ch 5 is controlling TSTOP."]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TSTOPW::PRSCH5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline]
    pub fn tstarten(&self) -> TSTARTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTARTENR { bits }
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS select"]
    #[inline]
    pub fn tstart(&self) -> TSTARTR {
        TSTARTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline]
    pub fn tstopen(&self) -> TSTOPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTOPENR { bits }
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS select"]
    #[inline]
    pub fn tstop(&self) -> TSTOPR {
        TSTOPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline]
    pub fn tstarten(&mut self) -> _TSTARTENW {
        _TSTARTENW { w: self }
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS select"]
    #[inline]
    pub fn tstart(&mut self) -> _TSTARTW {
        _TSTARTW { w: self }
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline]
    pub fn tstopen(&mut self) -> _TSTOPENW {
        _TSTOPENW { w: self }
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS select"]
    #[inline]
    pub fn tstop(&mut self) -> _TSTOPW {
        _TSTOPW { w: self }
    }
}
