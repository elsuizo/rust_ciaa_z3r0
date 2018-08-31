#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFCORECLKDIV {
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
#[doc = "Possible values of the field `HFCORECLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCORECLKDIVR {
    #[doc = "HFCORECLK = HFCLK."]
    HFCLK,
    #[doc = "HFCORECLK = HFCLK/2."]
    HFCLK2,
    #[doc = "HFCORECLK = HFCLK/4."]
    HFCLK4,
    #[doc = "HFCORECLK = HFCLK/8."]
    HFCLK8,
    #[doc = "HFCORECLK = HFCLK/16."]
    HFCLK16,
    #[doc = "HFCORECLK = HFCLK/32."]
    HFCLK32,
    #[doc = "HFCORECLK = HFCLK/64."]
    HFCLK64,
    #[doc = "HFCORECLK = HFCLK/128."]
    HFCLK128,
    #[doc = "HFCORECLK = HFCLK/256."]
    HFCLK256,
    #[doc = "HFCORECLK = HFCLK/512."]
    HFCLK512,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HFCORECLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFCORECLKDIVR::HFCLK => 0,
            HFCORECLKDIVR::HFCLK2 => 1,
            HFCORECLKDIVR::HFCLK4 => 2,
            HFCORECLKDIVR::HFCLK8 => 3,
            HFCORECLKDIVR::HFCLK16 => 4,
            HFCORECLKDIVR::HFCLK32 => 5,
            HFCORECLKDIVR::HFCLK64 => 6,
            HFCORECLKDIVR::HFCLK128 => 7,
            HFCORECLKDIVR::HFCLK256 => 8,
            HFCORECLKDIVR::HFCLK512 => 9,
            HFCORECLKDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFCORECLKDIVR {
        match value {
            0 => HFCORECLKDIVR::HFCLK,
            1 => HFCORECLKDIVR::HFCLK2,
            2 => HFCORECLKDIVR::HFCLK4,
            3 => HFCORECLKDIVR::HFCLK8,
            4 => HFCORECLKDIVR::HFCLK16,
            5 => HFCORECLKDIVR::HFCLK32,
            6 => HFCORECLKDIVR::HFCLK64,
            7 => HFCORECLKDIVR::HFCLK128,
            8 => HFCORECLKDIVR::HFCLK256,
            9 => HFCORECLKDIVR::HFCLK512,
            i => HFCORECLKDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline]
    pub fn is_hfclk(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFCLK2`"]
    #[inline]
    pub fn is_hfclk2(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK2
    }
    #[doc = "Checks if the value of the field is `HFCLK4`"]
    #[inline]
    pub fn is_hfclk4(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK4
    }
    #[doc = "Checks if the value of the field is `HFCLK8`"]
    #[inline]
    pub fn is_hfclk8(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK8
    }
    #[doc = "Checks if the value of the field is `HFCLK16`"]
    #[inline]
    pub fn is_hfclk16(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK16
    }
    #[doc = "Checks if the value of the field is `HFCLK32`"]
    #[inline]
    pub fn is_hfclk32(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK32
    }
    #[doc = "Checks if the value of the field is `HFCLK64`"]
    #[inline]
    pub fn is_hfclk64(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK64
    }
    #[doc = "Checks if the value of the field is `HFCLK128`"]
    #[inline]
    pub fn is_hfclk128(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK128
    }
    #[doc = "Checks if the value of the field is `HFCLK256`"]
    #[inline]
    pub fn is_hfclk256(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK256
    }
    #[doc = "Checks if the value of the field is `HFCLK512`"]
    #[inline]
    pub fn is_hfclk512(&self) -> bool {
        *self == HFCORECLKDIVR::HFCLK512
    }
}
#[doc = r" Value of the field"]
pub struct HFCORECLKLEDIVR {
    bits: bool,
}
impl HFCORECLKLEDIVR {
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
#[doc = "Values that can be written to the field `HFCORECLKDIV`"]
pub enum HFCORECLKDIVW {
    #[doc = "HFCORECLK = HFCLK."]
    HFCLK,
    #[doc = "HFCORECLK = HFCLK/2."]
    HFCLK2,
    #[doc = "HFCORECLK = HFCLK/4."]
    HFCLK4,
    #[doc = "HFCORECLK = HFCLK/8."]
    HFCLK8,
    #[doc = "HFCORECLK = HFCLK/16."]
    HFCLK16,
    #[doc = "HFCORECLK = HFCLK/32."]
    HFCLK32,
    #[doc = "HFCORECLK = HFCLK/64."]
    HFCLK64,
    #[doc = "HFCORECLK = HFCLK/128."]
    HFCLK128,
    #[doc = "HFCORECLK = HFCLK/256."]
    HFCLK256,
    #[doc = "HFCORECLK = HFCLK/512."]
    HFCLK512,
}
impl HFCORECLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFCORECLKDIVW::HFCLK => 0,
            HFCORECLKDIVW::HFCLK2 => 1,
            HFCORECLKDIVW::HFCLK4 => 2,
            HFCORECLKDIVW::HFCLK8 => 3,
            HFCORECLKDIVW::HFCLK16 => 4,
            HFCORECLKDIVW::HFCLK32 => 5,
            HFCORECLKDIVW::HFCLK64 => 6,
            HFCORECLKDIVW::HFCLK128 => 7,
            HFCORECLKDIVW::HFCLK256 => 8,
            HFCORECLKDIVW::HFCLK512 => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFCORECLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _HFCORECLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFCORECLKDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HFCORECLK = HFCLK."]
    #[inline]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK)
    }
    #[doc = "HFCORECLK = HFCLK/2."]
    #[inline]
    pub fn hfclk2(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK2)
    }
    #[doc = "HFCORECLK = HFCLK/4."]
    #[inline]
    pub fn hfclk4(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK4)
    }
    #[doc = "HFCORECLK = HFCLK/8."]
    #[inline]
    pub fn hfclk8(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK8)
    }
    #[doc = "HFCORECLK = HFCLK/16."]
    #[inline]
    pub fn hfclk16(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK16)
    }
    #[doc = "HFCORECLK = HFCLK/32."]
    #[inline]
    pub fn hfclk32(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK32)
    }
    #[doc = "HFCORECLK = HFCLK/64."]
    #[inline]
    pub fn hfclk64(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK64)
    }
    #[doc = "HFCORECLK = HFCLK/128."]
    #[inline]
    pub fn hfclk128(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK128)
    }
    #[doc = "HFCORECLK = HFCLK/256."]
    #[inline]
    pub fn hfclk256(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK256)
    }
    #[doc = "HFCORECLK = HFCLK/512."]
    #[inline]
    pub fn hfclk512(self) -> &'a mut W {
        self.variant(HFCORECLKDIVW::HFCLK512)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HFCORECLKLEDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _HFCORECLKLEDIVW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - HFCORECLK Divider"]
    #[inline]
    pub fn hfcoreclkdiv(&self) -> HFCORECLKDIVR {
        HFCORECLKDIVR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Additional Division Factor For HFCORECLKLE"]
    #[inline]
    pub fn hfcoreclklediv(&self) -> HFCORECLKLEDIVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HFCORECLKLEDIVR { bits }
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
    #[doc = "Bits 0:3 - HFCORECLK Divider"]
    #[inline]
    pub fn hfcoreclkdiv(&mut self) -> _HFCORECLKDIVW {
        _HFCORECLKDIVW { w: self }
    }
    #[doc = "Bit 8 - Additional Division Factor For HFCORECLKLE"]
    #[inline]
    pub fn hfcoreclklediv(&mut self) -> _HFCORECLKLEDIVW {
        _HFCORECLKLEDIVW { w: self }
    }
}
