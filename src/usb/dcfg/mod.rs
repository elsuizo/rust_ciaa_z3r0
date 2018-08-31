#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCFG {
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
#[doc = "Possible values of the field `DEVSPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVSPDR {
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    LS,
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    FS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEVSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEVSPDR::LS => 2,
            DEVSPDR::FS => 3,
            DEVSPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEVSPDR {
        match value {
            2 => DEVSPDR::LS,
            3 => DEVSPDR::FS,
            i => DEVSPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == DEVSPDR::LS
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline]
    pub fn is_fs(&self) -> bool {
        *self == DEVSPDR::FS
    }
}
#[doc = r" Value of the field"]
pub struct NZSTSOUTHSHKR {
    bits: bool,
}
impl NZSTSOUTHSHKR {
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
pub struct ENA32KHZSUSPR {
    bits: bool,
}
impl ENA32KHZSUSPR {
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
pub struct DEVADDRR {
    bits: u8,
}
impl DEVADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PERFRINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERFRINTR {
    #[doc = "80% of the frame interval."]
    _80PCNT,
    #[doc = "85% of the frame interval."]
    _85PCNT,
    #[doc = "90% of the frame interval."]
    _90PCNT,
    #[doc = "95% of the frame interval."]
    _95PCNT,
}
impl PERFRINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERFRINTR::_80PCNT => 0,
            PERFRINTR::_85PCNT => 1,
            PERFRINTR::_90PCNT => 2,
            PERFRINTR::_95PCNT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERFRINTR {
        match value {
            0 => PERFRINTR::_80PCNT,
            1 => PERFRINTR::_85PCNT,
            2 => PERFRINTR::_90PCNT,
            3 => PERFRINTR::_95PCNT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_80PCNT`"]
    #[inline]
    pub fn is_80pcnt(&self) -> bool {
        *self == PERFRINTR::_80PCNT
    }
    #[doc = "Checks if the value of the field is `_85PCNT`"]
    #[inline]
    pub fn is_85pcnt(&self) -> bool {
        *self == PERFRINTR::_85PCNT
    }
    #[doc = "Checks if the value of the field is `_90PCNT`"]
    #[inline]
    pub fn is_90pcnt(&self) -> bool {
        *self == PERFRINTR::_90PCNT
    }
    #[doc = "Checks if the value of the field is `_95PCNT`"]
    #[inline]
    pub fn is_95pcnt(&self) -> bool {
        *self == PERFRINTR::_95PCNT
    }
}
#[doc = r" Value of the field"]
pub struct ERRATICINTMSKR {
    bits: bool,
}
impl ERRATICINTMSKR {
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
pub struct RESVALIDR {
    bits: u8,
}
impl RESVALIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DEVSPD`"]
pub enum DEVSPDW {
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    LS,
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    FS,
}
impl DEVSPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEVSPDW::LS => 2,
            DEVSPDW::FS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEVSPDW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVSPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEVSPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    #[inline]
    pub fn ls(self) -> &'a mut W {
        self.variant(DEVSPDW::LS)
    }
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    #[inline]
    pub fn fs(self) -> &'a mut W {
        self.variant(DEVSPDW::FS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NZSTSOUTHSHKW<'a> {
    w: &'a mut W,
}
impl<'a> _NZSTSOUTHSHKW<'a> {
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
pub struct _ENA32KHZSUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _ENA32KHZSUSPW<'a> {
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
pub struct _DEVADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PERFRINT`"]
pub enum PERFRINTW {
    #[doc = "80% of the frame interval."]
    _80PCNT,
    #[doc = "85% of the frame interval."]
    _85PCNT,
    #[doc = "90% of the frame interval."]
    _90PCNT,
    #[doc = "95% of the frame interval."]
    _95PCNT,
}
impl PERFRINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERFRINTW::_80PCNT => 0,
            PERFRINTW::_85PCNT => 1,
            PERFRINTW::_90PCNT => 2,
            PERFRINTW::_95PCNT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERFRINTW<'a> {
    w: &'a mut W,
}
impl<'a> _PERFRINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERFRINTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "80% of the frame interval."]
    #[inline]
    pub fn _80pcnt(self) -> &'a mut W {
        self.variant(PERFRINTW::_80PCNT)
    }
    #[doc = "85% of the frame interval."]
    #[inline]
    pub fn _85pcnt(self) -> &'a mut W {
        self.variant(PERFRINTW::_85PCNT)
    }
    #[doc = "90% of the frame interval."]
    #[inline]
    pub fn _90pcnt(self) -> &'a mut W {
        self.variant(PERFRINTW::_90PCNT)
    }
    #[doc = "95% of the frame interval."]
    #[inline]
    pub fn _95pcnt(self) -> &'a mut W {
        self.variant(PERFRINTW::_95PCNT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ERRATICINTMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRATICINTMSKW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _RESVALIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline]
    pub fn devspd(&self) -> DEVSPDR {
        DEVSPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline]
    pub fn nzstsouthshk(&self) -> NZSTSOUTHSHKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NZSTSOUTHSHKR { bits }
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline]
    pub fn ena32khzsusp(&self) -> ENA32KHZSUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA32KHZSUSPR { bits }
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline]
    pub fn devaddr(&self) -> DEVADDRR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEVADDRR { bits }
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline]
    pub fn perfrint(&self) -> PERFRINTR {
        PERFRINTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - \"\""]
    #[inline]
    pub fn erraticintmsk(&self) -> ERRATICINTMSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRATICINTMSKR { bits }
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline]
    pub fn resvalid(&self) -> RESVALIDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESVALIDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 134217728 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline]
    pub fn devspd(&mut self) -> _DEVSPDW {
        _DEVSPDW { w: self }
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline]
    pub fn nzstsouthshk(&mut self) -> _NZSTSOUTHSHKW {
        _NZSTSOUTHSHKW { w: self }
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline]
    pub fn ena32khzsusp(&mut self) -> _ENA32KHZSUSPW {
        _ENA32KHZSUSPW { w: self }
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline]
    pub fn devaddr(&mut self) -> _DEVADDRW {
        _DEVADDRW { w: self }
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline]
    pub fn perfrint(&mut self) -> _PERFRINTW {
        _PERFRINTW { w: self }
    }
    #[doc = "Bit 15 - \"\""]
    #[inline]
    pub fn erraticintmsk(&mut self) -> _ERRATICINTMSKW {
        _ERRATICINTMSKW { w: self }
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline]
    pub fn resvalid(&mut self) -> _RESVALIDW {
        _RESVALIDW { w: self }
    }
}
