#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DTTIME {
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
#[doc = "Possible values of the field `DTPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPRESCR {
    #[doc = "The HFPERCLK is undivided"]
    DIV1,
    #[doc = "The HFPERCLK is divided by 2"]
    DIV2,
    #[doc = "The HFPERCLK is divided by 4"]
    DIV4,
    #[doc = "The HFPERCLK is divided by 8"]
    DIV8,
    #[doc = "The HFPERCLK is divided by 16"]
    DIV16,
    #[doc = "The HFPERCLK is divided by 32"]
    DIV32,
    #[doc = "The HFPERCLK is divided by 64"]
    DIV64,
    #[doc = "The HFPERCLK is divided by 128"]
    DIV128,
    #[doc = "The HFPERCLK is divided by 256"]
    DIV256,
    #[doc = "The HFPERCLK is divided by 512"]
    DIV512,
    #[doc = "The HFPERCLK is divided by 1024"]
    DIV1024,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTPRESCR::DIV1 => 0,
            DTPRESCR::DIV2 => 1,
            DTPRESCR::DIV4 => 2,
            DTPRESCR::DIV8 => 3,
            DTPRESCR::DIV16 => 4,
            DTPRESCR::DIV32 => 5,
            DTPRESCR::DIV64 => 6,
            DTPRESCR::DIV128 => 7,
            DTPRESCR::DIV256 => 8,
            DTPRESCR::DIV512 => 9,
            DTPRESCR::DIV1024 => 10,
            DTPRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTPRESCR {
        match value {
            0 => DTPRESCR::DIV1,
            1 => DTPRESCR::DIV2,
            2 => DTPRESCR::DIV4,
            3 => DTPRESCR::DIV8,
            4 => DTPRESCR::DIV16,
            5 => DTPRESCR::DIV32,
            6 => DTPRESCR::DIV64,
            7 => DTPRESCR::DIV128,
            8 => DTPRESCR::DIV256,
            9 => DTPRESCR::DIV512,
            10 => DTPRESCR::DIV1024,
            i => DTPRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == DTPRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == DTPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == DTPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == DTPRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == DTPRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == DTPRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == DTPRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == DTPRESCR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == DTPRESCR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == DTPRESCR::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == DTPRESCR::DIV1024
    }
}
#[doc = r" Value of the field"]
pub struct DTRISETR {
    bits: u8,
}
impl DTRISETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DTFALLTR {
    bits: u8,
}
impl DTFALLTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DTPRESC`"]
pub enum DTPRESCW {
    #[doc = "The HFPERCLK is undivided"]
    DIV1,
    #[doc = "The HFPERCLK is divided by 2"]
    DIV2,
    #[doc = "The HFPERCLK is divided by 4"]
    DIV4,
    #[doc = "The HFPERCLK is divided by 8"]
    DIV8,
    #[doc = "The HFPERCLK is divided by 16"]
    DIV16,
    #[doc = "The HFPERCLK is divided by 32"]
    DIV32,
    #[doc = "The HFPERCLK is divided by 64"]
    DIV64,
    #[doc = "The HFPERCLK is divided by 128"]
    DIV128,
    #[doc = "The HFPERCLK is divided by 256"]
    DIV256,
    #[doc = "The HFPERCLK is divided by 512"]
    DIV512,
    #[doc = "The HFPERCLK is divided by 1024"]
    DIV1024,
}
impl DTPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTPRESCW::DIV1 => 0,
            DTPRESCW::DIV2 => 1,
            DTPRESCW::DIV4 => 2,
            DTPRESCW::DIV8 => 3,
            DTPRESCW::DIV16 => 4,
            DTPRESCW::DIV32 => 5,
            DTPRESCW::DIV64 => 6,
            DTPRESCW::DIV128 => 7,
            DTPRESCW::DIV256 => 8,
            DTPRESCW::DIV512 => 9,
            DTPRESCW::DIV1024 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTPRESCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The HFPERCLK is undivided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(DTPRESCW::DIV1024)
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
pub struct _DTRISETW<'a> {
    w: &'a mut W,
}
impl<'a> _DTRISETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTFALLTW<'a> {
    w: &'a mut W,
}
impl<'a> _DTFALLTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline]
    pub fn dtpresc(&self) -> DTPRESCR {
        DTPRESCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline]
    pub fn dtriset(&self) -> DTRISETR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTRISETR { bits }
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline]
    pub fn dtfallt(&self) -> DTFALLTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTFALLTR { bits }
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
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline]
    pub fn dtpresc(&mut self) -> _DTPRESCW {
        _DTPRESCW { w: self }
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline]
    pub fn dtriset(&mut self) -> _DTRISETW {
        _DTRISETW { w: self }
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline]
    pub fn dtfallt(&mut self) -> _DTFALLTW {
        _DTFALLTW { w: self }
    }
}
