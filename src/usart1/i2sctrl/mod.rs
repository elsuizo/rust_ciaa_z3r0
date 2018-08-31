#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::I2SCTRL {
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
#[doc = r" Value of the field"]
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct MONOR {
    bits: bool,
}
impl MONOR {
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
pub struct JUSTIFYR {
    bits: bool,
}
impl JUSTIFYR {
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
pub struct DMASPLITR {
    bits: bool,
}
impl DMASPLITR {
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
pub struct DELAYR {
    bits: bool,
}
impl DELAYR {
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
#[doc = "Possible values of the field `FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMATR {
    #[doc = "32-bit word, 32-bit data"]
    W32D32,
    #[doc = "32-bit word, 32-bit data with 8 lsb masked"]
    W32D24M,
    #[doc = "32-bit word, 24-bit data"]
    W32D24,
    #[doc = "32-bit word, 16-bit data"]
    W32D16,
    #[doc = "32-bit word, 8-bit data"]
    W32D8,
    #[doc = "16-bit word, 16-bit data"]
    W16D16,
    #[doc = "16-bit word, 8-bit data"]
    W16D8,
    #[doc = "8-bit word, 8-bit data"]
    W8D8,
}
impl FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FORMATR::W32D32 => 0,
            FORMATR::W32D24M => 1,
            FORMATR::W32D24 => 2,
            FORMATR::W32D16 => 3,
            FORMATR::W32D8 => 4,
            FORMATR::W16D16 => 5,
            FORMATR::W16D8 => 6,
            FORMATR::W8D8 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FORMATR {
        match value {
            0 => FORMATR::W32D32,
            1 => FORMATR::W32D24M,
            2 => FORMATR::W32D24,
            3 => FORMATR::W32D16,
            4 => FORMATR::W32D8,
            5 => FORMATR::W16D16,
            6 => FORMATR::W16D8,
            7 => FORMATR::W8D8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `W32D32`"]
    #[inline]
    pub fn is_w32d32(&self) -> bool {
        *self == FORMATR::W32D32
    }
    #[doc = "Checks if the value of the field is `W32D24M`"]
    #[inline]
    pub fn is_w32d24m(&self) -> bool {
        *self == FORMATR::W32D24M
    }
    #[doc = "Checks if the value of the field is `W32D24`"]
    #[inline]
    pub fn is_w32d24(&self) -> bool {
        *self == FORMATR::W32D24
    }
    #[doc = "Checks if the value of the field is `W32D16`"]
    #[inline]
    pub fn is_w32d16(&self) -> bool {
        *self == FORMATR::W32D16
    }
    #[doc = "Checks if the value of the field is `W32D8`"]
    #[inline]
    pub fn is_w32d8(&self) -> bool {
        *self == FORMATR::W32D8
    }
    #[doc = "Checks if the value of the field is `W16D16`"]
    #[inline]
    pub fn is_w16d16(&self) -> bool {
        *self == FORMATR::W16D16
    }
    #[doc = "Checks if the value of the field is `W16D8`"]
    #[inline]
    pub fn is_w16d8(&self) -> bool {
        *self == FORMATR::W16D8
    }
    #[doc = "Checks if the value of the field is `W8D8`"]
    #[inline]
    pub fn is_w8d8(&self) -> bool {
        *self == FORMATR::W8D8
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MONOW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _JUSTIFYW<'a> {
    w: &'a mut W,
}
impl<'a> _JUSTIFYW<'a> {
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
pub struct _DMASPLITW<'a> {
    w: &'a mut W,
}
impl<'a> _DMASPLITW<'a> {
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
pub struct _DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _DELAYW<'a> {
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
#[doc = "Values that can be written to the field `FORMAT`"]
pub enum FORMATW {
    #[doc = "32-bit word, 32-bit data"]
    W32D32,
    #[doc = "32-bit word, 32-bit data with 8 lsb masked"]
    W32D24M,
    #[doc = "32-bit word, 24-bit data"]
    W32D24,
    #[doc = "32-bit word, 16-bit data"]
    W32D16,
    #[doc = "32-bit word, 8-bit data"]
    W32D8,
    #[doc = "16-bit word, 16-bit data"]
    W16D16,
    #[doc = "16-bit word, 8-bit data"]
    W16D8,
    #[doc = "8-bit word, 8-bit data"]
    W8D8,
}
impl FORMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FORMATW::W32D32 => 0,
            FORMATW::W32D24M => 1,
            FORMATW::W32D24 => 2,
            FORMATW::W32D16 => 3,
            FORMATW::W32D8 => 4,
            FORMATW::W16D16 => 5,
            FORMATW::W16D8 => 6,
            FORMATW::W8D8 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORMATW<'a> {
    w: &'a mut W,
}
impl<'a> _FORMATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORMATW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "32-bit word, 32-bit data"]
    #[inline]
    pub fn w32d32(self) -> &'a mut W {
        self.variant(FORMATW::W32D32)
    }
    #[doc = "32-bit word, 32-bit data with 8 lsb masked"]
    #[inline]
    pub fn w32d24m(self) -> &'a mut W {
        self.variant(FORMATW::W32D24M)
    }
    #[doc = "32-bit word, 24-bit data"]
    #[inline]
    pub fn w32d24(self) -> &'a mut W {
        self.variant(FORMATW::W32D24)
    }
    #[doc = "32-bit word, 16-bit data"]
    #[inline]
    pub fn w32d16(self) -> &'a mut W {
        self.variant(FORMATW::W32D16)
    }
    #[doc = "32-bit word, 8-bit data"]
    #[inline]
    pub fn w32d8(self) -> &'a mut W {
        self.variant(FORMATW::W32D8)
    }
    #[doc = "16-bit word, 16-bit data"]
    #[inline]
    pub fn w16d16(self) -> &'a mut W {
        self.variant(FORMATW::W16D16)
    }
    #[doc = "16-bit word, 8-bit data"]
    #[inline]
    pub fn w16d8(self) -> &'a mut W {
        self.variant(FORMATW::W16D8)
    }
    #[doc = "8-bit word, 8-bit data"]
    #[inline]
    pub fn w8d8(self) -> &'a mut W {
        self.variant(FORMATW::W8D8)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline]
    pub fn mono(&self) -> MONOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MONOR { bits }
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline]
    pub fn justify(&self) -> JUSTIFYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        JUSTIFYR { bits }
    }
    #[doc = "Bit 3 - Separate DMA Request For Left/Right Data"]
    #[inline]
    pub fn dmasplit(&self) -> DMASPLITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMASPLITR { bits }
    }
    #[doc = "Bit 4 - Delay on I2S data"]
    #[inline]
    pub fn delay(&self) -> DELAYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DELAYR { bits }
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline]
    pub fn format(&self) -> FORMATR {
        FORMATR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline]
    pub fn mono(&mut self) -> _MONOW {
        _MONOW { w: self }
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline]
    pub fn justify(&mut self) -> _JUSTIFYW {
        _JUSTIFYW { w: self }
    }
    #[doc = "Bit 3 - Separate DMA Request For Left/Right Data"]
    #[inline]
    pub fn dmasplit(&mut self) -> _DMASPLITW {
        _DMASPLITW { w: self }
    }
    #[doc = "Bit 4 - Delay on I2S data"]
    #[inline]
    pub fn delay(&mut self) -> _DELAYW {
        _DELAYW { w: self }
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline]
    pub fn format(&mut self) -> _FORMATW {
        _FORMATW { w: self }
    }
}
