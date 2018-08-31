#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USHFRCOCONF {
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
#[doc = "Possible values of the field `BAND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANDR {
    #[doc = "48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _48MHZ,
    #[doc = "24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _24MHZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BANDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BANDR::_48MHZ => 1,
            BANDR::_24MHZ => 3,
            BANDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BANDR {
        match value {
            1 => BANDR::_48MHZ,
            3 => BANDR::_24MHZ,
            i => BANDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_48MHZ`"]
    #[inline]
    pub fn is_48mhz(&self) -> bool {
        *self == BANDR::_48MHZ
    }
    #[doc = "Checks if the value of the field is `_24MHZ`"]
    #[inline]
    pub fn is_24mhz(&self) -> bool {
        *self == BANDR::_24MHZ
    }
}
#[doc = r" Value of the field"]
pub struct USHFRCODIV2DISR {
    bits: bool,
}
impl USHFRCODIV2DISR {
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
#[doc = "Values that can be written to the field `BAND`"]
pub enum BANDW {
    #[doc = "48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _48MHZ,
    #[doc = "24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _24MHZ,
}
impl BANDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BANDW::_48MHZ => 1,
            BANDW::_24MHZ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BANDW<'a> {
    w: &'a mut W,
}
impl<'a> _BANDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BANDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline]
    pub fn _48mhz(self) -> &'a mut W {
        self.variant(BANDW::_48MHZ)
    }
    #[doc = "24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline]
    pub fn _24mhz(self) -> &'a mut W {
        self.variant(BANDW::_24MHZ)
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
pub struct _USHFRCODIV2DISW<'a> {
    w: &'a mut W,
}
impl<'a> _USHFRCODIV2DISW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline]
    pub fn band(&self) -> BANDR {
        BANDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline]
    pub fn ushfrcodiv2dis(&self) -> USHFRCODIV2DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USHFRCODIV2DISR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline]
    pub fn band(&mut self) -> _BANDW {
        _BANDW { w: self }
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline]
    pub fn ushfrcodiv2dis(&mut self) -> _USHFRCODIV2DISW {
        _USHFRCODIV2DISW { w: self }
    }
}
