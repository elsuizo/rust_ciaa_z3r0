#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct DMPUAPR {
    bits: bool,
}
impl DMPUAPR {
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
#[doc = "Possible values of the field `LEMOSCCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEMOSCCTRLR {
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    NONE,
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    GATE,
    #[doc = "The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    SUSPEND,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LEMOSCCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LEMOSCCTRLR::NONE => 0,
            LEMOSCCTRLR::GATE => 1,
            LEMOSCCTRLR::SUSPEND => 2,
            LEMOSCCTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LEMOSCCTRLR {
        match value {
            0 => LEMOSCCTRLR::NONE,
            1 => LEMOSCCTRLR::GATE,
            2 => LEMOSCCTRLR::SUSPEND,
            i => LEMOSCCTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == LEMOSCCTRLR::NONE
    }
    #[doc = "Checks if the value of the field is `GATE`"]
    #[inline]
    pub fn is_gate(&self) -> bool {
        *self == LEMOSCCTRLR::GATE
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == LEMOSCCTRLR::SUSPEND
    }
}
#[doc = r" Value of the field"]
pub struct LEMPHYCTRLR {
    bits: bool,
}
impl LEMPHYCTRLR {
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
pub struct LEMIDLEENR {
    bits: bool,
}
impl LEMIDLEENR {
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
pub struct LEMNAKENR {
    bits: bool,
}
impl LEMNAKENR {
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
pub struct LEMADDRMENR {
    bits: bool,
}
impl LEMADDRMENR {
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
pub struct VREGDISR {
    bits: bool,
}
impl VREGDISR {
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
pub struct VREGOSENR {
    bits: bool,
}
impl VREGOSENR {
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
pub struct BIASPROGEM01R {
    bits: u8,
}
impl BIASPROGEM01R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BIASPROGEM23R {
    bits: u8,
}
impl BIASPROGEM23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DMPUAPW<'a> {
    w: &'a mut W,
}
impl<'a> _DMPUAPW<'a> {
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
#[doc = "Values that can be written to the field `LEMOSCCTRL`"]
pub enum LEMOSCCTRLW {
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    NONE,
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    GATE,
    #[doc = "The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    SUSPEND,
}
impl LEMOSCCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LEMOSCCTRLW::NONE => 0,
            LEMOSCCTRLW::GATE => 1,
            LEMOSCCTRLW::SUSPEND => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEMOSCCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LEMOSCCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEMOSCCTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(LEMOSCCTRLW::NONE)
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline]
    pub fn gate(self) -> &'a mut W {
        self.variant(LEMOSCCTRLW::GATE)
    }
    #[doc = "The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(LEMOSCCTRLW::SUSPEND)
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
pub struct _LEMPHYCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LEMPHYCTRLW<'a> {
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
pub struct _LEMIDLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _LEMIDLEENW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LEMNAKENW<'a> {
    w: &'a mut W,
}
impl<'a> _LEMNAKENW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LEMADDRMENW<'a> {
    w: &'a mut W,
}
impl<'a> _LEMADDRMENW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VREGDISW<'a> {
    w: &'a mut W,
}
impl<'a> _VREGDISW<'a> {
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
pub struct _VREGOSENW<'a> {
    w: &'a mut W,
}
impl<'a> _VREGOSENW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BIASPROGEM01W<'a> {
    w: &'a mut W,
}
impl<'a> _BIASPROGEM01W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BIASPROGEM23W<'a> {
    w: &'a mut W,
}
impl<'a> _BIASPROGEM23W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline]
    pub fn dmpuap(&self) -> DMPUAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMPUAPR { bits }
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline]
    pub fn lemoscctrl(&self) -> LEMOSCCTRLR {
        LEMOSCCTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline]
    pub fn lemphyctrl(&self) -> LEMPHYCTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LEMPHYCTRLR { bits }
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline]
    pub fn lemidleen(&self) -> LEMIDLEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LEMIDLEENR { bits }
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline]
    pub fn lemnaken(&self) -> LEMNAKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LEMNAKENR { bits }
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline]
    pub fn lemaddrmen(&self) -> LEMADDRMENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LEMADDRMENR { bits }
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline]
    pub fn vregdis(&self) -> VREGDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VREGDISR { bits }
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline]
    pub fn vregosen(&self) -> VREGOSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VREGOSENR { bits }
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline]
    pub fn biasprogem01(&self) -> BIASPROGEM01R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BIASPROGEM01R { bits }
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline]
    pub fn biasprogem23(&self) -> BIASPROGEM23R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BIASPROGEM23R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline]
    pub fn dmpuap(&mut self) -> _DMPUAPW {
        _DMPUAPW { w: self }
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline]
    pub fn lemoscctrl(&mut self) -> _LEMOSCCTRLW {
        _LEMOSCCTRLW { w: self }
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline]
    pub fn lemphyctrl(&mut self) -> _LEMPHYCTRLW {
        _LEMPHYCTRLW { w: self }
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline]
    pub fn lemidleen(&mut self) -> _LEMIDLEENW {
        _LEMIDLEENW { w: self }
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline]
    pub fn lemnaken(&mut self) -> _LEMNAKENW {
        _LEMNAKENW { w: self }
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline]
    pub fn lemaddrmen(&mut self) -> _LEMADDRMENW {
        _LEMADDRMENW { w: self }
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline]
    pub fn vregdis(&mut self) -> _VREGDISW {
        _VREGDISW { w: self }
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline]
    pub fn vregosen(&mut self) -> _VREGOSENW {
        _VREGOSENW { w: self }
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline]
    pub fn biasprogem01(&mut self) -> _BIASPROGEM01W {
        _BIASPROGEM01W { w: self }
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline]
    pub fn biasprogem23(&mut self) -> _BIASPROGEM23W {
        _BIASPROGEM23W { w: self }
    }
}
