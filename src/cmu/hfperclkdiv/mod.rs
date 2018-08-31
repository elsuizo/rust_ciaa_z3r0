#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFPERCLKDIV {
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
#[doc = "Possible values of the field `HFPERCLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFPERCLKDIVR {
    #[doc = "HFPERCLK = HFCLK."]
    HFCLK,
    #[doc = "HFPERCLK = HFCLK/2."]
    HFCLK2,
    #[doc = "HFPERCLK = HFCLK/4."]
    HFCLK4,
    #[doc = "HFPERCLK = HFCLK/8."]
    HFCLK8,
    #[doc = "HFPERCLK = HFCLK/16."]
    HFCLK16,
    #[doc = "HFPERCLK = HFCLK/32."]
    HFCLK32,
    #[doc = "HFPERCLK = HFCLK/64."]
    HFCLK64,
    #[doc = "HFPERCLK = HFCLK/128."]
    HFCLK128,
    #[doc = "HFPERCLK = HFCLK/256."]
    HFCLK256,
    #[doc = "HFPERCLK = HFCLK/512."]
    HFCLK512,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HFPERCLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFPERCLKDIVR::HFCLK => 0,
            HFPERCLKDIVR::HFCLK2 => 1,
            HFPERCLKDIVR::HFCLK4 => 2,
            HFPERCLKDIVR::HFCLK8 => 3,
            HFPERCLKDIVR::HFCLK16 => 4,
            HFPERCLKDIVR::HFCLK32 => 5,
            HFPERCLKDIVR::HFCLK64 => 6,
            HFPERCLKDIVR::HFCLK128 => 7,
            HFPERCLKDIVR::HFCLK256 => 8,
            HFPERCLKDIVR::HFCLK512 => 9,
            HFPERCLKDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFPERCLKDIVR {
        match value {
            0 => HFPERCLKDIVR::HFCLK,
            1 => HFPERCLKDIVR::HFCLK2,
            2 => HFPERCLKDIVR::HFCLK4,
            3 => HFPERCLKDIVR::HFCLK8,
            4 => HFPERCLKDIVR::HFCLK16,
            5 => HFPERCLKDIVR::HFCLK32,
            6 => HFPERCLKDIVR::HFCLK64,
            7 => HFPERCLKDIVR::HFCLK128,
            8 => HFPERCLKDIVR::HFCLK256,
            9 => HFPERCLKDIVR::HFCLK512,
            i => HFPERCLKDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline]
    pub fn is_hfclk(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFCLK2`"]
    #[inline]
    pub fn is_hfclk2(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK2
    }
    #[doc = "Checks if the value of the field is `HFCLK4`"]
    #[inline]
    pub fn is_hfclk4(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK4
    }
    #[doc = "Checks if the value of the field is `HFCLK8`"]
    #[inline]
    pub fn is_hfclk8(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK8
    }
    #[doc = "Checks if the value of the field is `HFCLK16`"]
    #[inline]
    pub fn is_hfclk16(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK16
    }
    #[doc = "Checks if the value of the field is `HFCLK32`"]
    #[inline]
    pub fn is_hfclk32(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK32
    }
    #[doc = "Checks if the value of the field is `HFCLK64`"]
    #[inline]
    pub fn is_hfclk64(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK64
    }
    #[doc = "Checks if the value of the field is `HFCLK128`"]
    #[inline]
    pub fn is_hfclk128(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK128
    }
    #[doc = "Checks if the value of the field is `HFCLK256`"]
    #[inline]
    pub fn is_hfclk256(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK256
    }
    #[doc = "Checks if the value of the field is `HFCLK512`"]
    #[inline]
    pub fn is_hfclk512(&self) -> bool {
        *self == HFPERCLKDIVR::HFCLK512
    }
}
#[doc = r" Value of the field"]
pub struct HFPERCLKENR {
    bits: bool,
}
impl HFPERCLKENR {
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
#[doc = "Values that can be written to the field `HFPERCLKDIV`"]
pub enum HFPERCLKDIVW {
    #[doc = "HFPERCLK = HFCLK."]
    HFCLK,
    #[doc = "HFPERCLK = HFCLK/2."]
    HFCLK2,
    #[doc = "HFPERCLK = HFCLK/4."]
    HFCLK4,
    #[doc = "HFPERCLK = HFCLK/8."]
    HFCLK8,
    #[doc = "HFPERCLK = HFCLK/16."]
    HFCLK16,
    #[doc = "HFPERCLK = HFCLK/32."]
    HFCLK32,
    #[doc = "HFPERCLK = HFCLK/64."]
    HFCLK64,
    #[doc = "HFPERCLK = HFCLK/128."]
    HFCLK128,
    #[doc = "HFPERCLK = HFCLK/256."]
    HFCLK256,
    #[doc = "HFPERCLK = HFCLK/512."]
    HFCLK512,
}
impl HFPERCLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFPERCLKDIVW::HFCLK => 0,
            HFPERCLKDIVW::HFCLK2 => 1,
            HFPERCLKDIVW::HFCLK4 => 2,
            HFPERCLKDIVW::HFCLK8 => 3,
            HFPERCLKDIVW::HFCLK16 => 4,
            HFPERCLKDIVW::HFCLK32 => 5,
            HFPERCLKDIVW::HFCLK64 => 6,
            HFPERCLKDIVW::HFCLK128 => 7,
            HFPERCLKDIVW::HFCLK256 => 8,
            HFPERCLKDIVW::HFCLK512 => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFPERCLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _HFPERCLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFPERCLKDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HFPERCLK = HFCLK."]
    #[inline]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK)
    }
    #[doc = "HFPERCLK = HFCLK/2."]
    #[inline]
    pub fn hfclk2(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK2)
    }
    #[doc = "HFPERCLK = HFCLK/4."]
    #[inline]
    pub fn hfclk4(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK4)
    }
    #[doc = "HFPERCLK = HFCLK/8."]
    #[inline]
    pub fn hfclk8(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK8)
    }
    #[doc = "HFPERCLK = HFCLK/16."]
    #[inline]
    pub fn hfclk16(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK16)
    }
    #[doc = "HFPERCLK = HFCLK/32."]
    #[inline]
    pub fn hfclk32(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK32)
    }
    #[doc = "HFPERCLK = HFCLK/64."]
    #[inline]
    pub fn hfclk64(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK64)
    }
    #[doc = "HFPERCLK = HFCLK/128."]
    #[inline]
    pub fn hfclk128(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK128)
    }
    #[doc = "HFPERCLK = HFCLK/256."]
    #[inline]
    pub fn hfclk256(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK256)
    }
    #[doc = "HFPERCLK = HFCLK/512."]
    #[inline]
    pub fn hfclk512(self) -> &'a mut W {
        self.variant(HFPERCLKDIVW::HFCLK512)
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
pub struct _HFPERCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _HFPERCLKENW<'a> {
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
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline]
    pub fn hfperclkdiv(&self) -> HFPERCLKDIVR {
        HFPERCLKDIVR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline]
    pub fn hfperclken(&self) -> HFPERCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HFPERCLKENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline]
    pub fn hfperclkdiv(&mut self) -> _HFPERCLKDIVW {
        _HFPERCLKDIVW { w: self }
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline]
    pub fn hfperclken(&mut self) -> _HFPERCLKENW {
        _HFPERCLKENW { w: self }
    }
}
