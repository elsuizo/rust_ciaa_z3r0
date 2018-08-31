#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFCLKSEL {
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
#[doc = "Possible values of the field `LFA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFAR {
    #[doc = "LFACLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFACLK"]
    LFRCO,
    #[doc = "LFXO selected as LFACLK"]
    LFXO,
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2,
}
impl LFAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFAR::DISABLED => 0,
            LFAR::LFRCO => 1,
            LFAR::LFXO => 2,
            LFAR::HFCORECLKLEDIV2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFAR {
        match value {
            0 => LFAR::DISABLED,
            1 => LFAR::LFRCO,
            2 => LFAR::LFXO,
            3 => LFAR::HFCORECLKLEDIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LFAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == LFAR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == LFAR::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCORECLKLEDIV2`"]
    #[inline]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == LFAR::HFCORECLKLEDIV2
    }
}
#[doc = "Possible values of the field `LFB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFBR {
    #[doc = "LFBCLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFBCLK"]
    LFRCO,
    #[doc = "LFXO selected as LFBCLK"]
    LFXO,
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2,
}
impl LFBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFBR::DISABLED => 0,
            LFBR::LFRCO => 1,
            LFBR::LFXO => 2,
            LFBR::HFCORECLKLEDIV2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFBR {
        match value {
            0 => LFBR::DISABLED,
            1 => LFBR::LFRCO,
            2 => LFBR::LFXO,
            3 => LFBR::HFCORECLKLEDIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LFBR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == LFBR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == LFBR::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCORECLKLEDIV2`"]
    #[inline]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == LFBR::HFCORECLKLEDIV2
    }
}
#[doc = "Possible values of the field `LFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCR {
    #[doc = "LFCCLK clock disabled."]
    DISABLED,
    #[doc = "LFRCO selected as LFCCLK clock"]
    LFRCO,
    #[doc = "LFXO selected as LFCCLK clock"]
    LFXO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFCR::DISABLED => 0,
            LFCR::LFRCO => 1,
            LFCR::LFXO => 2,
            LFCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFCR {
        match value {
            0 => LFCR::DISABLED,
            1 => LFCR::LFRCO,
            2 => LFCR::LFXO,
            i => LFCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LFCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == LFCR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == LFCR::LFXO
    }
}
#[doc = r" Value of the field"]
pub struct LFAER {
    bits: bool,
}
impl LFAER {
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
pub struct LFBER {
    bits: bool,
}
impl LFBER {
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
#[doc = "Values that can be written to the field `LFA`"]
pub enum LFAW {
    #[doc = "LFACLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFACLK"]
    LFRCO,
    #[doc = "LFXO selected as LFACLK"]
    LFXO,
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2,
}
impl LFAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFAW::DISABLED => 0,
            LFAW::LFRCO => 1,
            LFAW::LFXO => 2,
            LFAW::HFCORECLKLEDIV2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFAW<'a> {
    w: &'a mut W,
}
impl<'a> _LFAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFACLK is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFAW::DISABLED)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFAW::LFRCO)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFAW::LFXO)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline]
    pub fn hfcoreclklediv2(self) -> &'a mut W {
        self.variant(LFAW::HFCORECLKLEDIV2)
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
#[doc = "Values that can be written to the field `LFB`"]
pub enum LFBW {
    #[doc = "LFBCLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFBCLK"]
    LFRCO,
    #[doc = "LFXO selected as LFBCLK"]
    LFXO,
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    HFCORECLKLEDIV2,
}
impl LFBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFBW::DISABLED => 0,
            LFBW::LFRCO => 1,
            LFBW::LFXO => 2,
            LFBW::HFCORECLKLEDIV2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFBW<'a> {
    w: &'a mut W,
}
impl<'a> _LFBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFBCLK is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFBW::DISABLED)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFBW::LFRCO)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFBW::LFXO)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline]
    pub fn hfcoreclklediv2(self) -> &'a mut W {
        self.variant(LFBW::HFCORECLKLEDIV2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LFC`"]
pub enum LFCW {
    #[doc = "LFCCLK clock disabled."]
    DISABLED,
    #[doc = "LFRCO selected as LFCCLK clock"]
    LFRCO,
    #[doc = "LFXO selected as LFCCLK clock"]
    LFXO,
}
impl LFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFCW::DISABLED => 0,
            LFCW::LFRCO => 1,
            LFCW::LFXO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFCW<'a> {
    w: &'a mut W,
}
impl<'a> _LFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFCCLK clock disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFCW::DISABLED)
    }
    #[doc = "LFRCO selected as LFCCLK clock"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFCW::LFRCO)
    }
    #[doc = "LFXO selected as LFCCLK clock"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFCW::LFXO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LFAEW<'a> {
    w: &'a mut W,
}
impl<'a> _LFAEW<'a> {
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
pub struct _LFBEW<'a> {
    w: &'a mut W,
}
impl<'a> _LFBEW<'a> {
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline]
    pub fn lfa(&self) -> LFAR {
        LFAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline]
    pub fn lfb(&self) -> LFBR {
        LFBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Clock Select for LFC"]
    #[inline]
    pub fn lfc(&self) -> LFCR {
        LFCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline]
    pub fn lfae(&self) -> LFAER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LFAER { bits }
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline]
    pub fn lfbe(&self) -> LFBER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LFBER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 21 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline]
    pub fn lfa(&mut self) -> _LFAW {
        _LFAW { w: self }
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline]
    pub fn lfb(&mut self) -> _LFBW {
        _LFBW { w: self }
    }
    #[doc = "Bits 4:5 - Clock Select for LFC"]
    #[inline]
    pub fn lfc(&mut self) -> _LFCW {
        _LFCW { w: self }
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline]
    pub fn lfae(&mut self) -> _LFAEW {
        _LFAEW { w: self }
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline]
    pub fn lfbe(&mut self) -> _LFBEW {
        _LFBEW { w: self }
    }
}
