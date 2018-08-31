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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "The module is disabled."]
    DISABLE,
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    OVSSINGLE,
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    EXTCLKSINGLE,
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    EXTCLKQUAD,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::DISABLE => 0,
            MODER::OVSSINGLE => 1,
            MODER::EXTCLKSINGLE => 2,
            MODER::EXTCLKQUAD => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::DISABLE,
            1 => MODER::OVSSINGLE,
            2 => MODER::EXTCLKSINGLE,
            3 => MODER::EXTCLKQUAD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `OVSSINGLE`"]
    #[inline]
    pub fn is_ovssingle(&self) -> bool {
        *self == MODER::OVSSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKSINGLE`"]
    #[inline]
    pub fn is_extclksingle(&self) -> bool {
        *self == MODER::EXTCLKSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKQUAD`"]
    #[inline]
    pub fn is_extclkquad(&self) -> bool {
        *self == MODER::EXTCLKQUAD
    }
}
#[doc = r" Value of the field"]
pub struct CNTDIRR {
    bits: bool,
}
impl CNTDIRR {
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
pub struct EDGER {
    bits: bool,
}
impl EDGER {
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
pub struct FILTR {
    bits: bool,
}
impl FILTR {
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
pub struct RSTENR {
    bits: bool,
}
impl RSTENR {
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
pub struct AUXCNTRSTENR {
    bits: bool,
}
impl AUXCNTRSTENR {
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
pub struct HYSTR {
    bits: bool,
}
impl HYSTR {
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
pub struct S1CDIRR {
    bits: bool,
}
impl S1CDIRR {
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
#[doc = "Possible values of the field `CNTEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTEVR {
    #[doc = "Counts up on up-count and down on down-count events."]
    BOTH,
    #[doc = "Only counts up on up-count events."]
    UP,
    #[doc = "Only counts down on down-count events."]
    DOWN,
    #[doc = "Never counts."]
    NONE,
}
impl CNTEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CNTEVR::BOTH => 0,
            CNTEVR::UP => 1,
            CNTEVR::DOWN => 2,
            CNTEVR::NONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CNTEVR {
        match value {
            0 => CNTEVR::BOTH,
            1 => CNTEVR::UP,
            2 => CNTEVR::DOWN,
            3 => CNTEVR::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == CNTEVR::BOTH
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == CNTEVR::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline]
    pub fn is_down(&self) -> bool {
        *self == CNTEVR::DOWN
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == CNTEVR::NONE
    }
}
#[doc = "Possible values of the field `AUXCNTEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUXCNTEVR {
    #[doc = "Never counts."]
    NONE,
    #[doc = "Counts up on up-count events."]
    UP,
    #[doc = "Counts up on down-count events."]
    DOWN,
    #[doc = "Counts up on both up-count and down-count events."]
    BOTH,
}
impl AUXCNTEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AUXCNTEVR::NONE => 0,
            AUXCNTEVR::UP => 1,
            AUXCNTEVR::DOWN => 2,
            AUXCNTEVR::BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AUXCNTEVR {
        match value {
            0 => AUXCNTEVR::NONE,
            1 => AUXCNTEVR::UP,
            2 => AUXCNTEVR::DOWN,
            3 => AUXCNTEVR::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == AUXCNTEVR::NONE
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == AUXCNTEVR::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline]
    pub fn is_down(&self) -> bool {
        *self == AUXCNTEVR::DOWN
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == AUXCNTEVR::BOTH
    }
}
#[doc = "Possible values of the field `TCCMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCMODER {
    #[doc = "Triggered compare and clear not enabled."]
    DISABLED,
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    LFA,
    #[doc = "Compare and clear performed on positive PRS edges."]
    PRS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TCCMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCCMODER::DISABLED => 0,
            TCCMODER::LFA => 1,
            TCCMODER::PRS => 2,
            TCCMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCCMODER {
        match value {
            0 => TCCMODER::DISABLED,
            1 => TCCMODER::LFA,
            2 => TCCMODER::PRS,
            i => TCCMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TCCMODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFA`"]
    #[inline]
    pub fn is_lfa(&self) -> bool {
        *self == TCCMODER::LFA
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline]
    pub fn is_prs(&self) -> bool {
        *self == TCCMODER::PRS
    }
}
#[doc = "Possible values of the field `TCCPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCPRESCR {
    #[doc = "Compare and clear event each LFA cycle."]
    DIV1,
    #[doc = "Compare and clear performed on every other LFA cycle."]
    DIV2,
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    DIV4,
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    DIV8,
}
impl TCCPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCCPRESCR::DIV1 => 0,
            TCCPRESCR::DIV2 => 1,
            TCCPRESCR::DIV4 => 2,
            TCCPRESCR::DIV8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCCPRESCR {
        match value {
            0 => TCCPRESCR::DIV1,
            1 => TCCPRESCR::DIV2,
            2 => TCCPRESCR::DIV4,
            3 => TCCPRESCR::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == TCCPRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == TCCPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == TCCPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == TCCPRESCR::DIV8
    }
}
#[doc = "Possible values of the field `TCCCOMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCCOMPR {
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    LTOE,
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    GTOE,
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP[15:8]], and greater than, or equal to PCNT_TOP[7:0]."]
    RANGE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TCCCOMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCCCOMPR::LTOE => 0,
            TCCCOMPR::GTOE => 1,
            TCCCOMPR::RANGE => 2,
            TCCCOMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCCCOMPR {
        match value {
            0 => TCCCOMPR::LTOE,
            1 => TCCCOMPR::GTOE,
            2 => TCCCOMPR::RANGE,
            i => TCCCOMPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LTOE`"]
    #[inline]
    pub fn is_ltoe(&self) -> bool {
        *self == TCCCOMPR::LTOE
    }
    #[doc = "Checks if the value of the field is `GTOE`"]
    #[inline]
    pub fn is_gtoe(&self) -> bool {
        *self == TCCCOMPR::GTOE
    }
    #[doc = "Checks if the value of the field is `RANGE`"]
    #[inline]
    pub fn is_range(&self) -> bool {
        *self == TCCCOMPR::RANGE
    }
}
#[doc = r" Value of the field"]
pub struct PRSGATEENR {
    bits: bool,
}
impl PRSGATEENR {
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
pub struct TCCPRSPOLR {
    bits: bool,
}
impl TCCPRSPOLR {
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
#[doc = "Possible values of the field `TCCPRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCPRSSELR {
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
impl TCCPRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCCPRSSELR::PRSCH0 => 0,
            TCCPRSSELR::PRSCH1 => 1,
            TCCPRSSELR::PRSCH2 => 2,
            TCCPRSSELR::PRSCH3 => 3,
            TCCPRSSELR::PRSCH4 => 4,
            TCCPRSSELR::PRSCH5 => 5,
            TCCPRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCCPRSSELR {
        match value {
            0 => TCCPRSSELR::PRSCH0,
            1 => TCCPRSSELR::PRSCH1,
            2 => TCCPRSSELR::PRSCH2,
            3 => TCCPRSSELR::PRSCH3,
            4 => TCCPRSSELR::PRSCH4,
            5 => TCCPRSSELR::PRSCH5,
            i => TCCPRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == TCCPRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == TCCPRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == TCCPRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == TCCPRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == TCCPRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == TCCPRSSELR::PRSCH5
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "The module is disabled."]
    DISABLE,
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    OVSSINGLE,
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    EXTCLKSINGLE,
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    EXTCLKQUAD,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::DISABLE => 0,
            MODEW::OVSSINGLE => 1,
            MODEW::EXTCLKSINGLE => 2,
            MODEW::EXTCLKQUAD => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The module is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MODEW::DISABLE)
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    #[inline]
    pub fn ovssingle(self) -> &'a mut W {
        self.variant(MODEW::OVSSINGLE)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline]
    pub fn extclksingle(self) -> &'a mut W {
        self.variant(MODEW::EXTCLKSINGLE)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline]
    pub fn extclkquad(self) -> &'a mut W {
        self.variant(MODEW::EXTCLKQUAD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CNTDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTDIRW<'a> {
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
pub struct _EDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGEW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTW<'a> {
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
#[doc = r" Proxy"]
pub struct _RSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTENW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUXCNTRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _AUXCNTRSTENW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _S1CDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _S1CDIRW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNTEV`"]
pub enum CNTEVW {
    #[doc = "Counts up on up-count and down on down-count events."]
    BOTH,
    #[doc = "Only counts up on up-count events."]
    UP,
    #[doc = "Only counts down on down-count events."]
    DOWN,
    #[doc = "Never counts."]
    NONE,
}
impl CNTEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CNTEVW::BOTH => 0,
            CNTEVW::UP => 1,
            CNTEVW::DOWN => 2,
            CNTEVW::NONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTEVW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTEVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(CNTEVW::BOTH)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(CNTEVW::UP)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline]
    pub fn down(self) -> &'a mut W {
        self.variant(CNTEVW::DOWN)
    }
    #[doc = "Never counts."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(CNTEVW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUXCNTEV`"]
pub enum AUXCNTEVW {
    #[doc = "Never counts."]
    NONE,
    #[doc = "Counts up on up-count events."]
    UP,
    #[doc = "Counts up on down-count events."]
    DOWN,
    #[doc = "Counts up on both up-count and down-count events."]
    BOTH,
}
impl AUXCNTEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AUXCNTEVW::NONE => 0,
            AUXCNTEVW::UP => 1,
            AUXCNTEVW::DOWN => 2,
            AUXCNTEVW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUXCNTEVW<'a> {
    w: &'a mut W,
}
impl<'a> _AUXCNTEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUXCNTEVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Never counts."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(AUXCNTEVW::NONE)
    }
    #[doc = "Counts up on up-count events."]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(AUXCNTEVW::UP)
    }
    #[doc = "Counts up on down-count events."]
    #[inline]
    pub fn down(self) -> &'a mut W {
        self.variant(AUXCNTEVW::DOWN)
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(AUXCNTEVW::BOTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCCMODE`"]
pub enum TCCMODEW {
    #[doc = "Triggered compare and clear not enabled."]
    DISABLED,
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    LFA,
    #[doc = "Compare and clear performed on positive PRS edges."]
    PRS,
}
impl TCCMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCCMODEW::DISABLED => 0,
            TCCMODEW::LFA => 1,
            TCCMODEW::PRS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCCMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCCMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Triggered compare and clear not enabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCCMODEW::DISABLED)
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline]
    pub fn lfa(self) -> &'a mut W {
        self.variant(TCCMODEW::LFA)
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline]
    pub fn prs(self) -> &'a mut W {
        self.variant(TCCMODEW::PRS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCCPRESC`"]
pub enum TCCPRESCW {
    #[doc = "Compare and clear event each LFA cycle."]
    DIV1,
    #[doc = "Compare and clear performed on every other LFA cycle."]
    DIV2,
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    DIV4,
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    DIV8,
}
impl TCCPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCCPRESCW::DIV1 => 0,
            TCCPRESCW::DIV2 => 1,
            TCCPRESCW::DIV4 => 2,
            TCCPRESCW::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCCPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCCPRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(TCCPRESCW::DIV1)
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(TCCPRESCW::DIV2)
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(TCCPRESCW::DIV4)
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(TCCPRESCW::DIV8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCCCOMP`"]
pub enum TCCCOMPW {
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    LTOE,
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    GTOE,
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP[15:8]], and greater than, or equal to PCNT_TOP[7:0]."]
    RANGE,
}
impl TCCCOMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCCCOMPW::LTOE => 0,
            TCCCOMPW::GTOE => 1,
            TCCCOMPW::RANGE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCCCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCCOMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCCCOMPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline]
    pub fn ltoe(self) -> &'a mut W {
        self.variant(TCCCOMPW::LTOE)
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline]
    pub fn gtoe(self) -> &'a mut W {
        self.variant(TCCCOMPW::GTOE)
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP[15:8]], and greater than, or equal to PCNT_TOP[7:0]."]
    #[inline]
    pub fn range(self) -> &'a mut W {
        self.variant(TCCCOMPW::RANGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRSGATEENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSGATEENW<'a> {
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
#[doc = r" Proxy"]
pub struct _TCCPRSPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCPRSPOLW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCCPRSSEL`"]
pub enum TCCPRSSELW {
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
impl TCCPRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCCPRSSELW::PRSCH0 => 0,
            TCCPRSSELW::PRSCH1 => 1,
            TCCPRSSELW::PRSCH2 => 2,
            TCCPRSSELW::PRSCH3 => 3,
            TCCPRSSELW::PRSCH4 => 4,
            TCCPRSSELW::PRSCH5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCCPRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCPRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCCPRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TCCPRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TCCPRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TCCPRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TCCPRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TCCPRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TCCPRSSELW::PRSCH5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:1 - Mode Select"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Non-Quadrature Mode Counter Direction Control"]
    #[inline]
    pub fn cntdir(&self) -> CNTDIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CNTDIRR { bits }
    }
    #[doc = "Bit 3 - Edge Select"]
    #[inline]
    pub fn edge(&self) -> EDGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EDGER { bits }
    }
    #[doc = "Bit 4 - Enable Digital Pulse Width Filter"]
    #[inline]
    pub fn filt(&self) -> FILTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTR { bits }
    }
    #[doc = "Bit 5 - Enable PCNT Clock Domain Reset"]
    #[inline]
    pub fn rsten(&self) -> RSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSTENR { bits }
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline]
    pub fn auxcntrsten(&self) -> AUXCNTRSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUXCNTRSTENR { bits }
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYSTR { bits }
    }
    #[doc = "Bit 9 - Count direction determined by S1"]
    #[inline]
    pub fn s1cdir(&self) -> S1CDIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S1CDIRR { bits }
    }
    #[doc = "Bits 10:11 - Controls when the counter counts"]
    #[inline]
    pub fn cntev(&self) -> CNTEVR {
        CNTEVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Controls when the auxiliary counter counts"]
    #[inline]
    pub fn auxcntev(&self) -> AUXCNTEVR {
        AUXCNTEVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Sets the mode for triggered compare and clear"]
    #[inline]
    pub fn tccmode(&self) -> TCCMODER {
        TCCMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Set the LFA prescaler for triggered compare and clear"]
    #[inline]
    pub fn tccpresc(&self) -> TCCPRESCR {
        TCCPRESCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 25:26 - Triggered compare and clear compare mode"]
    #[inline]
    pub fn tcccomp(&self) -> TCCCOMPR {
        TCCCOMPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - PRS gate enable"]
    #[inline]
    pub fn prsgateen(&self) -> PRSGATEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSGATEENR { bits }
    }
    #[doc = "Bit 28 - TCC PRS polarity select"]
    #[inline]
    pub fn tccprspol(&self) -> TCCPRSPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCCPRSPOLR { bits }
    }
    #[doc = "Bits 29:31 - TCC PRS Channel Select"]
    #[inline]
    pub fn tccprssel(&self) -> TCCPRSSELR {
        TCCPRSSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:1 - Mode Select"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 2 - Non-Quadrature Mode Counter Direction Control"]
    #[inline]
    pub fn cntdir(&mut self) -> _CNTDIRW {
        _CNTDIRW { w: self }
    }
    #[doc = "Bit 3 - Edge Select"]
    #[inline]
    pub fn edge(&mut self) -> _EDGEW {
        _EDGEW { w: self }
    }
    #[doc = "Bit 4 - Enable Digital Pulse Width Filter"]
    #[inline]
    pub fn filt(&mut self) -> _FILTW {
        _FILTW { w: self }
    }
    #[doc = "Bit 5 - Enable PCNT Clock Domain Reset"]
    #[inline]
    pub fn rsten(&mut self) -> _RSTENW {
        _RSTENW { w: self }
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline]
    pub fn auxcntrsten(&mut self) -> _AUXCNTRSTENW {
        _AUXCNTRSTENW { w: self }
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
    #[doc = "Bit 9 - Count direction determined by S1"]
    #[inline]
    pub fn s1cdir(&mut self) -> _S1CDIRW {
        _S1CDIRW { w: self }
    }
    #[doc = "Bits 10:11 - Controls when the counter counts"]
    #[inline]
    pub fn cntev(&mut self) -> _CNTEVW {
        _CNTEVW { w: self }
    }
    #[doc = "Bits 14:15 - Controls when the auxiliary counter counts"]
    #[inline]
    pub fn auxcntev(&mut self) -> _AUXCNTEVW {
        _AUXCNTEVW { w: self }
    }
    #[doc = "Bits 18:19 - Sets the mode for triggered compare and clear"]
    #[inline]
    pub fn tccmode(&mut self) -> _TCCMODEW {
        _TCCMODEW { w: self }
    }
    #[doc = "Bits 22:23 - Set the LFA prescaler for triggered compare and clear"]
    #[inline]
    pub fn tccpresc(&mut self) -> _TCCPRESCW {
        _TCCPRESCW { w: self }
    }
    #[doc = "Bits 25:26 - Triggered compare and clear compare mode"]
    #[inline]
    pub fn tcccomp(&mut self) -> _TCCCOMPW {
        _TCCCOMPW { w: self }
    }
    #[doc = "Bit 27 - PRS gate enable"]
    #[inline]
    pub fn prsgateen(&mut self) -> _PRSGATEENW {
        _PRSGATEENW { w: self }
    }
    #[doc = "Bit 28 - TCC PRS polarity select"]
    #[inline]
    pub fn tccprspol(&mut self) -> _TCCPRSPOLW {
        _TCCPRSPOLW { w: self }
    }
    #[doc = "Bits 29:31 - TCC PRS Channel Select"]
    #[inline]
    pub fn tccprssel(&mut self) -> _TCCPRSSELW {
        _TCCPRSSELW { w: self }
    }
}
