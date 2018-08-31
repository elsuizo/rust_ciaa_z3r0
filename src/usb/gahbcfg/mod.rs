#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GAHBCFG {
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
pub struct GLBLINTRMSKR {
    bits: bool,
}
impl GLBLINTRMSKR {
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
#[doc = "Possible values of the field `HBSTLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HBSTLENR {
    #[doc = "Single transfer."]
    SINGLE,
    #[doc = "Incrementing burst of unspecified length."]
    INCR,
    #[doc = "4-beat incrementing burst."]
    INCR4,
    #[doc = "8-beat incrementing burst."]
    INCR8,
    #[doc = "16-beat incrementing burst."]
    INCR16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HBSTLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HBSTLENR::SINGLE => 0,
            HBSTLENR::INCR => 1,
            HBSTLENR::INCR4 => 3,
            HBSTLENR::INCR8 => 5,
            HBSTLENR::INCR16 => 7,
            HBSTLENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HBSTLENR {
        match value {
            0 => HBSTLENR::SINGLE,
            1 => HBSTLENR::INCR,
            3 => HBSTLENR::INCR4,
            5 => HBSTLENR::INCR8,
            7 => HBSTLENR::INCR16,
            i => HBSTLENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == HBSTLENR::SINGLE
    }
    #[doc = "Checks if the value of the field is `INCR`"]
    #[inline]
    pub fn is_incr(&self) -> bool {
        *self == HBSTLENR::INCR
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline]
    pub fn is_incr4(&self) -> bool {
        *self == HBSTLENR::INCR4
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline]
    pub fn is_incr8(&self) -> bool {
        *self == HBSTLENR::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline]
    pub fn is_incr16(&self) -> bool {
        *self == HBSTLENR::INCR16
    }
}
#[doc = r" Value of the field"]
pub struct DMAENR {
    bits: bool,
}
impl DMAENR {
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
pub struct NPTXFEMPLVLR {
    bits: bool,
}
impl NPTXFEMPLVLR {
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
pub struct REMMEMSUPPR {
    bits: bool,
}
impl REMMEMSUPPR {
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
pub struct NOTIALLDMAWRITR {
    bits: bool,
}
impl NOTIALLDMAWRITR {
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
pub struct AHBSINGLER {
    bits: bool,
}
impl AHBSINGLER {
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
#[doc = r" Proxy"]
pub struct _GLBLINTRMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _GLBLINTRMSKW<'a> {
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
#[doc = "Values that can be written to the field `HBSTLEN`"]
pub enum HBSTLENW {
    #[doc = "Single transfer."]
    SINGLE,
    #[doc = "Incrementing burst of unspecified length."]
    INCR,
    #[doc = "4-beat incrementing burst."]
    INCR4,
    #[doc = "8-beat incrementing burst."]
    INCR8,
    #[doc = "16-beat incrementing burst."]
    INCR16,
}
impl HBSTLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HBSTLENW::SINGLE => 0,
            HBSTLENW::INCR => 1,
            HBSTLENW::INCR4 => 3,
            HBSTLENW::INCR8 => 5,
            HBSTLENW::INCR16 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HBSTLENW<'a> {
    w: &'a mut W,
}
impl<'a> _HBSTLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HBSTLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single transfer."]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(HBSTLENW::SINGLE)
    }
    #[doc = "Incrementing burst of unspecified length."]
    #[inline]
    pub fn incr(self) -> &'a mut W {
        self.variant(HBSTLENW::INCR)
    }
    #[doc = "4-beat incrementing burst."]
    #[inline]
    pub fn incr4(self) -> &'a mut W {
        self.variant(HBSTLENW::INCR4)
    }
    #[doc = "8-beat incrementing burst."]
    #[inline]
    pub fn incr8(self) -> &'a mut W {
        self.variant(HBSTLENW::INCR8)
    }
    #[doc = "16-beat incrementing burst."]
    #[inline]
    pub fn incr16(self) -> &'a mut W {
        self.variant(HBSTLENW::INCR16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
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
pub struct _NPTXFEMPLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _NPTXFEMPLVLW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REMMEMSUPPW<'a> {
    w: &'a mut W,
}
impl<'a> _REMMEMSUPPW<'a> {
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
pub struct _NOTIALLDMAWRITW<'a> {
    w: &'a mut W,
}
impl<'a> _NOTIALLDMAWRITW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AHBSINGLEW<'a> {
    w: &'a mut W,
}
impl<'a> _AHBSINGLEW<'a> {
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline]
    pub fn glblintrmsk(&self) -> GLBLINTRMSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GLBLINTRMSKR { bits }
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline]
    pub fn hbstlen(&self) -> HBSTLENR {
        HBSTLENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAENR { bits }
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline]
    pub fn nptxfemplvl(&self) -> NPTXFEMPLVLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NPTXFEMPLVLR { bits }
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline]
    pub fn remmemsupp(&self) -> REMMEMSUPPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REMMEMSUPPR { bits }
    }
    #[doc = "Bit 22 - Notify All DMA Writes"]
    #[inline]
    pub fn notialldmawrit(&self) -> NOTIALLDMAWRITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOTIALLDMAWRITR { bits }
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline]
    pub fn ahbsingle(&self) -> AHBSINGLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AHBSINGLER { bits }
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
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline]
    pub fn glblintrmsk(&mut self) -> _GLBLINTRMSKW {
        _GLBLINTRMSKW { w: self }
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline]
    pub fn hbstlen(&mut self) -> _HBSTLENW {
        _HBSTLENW { w: self }
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline]
    pub fn nptxfemplvl(&mut self) -> _NPTXFEMPLVLW {
        _NPTXFEMPLVLW { w: self }
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline]
    pub fn remmemsupp(&mut self) -> _REMMEMSUPPW {
        _REMMEMSUPPW { w: self }
    }
    #[doc = "Bit 22 - Notify All DMA Writes"]
    #[inline]
    pub fn notialldmawrit(&mut self) -> _NOTIALLDMAWRITW {
        _NOTIALLDMAWRITW { w: self }
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline]
    pub fn ahbsingle(&mut self) -> _AHBSINGLEW {
        _AHBSINGLEW { w: self }
    }
}
