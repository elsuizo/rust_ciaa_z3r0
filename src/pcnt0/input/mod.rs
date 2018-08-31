#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INPUT {
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
#[doc = "Possible values of the field `S0PRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0PRSSELR {
    #[doc = "PRS Channel 0 selected."]
    PRSCH0,
    #[doc = "PRS Channel 1 selected."]
    PRSCH1,
    #[doc = "PRS Channel 2 selected."]
    PRSCH2,
    #[doc = "PRS Channel 3 selected."]
    PRSCH3,
    #[doc = "PRS Channel 4 selected."]
    PRSCH4,
    #[doc = "PRS Channel 5 selected."]
    PRSCH5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl S0PRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S0PRSSELR::PRSCH0 => 0,
            S0PRSSELR::PRSCH1 => 1,
            S0PRSSELR::PRSCH2 => 2,
            S0PRSSELR::PRSCH3 => 3,
            S0PRSSELR::PRSCH4 => 4,
            S0PRSSELR::PRSCH5 => 5,
            S0PRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S0PRSSELR {
        match value {
            0 => S0PRSSELR::PRSCH0,
            1 => S0PRSSELR::PRSCH1,
            2 => S0PRSSELR::PRSCH2,
            3 => S0PRSSELR::PRSCH3,
            4 => S0PRSSELR::PRSCH4,
            5 => S0PRSSELR::PRSCH5,
            i => S0PRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == S0PRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == S0PRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == S0PRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == S0PRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == S0PRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == S0PRSSELR::PRSCH5
    }
}
#[doc = r" Value of the field"]
pub struct S0PRSENR {
    bits: bool,
}
impl S0PRSENR {
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
#[doc = "Possible values of the field `S1PRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1PRSSELR {
    #[doc = "PRS Channel 0 selected."]
    PRSCH0,
    #[doc = "PRS Channel 1 selected."]
    PRSCH1,
    #[doc = "PRS Channel 2 selected."]
    PRSCH2,
    #[doc = "PRS Channel 3 selected."]
    PRSCH3,
    #[doc = "PRS Channel 4 selected."]
    PRSCH4,
    #[doc = "PRS Channel 5 selected."]
    PRSCH5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl S1PRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S1PRSSELR::PRSCH0 => 0,
            S1PRSSELR::PRSCH1 => 1,
            S1PRSSELR::PRSCH2 => 2,
            S1PRSSELR::PRSCH3 => 3,
            S1PRSSELR::PRSCH4 => 4,
            S1PRSSELR::PRSCH5 => 5,
            S1PRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S1PRSSELR {
        match value {
            0 => S1PRSSELR::PRSCH0,
            1 => S1PRSSELR::PRSCH1,
            2 => S1PRSSELR::PRSCH2,
            3 => S1PRSSELR::PRSCH3,
            4 => S1PRSSELR::PRSCH4,
            5 => S1PRSSELR::PRSCH5,
            i => S1PRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == S1PRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == S1PRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == S1PRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == S1PRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == S1PRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == S1PRSSELR::PRSCH5
    }
}
#[doc = r" Value of the field"]
pub struct S1PRSENR {
    bits: bool,
}
impl S1PRSENR {
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
#[doc = "Values that can be written to the field `S0PRSSEL`"]
pub enum S0PRSSELW {
    #[doc = "PRS Channel 0 selected."]
    PRSCH0,
    #[doc = "PRS Channel 1 selected."]
    PRSCH1,
    #[doc = "PRS Channel 2 selected."]
    PRSCH2,
    #[doc = "PRS Channel 3 selected."]
    PRSCH3,
    #[doc = "PRS Channel 4 selected."]
    PRSCH4,
    #[doc = "PRS Channel 5 selected."]
    PRSCH5,
}
impl S0PRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S0PRSSELW::PRSCH0 => 0,
            S0PRSSELW::PRSCH1 => 1,
            S0PRSSELW::PRSCH2 => 2,
            S0PRSSELW::PRSCH3 => 3,
            S0PRSSELW::PRSCH4 => 4,
            S0PRSSELW::PRSCH5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0PRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S0PRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0PRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(S0PRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(S0PRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(S0PRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(S0PRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(S0PRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(S0PRSSELW::PRSCH5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S0PRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _S0PRSENW<'a> {
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
#[doc = "Values that can be written to the field `S1PRSSEL`"]
pub enum S1PRSSELW {
    #[doc = "PRS Channel 0 selected."]
    PRSCH0,
    #[doc = "PRS Channel 1 selected."]
    PRSCH1,
    #[doc = "PRS Channel 2 selected."]
    PRSCH2,
    #[doc = "PRS Channel 3 selected."]
    PRSCH3,
    #[doc = "PRS Channel 4 selected."]
    PRSCH4,
    #[doc = "PRS Channel 5 selected."]
    PRSCH5,
}
impl S1PRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S1PRSSELW::PRSCH0 => 0,
            S1PRSSELW::PRSCH1 => 1,
            S1PRSSELW::PRSCH2 => 2,
            S1PRSSELW::PRSCH3 => 3,
            S1PRSSELW::PRSCH4 => 4,
            S1PRSSELW::PRSCH5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1PRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S1PRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1PRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(S1PRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(S1PRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(S1PRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(S1PRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(S1PRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(S1PRSSELW::PRSCH5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S1PRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _S1PRSENW<'a> {
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:2 - S0IN PRS Channel Select"]
    #[inline]
    pub fn s0prssel(&self) -> S0PRSSELR {
        S0PRSSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - S0IN PRS Enable"]
    #[inline]
    pub fn s0prsen(&self) -> S0PRSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S0PRSENR { bits }
    }
    #[doc = "Bits 6:8 - S1IN PRS Channel Select"]
    #[inline]
    pub fn s1prssel(&self) -> S1PRSSELR {
        S1PRSSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - S1IN PRS Enable"]
    #[inline]
    pub fn s1prsen(&self) -> S1PRSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S1PRSENR { bits }
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
    #[doc = "Bits 0:2 - S0IN PRS Channel Select"]
    #[inline]
    pub fn s0prssel(&mut self) -> _S0PRSSELW {
        _S0PRSSELW { w: self }
    }
    #[doc = "Bit 4 - S0IN PRS Enable"]
    #[inline]
    pub fn s0prsen(&mut self) -> _S0PRSENW {
        _S0PRSENW { w: self }
    }
    #[doc = "Bits 6:8 - S1IN PRS Channel Select"]
    #[inline]
    pub fn s1prssel(&mut self) -> _S1PRSSELW {
        _S1PRSSELW { w: self }
    }
    #[doc = "Bit 10 - S1IN PRS Enable"]
    #[inline]
    pub fn s1prsen(&mut self) -> _S1PRSENW {
        _S1PRSENW { w: self }
    }
}
