#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DTFC {
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
#[doc = "Possible values of the field `DTPRS0FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRS0FSELR {
    #[doc = "PRS Channel 0 selected as fault source 0"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as fault source 0"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as fault source 0"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as fault source 0"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as fault source 0"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as fault source 0"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as fault source 0"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as fault source 0"]
    PRSCH7,
}
impl DTPRS0FSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTPRS0FSELR::PRSCH0 => 0,
            DTPRS0FSELR::PRSCH1 => 1,
            DTPRS0FSELR::PRSCH2 => 2,
            DTPRS0FSELR::PRSCH3 => 3,
            DTPRS0FSELR::PRSCH4 => 4,
            DTPRS0FSELR::PRSCH5 => 5,
            DTPRS0FSELR::PRSCH6 => 6,
            DTPRS0FSELR::PRSCH7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTPRS0FSELR {
        match value {
            0 => DTPRS0FSELR::PRSCH0,
            1 => DTPRS0FSELR::PRSCH1,
            2 => DTPRS0FSELR::PRSCH2,
            3 => DTPRS0FSELR::PRSCH3,
            4 => DTPRS0FSELR::PRSCH4,
            5 => DTPRS0FSELR::PRSCH5,
            6 => DTPRS0FSELR::PRSCH6,
            7 => DTPRS0FSELR::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS0FSELR::PRSCH7
    }
}
#[doc = "Possible values of the field `DTPRS1FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRS1FSELR {
    #[doc = "PRS Channel 0 selected as fault source 1"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as fault source 1"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as fault source 1"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as fault source 1"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as fault source 1"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as fault source 1"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as fault source 1"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as fault source 1"]
    PRSCH7,
}
impl DTPRS1FSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTPRS1FSELR::PRSCH0 => 0,
            DTPRS1FSELR::PRSCH1 => 1,
            DTPRS1FSELR::PRSCH2 => 2,
            DTPRS1FSELR::PRSCH3 => 3,
            DTPRS1FSELR::PRSCH4 => 4,
            DTPRS1FSELR::PRSCH5 => 5,
            DTPRS1FSELR::PRSCH6 => 6,
            DTPRS1FSELR::PRSCH7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTPRS1FSELR {
        match value {
            0 => DTPRS1FSELR::PRSCH0,
            1 => DTPRS1FSELR::PRSCH1,
            2 => DTPRS1FSELR::PRSCH2,
            3 => DTPRS1FSELR::PRSCH3,
            4 => DTPRS1FSELR::PRSCH4,
            5 => DTPRS1FSELR::PRSCH5,
            6 => DTPRS1FSELR::PRSCH6,
            7 => DTPRS1FSELR::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS1FSELR::PRSCH7
    }
}
#[doc = "Possible values of the field `DTFA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTFAR {
    #[doc = "No action on fault"]
    NONE,
    #[doc = "Set outputs inactive"]
    INACTIVE,
    #[doc = "Clear outputs"]
    CLEAR,
    #[doc = "Tristate outputs"]
    TRISTATE,
}
impl DTFAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTFAR::NONE => 0,
            DTFAR::INACTIVE => 1,
            DTFAR::CLEAR => 2,
            DTFAR::TRISTATE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTFAR {
        match value {
            0 => DTFAR::NONE,
            1 => DTFAR::INACTIVE,
            2 => DTFAR::CLEAR,
            3 => DTFAR::TRISTATE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == DTFAR::NONE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == DTFAR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == DTFAR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline]
    pub fn is_tristate(&self) -> bool {
        *self == DTFAR::TRISTATE
    }
}
#[doc = r" Value of the field"]
pub struct DTPRS0FENR {
    bits: bool,
}
impl DTPRS0FENR {
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
pub struct DTPRS1FENR {
    bits: bool,
}
impl DTPRS1FENR {
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
pub struct DTDBGFENR {
    bits: bool,
}
impl DTDBGFENR {
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
pub struct DTLOCKUPFENR {
    bits: bool,
}
impl DTLOCKUPFENR {
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
#[doc = "Values that can be written to the field `DTPRS0FSEL`"]
pub enum DTPRS0FSELW {
    #[doc = "PRS Channel 0 selected as fault source 0"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as fault source 0"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as fault source 0"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as fault source 0"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as fault source 0"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as fault source 0"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as fault source 0"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as fault source 0"]
    PRSCH7,
}
impl DTPRS0FSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTPRS0FSELW::PRSCH0 => 0,
            DTPRS0FSELW::PRSCH1 => 1,
            DTPRS0FSELW::PRSCH2 => 2,
            DTPRS0FSELW::PRSCH3 => 3,
            DTPRS0FSELW::PRSCH4 => 4,
            DTPRS0FSELW::PRSCH5 => 5,
            DTPRS0FSELW::PRSCH6 => 6,
            DTPRS0FSELW::PRSCH7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTPRS0FSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRS0FSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTPRS0FSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PRS Channel 0 selected as fault source 0"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 0"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 0"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 0"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 0"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 0"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 0"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 0"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS0FSELW::PRSCH7)
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
#[doc = "Values that can be written to the field `DTPRS1FSEL`"]
pub enum DTPRS1FSELW {
    #[doc = "PRS Channel 0 selected as fault source 1"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as fault source 1"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as fault source 1"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as fault source 1"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as fault source 1"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as fault source 1"]
    PRSCH5,
    #[doc = "PRS Channel 6 selected as fault source 1"]
    PRSCH6,
    #[doc = "PRS Channel 7 selected as fault source 1"]
    PRSCH7,
}
impl DTPRS1FSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTPRS1FSELW::PRSCH0 => 0,
            DTPRS1FSELW::PRSCH1 => 1,
            DTPRS1FSELW::PRSCH2 => 2,
            DTPRS1FSELW::PRSCH3 => 3,
            DTPRS1FSELW::PRSCH4 => 4,
            DTPRS1FSELW::PRSCH5 => 5,
            DTPRS1FSELW::PRSCH6 => 6,
            DTPRS1FSELW::PRSCH7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTPRS1FSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRS1FSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTPRS1FSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PRS Channel 0 selected as fault source 1"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 1"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 1"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 1"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 1"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 1"]
    #[inline]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 1"]
    #[inline]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS1FSELW::PRSCH7)
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
#[doc = "Values that can be written to the field `DTFA`"]
pub enum DTFAW {
    #[doc = "No action on fault"]
    NONE,
    #[doc = "Set outputs inactive"]
    INACTIVE,
    #[doc = "Clear outputs"]
    CLEAR,
    #[doc = "Tristate outputs"]
    TRISTATE,
}
impl DTFAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTFAW::NONE => 0,
            DTFAW::INACTIVE => 1,
            DTFAW::CLEAR => 2,
            DTFAW::TRISTATE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTFAW<'a> {
    w: &'a mut W,
}
impl<'a> _DTFAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTFAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action on fault"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(DTFAW::NONE)
    }
    #[doc = "Set outputs inactive"]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DTFAW::INACTIVE)
    }
    #[doc = "Clear outputs"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(DTFAW::CLEAR)
    }
    #[doc = "Tristate outputs"]
    #[inline]
    pub fn tristate(self) -> &'a mut W {
        self.variant(DTFAW::TRISTATE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTPRS0FENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRS0FENW<'a> {
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
#[doc = r" Proxy"]
pub struct _DTPRS1FENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRS1FENW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTDBGFENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTDBGFENW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTLOCKUPFENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTLOCKUPFENW<'a> {
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:2 - DTI PRS Fault Source 0 Select"]
    #[inline]
    pub fn dtprs0fsel(&self) -> DTPRS0FSELR {
        DTPRS0FSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - DTI PRS Fault Source 1 Select"]
    #[inline]
    pub fn dtprs1fsel(&self) -> DTPRS1FSELR {
        DTPRS1FSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline]
    pub fn dtfa(&self) -> DTFAR {
        DTFAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline]
    pub fn dtprs0fen(&self) -> DTPRS0FENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTPRS0FENR { bits }
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline]
    pub fn dtprs1fen(&self) -> DTPRS1FENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTPRS1FENR { bits }
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline]
    pub fn dtdbgfen(&self) -> DTDBGFENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTDBGFENR { bits }
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline]
    pub fn dtlockupfen(&self) -> DTLOCKUPFENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTLOCKUPFENR { bits }
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
    #[doc = "Bits 0:2 - DTI PRS Fault Source 0 Select"]
    #[inline]
    pub fn dtprs0fsel(&mut self) -> _DTPRS0FSELW {
        _DTPRS0FSELW { w: self }
    }
    #[doc = "Bits 8:10 - DTI PRS Fault Source 1 Select"]
    #[inline]
    pub fn dtprs1fsel(&mut self) -> _DTPRS1FSELW {
        _DTPRS1FSELW { w: self }
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline]
    pub fn dtfa(&mut self) -> _DTFAW {
        _DTFAW { w: self }
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline]
    pub fn dtprs0fen(&mut self) -> _DTPRS0FENW {
        _DTPRS0FENW { w: self }
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline]
    pub fn dtprs1fen(&mut self) -> _DTPRS1FENW {
        _DTPRS1FENW { w: self }
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline]
    pub fn dtdbgfen(&mut self) -> _DTDBGFENW {
        _DTDBGFENW { w: self }
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline]
    pub fn dtlockupfen(&mut self) -> _DTLOCKUPFENW {
        _DTLOCKUPFENW { w: self }
    }
}
