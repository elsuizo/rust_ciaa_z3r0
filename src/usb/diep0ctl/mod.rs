#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIEP0CTL {
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
#[doc = "Possible values of the field `MPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPSR {
    #[doc = "64 bytes."]
    _64B,
    #[doc = "32 bytes."]
    _32B,
    #[doc = "16 bytes."]
    _16B,
    #[doc = "8 bytes."]
    _8B,
}
impl MPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MPSR::_64B => 0,
            MPSR::_32B => 1,
            MPSR::_16B => 2,
            MPSR::_8B => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MPSR {
        match value {
            0 => MPSR::_64B,
            1 => MPSR::_32B,
            2 => MPSR::_16B,
            3 => MPSR::_8B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64B`"]
    #[inline]
    pub fn is_64b(&self) -> bool {
        *self == MPSR::_64B
    }
    #[doc = "Checks if the value of the field is `_32B`"]
    #[inline]
    pub fn is_32b(&self) -> bool {
        *self == MPSR::_32B
    }
    #[doc = "Checks if the value of the field is `_16B`"]
    #[inline]
    pub fn is_16b(&self) -> bool {
        *self == MPSR::_16B
    }
    #[doc = "Checks if the value of the field is `_8B`"]
    #[inline]
    pub fn is_8b(&self) -> bool {
        *self == MPSR::_8B
    }
}
#[doc = r" Value of the field"]
pub struct USBACTEPR {
    bits: bool,
}
impl USBACTEPR {
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
pub struct NAKSTSR {
    bits: bool,
}
impl NAKSTSR {
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
pub struct EPTYPER {
    bits: u8,
}
impl EPTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STALLR {
    bits: bool,
}
impl STALLR {
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
pub struct TXFNUMR {
    bits: u8,
}
impl TXFNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EPDISR {
    bits: bool,
}
impl EPDISR {
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
pub struct EPENAR {
    bits: bool,
}
impl EPENAR {
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
#[doc = "Values that can be written to the field `MPS`"]
pub enum MPSW {
    #[doc = "64 bytes."]
    _64B,
    #[doc = "32 bytes."]
    _32B,
    #[doc = "16 bytes."]
    _16B,
    #[doc = "8 bytes."]
    _8B,
}
impl MPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MPSW::_64B => 0,
            MPSW::_32B => 1,
            MPSW::_16B => 2,
            MPSW::_8B => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPSW<'a> {
    w: &'a mut W,
}
impl<'a> _MPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "64 bytes."]
    #[inline]
    pub fn _64b(self) -> &'a mut W {
        self.variant(MPSW::_64B)
    }
    #[doc = "32 bytes."]
    #[inline]
    pub fn _32b(self) -> &'a mut W {
        self.variant(MPSW::_32B)
    }
    #[doc = "16 bytes."]
    #[inline]
    pub fn _16b(self) -> &'a mut W {
        self.variant(MPSW::_16B)
    }
    #[doc = "8 bytes."]
    #[inline]
    pub fn _8b(self) -> &'a mut W {
        self.variant(MPSW::_8B)
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
pub struct _STALLW<'a> {
    w: &'a mut W,
}
impl<'a> _STALLW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXFNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _CNAKW<'a> {
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
pub struct _SNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _SNAKW<'a> {
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
pub struct _EPDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPDISW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPENAW<'a> {
    w: &'a mut W,
}
impl<'a> _EPENAW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline]
    pub fn mps(&self) -> MPSR {
        MPSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline]
    pub fn usbactep(&self) -> USBACTEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USBACTEPR { bits }
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline]
    pub fn naksts(&self) -> NAKSTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NAKSTSR { bits }
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline]
    pub fn eptype(&self) -> EPTYPER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPTYPER { bits }
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline]
    pub fn stall(&self) -> STALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STALLR { bits }
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline]
    pub fn txfnum(&self) -> TXFNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXFNUMR { bits }
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline]
    pub fn epdis(&self) -> EPDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EPDISR { bits }
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline]
    pub fn epena(&self) -> EPENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EPENAR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32768 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline]
    pub fn mps(&mut self) -> _MPSW {
        _MPSW { w: self }
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline]
    pub fn stall(&mut self) -> _STALLW {
        _STALLW { w: self }
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline]
    pub fn txfnum(&mut self) -> _TXFNUMW {
        _TXFNUMW { w: self }
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline]
    pub fn cnak(&mut self) -> _CNAKW {
        _CNAKW { w: self }
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline]
    pub fn snak(&mut self) -> _SNAKW {
        _SNAKW { w: self }
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline]
    pub fn epdis(&mut self) -> _EPDISW {
        _EPDISW { w: self }
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline]
    pub fn epena(&mut self) -> _EPENAW {
        _EPENAW { w: self }
    }
}
