#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMD {
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
}
#[doc = "Values that can be written to the field `HFCLKSEL`"]
pub enum HFCLKSELW {
    #[doc = "Select HFRCO as HFCLK."]
    HFRCO,
    #[doc = "Select HFXO as HFCLK."]
    HFXO,
    #[doc = "Select LFRCO as HFCLK."]
    LFRCO,
    #[doc = "Select LFXO as HFCLK."]
    LFXO,
    #[doc = "Select USHFRCO divided by two as HFCLK."]
    USHFRCODIV2,
}
impl HFCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFCLKSELW::HFRCO => 1,
            HFCLKSELW::HFXO => 2,
            HFCLKSELW::LFRCO => 3,
            HFCLKSELW::LFXO => 4,
            HFCLKSELW::USHFRCODIV2 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _HFCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFCLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select HFRCO as HFCLK."]
    #[inline]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(HFCLKSELW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK."]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(HFCLKSELW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK."]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(HFCLKSELW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK."]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(HFCLKSELW::LFXO)
    }
    #[doc = "Select USHFRCO divided by two as HFCLK."]
    #[inline]
    pub fn ushfrcodiv2(self) -> &'a mut W {
        self.variant(HFCLKSELW::USHFRCODIV2)
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
pub struct _CALSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _CALSTARTW<'a> {
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
pub struct _CALSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _CALSTOPW<'a> {
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
#[doc = "Values that can be written to the field `USBCCLKSEL`"]
pub enum USBCCLKSELW {
    #[doc = "Select LFXO as HFCORECLKUSBC."]
    LFXO,
    #[doc = "Select LFRCO as HFCORECLKUSBC."]
    LFRCO,
    #[doc = "Select USHFRCO as HFCORECLKUSBC."]
    USHFRCO,
}
impl USBCCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USBCCLKSELW::LFXO => 2,
            USBCCLKSELW::LFRCO => 3,
            USBCCLKSELW::USHFRCO => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBCCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBCCLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select LFXO as HFCORECLKUSBC."]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(USBCCLKSELW::LFXO)
    }
    #[doc = "Select LFRCO as HFCORECLKUSBC."]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(USBCCLKSELW::LFRCO)
    }
    #[doc = "Select USHFRCO as HFCORECLKUSBC."]
    #[inline]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(USBCCLKSELW::USHFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline]
    pub fn hfclksel(&mut self) -> _HFCLKSELW {
        _HFCLKSELW { w: self }
    }
    #[doc = "Bit 3 - Calibration Start"]
    #[inline]
    pub fn calstart(&mut self) -> _CALSTARTW {
        _CALSTARTW { w: self }
    }
    #[doc = "Bit 4 - Calibration Stop"]
    #[inline]
    pub fn calstop(&mut self) -> _CALSTOPW {
        _CALSTOPW { w: self }
    }
    #[doc = "Bits 5:7 - USB Core Clock Select"]
    #[inline]
    pub fn usbcclksel(&mut self) -> _USBCCLKSELW {
        _USBCCLKSELW { w: self }
    }
}
