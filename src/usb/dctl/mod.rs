#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCTL {
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
pub struct RMTWKUPSIGR {
    bits: bool,
}
impl RMTWKUPSIGR {
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
pub struct SFTDISCONR {
    bits: bool,
}
impl SFTDISCONR {
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
pub struct GNPINNAKSTSR {
    bits: bool,
}
impl GNPINNAKSTSR {
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
pub struct GOUTNAKSTSR {
    bits: bool,
}
impl GOUTNAKSTSR {
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
#[doc = "Possible values of the field `TSTCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTCTLR {
    #[doc = "Test mode disabled."]
    DISABLE,
    #[doc = "Test_J mode."]
    J,
    #[doc = "Test_K mode."]
    K,
    #[doc = "Test_SE0_NAK mode."]
    SE0NAK,
    #[doc = "Test_Packet mode."]
    PACKET,
    #[doc = "Test_Force_Enable."]
    FORCE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSTCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTCTLR::DISABLE => 0,
            TSTCTLR::J => 1,
            TSTCTLR::K => 2,
            TSTCTLR::SE0NAK => 3,
            TSTCTLR::PACKET => 4,
            TSTCTLR::FORCE => 5,
            TSTCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTCTLR {
        match value {
            0 => TSTCTLR::DISABLE,
            1 => TSTCTLR::J,
            2 => TSTCTLR::K,
            3 => TSTCTLR::SE0NAK,
            4 => TSTCTLR::PACKET,
            5 => TSTCTLR::FORCE,
            i => TSTCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TSTCTLR::DISABLE
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline]
    pub fn is_j(&self) -> bool {
        *self == TSTCTLR::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline]
    pub fn is_k(&self) -> bool {
        *self == TSTCTLR::K
    }
    #[doc = "Checks if the value of the field is `SE0NAK`"]
    #[inline]
    pub fn is_se0nak(&self) -> bool {
        *self == TSTCTLR::SE0NAK
    }
    #[doc = "Checks if the value of the field is `PACKET`"]
    #[inline]
    pub fn is_packet(&self) -> bool {
        *self == TSTCTLR::PACKET
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline]
    pub fn is_force(&self) -> bool {
        *self == TSTCTLR::FORCE
    }
}
#[doc = r" Value of the field"]
pub struct PWRONPRGDONER {
    bits: bool,
}
impl PWRONPRGDONER {
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
pub struct IGNRFRMNUMR {
    bits: bool,
}
impl IGNRFRMNUMR {
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
pub struct NAKONBBLER {
    bits: bool,
}
impl NAKONBBLER {
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
pub struct _RMTWKUPSIGW<'a> {
    w: &'a mut W,
}
impl<'a> _RMTWKUPSIGW<'a> {
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
pub struct _SFTDISCONW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTDISCONW<'a> {
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
#[doc = "Values that can be written to the field `TSTCTL`"]
pub enum TSTCTLW {
    #[doc = "Test mode disabled."]
    DISABLE,
    #[doc = "Test_J mode."]
    J,
    #[doc = "Test_K mode."]
    K,
    #[doc = "Test_SE0_NAK mode."]
    SE0NAK,
    #[doc = "Test_Packet mode."]
    PACKET,
    #[doc = "Test_Force_Enable."]
    FORCE,
}
impl TSTCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTCTLW::DISABLE => 0,
            TSTCTLW::J => 1,
            TSTCTLW::K => 2,
            TSTCTLW::SE0NAK => 3,
            TSTCTLW::PACKET => 4,
            TSTCTLW::FORCE => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Test mode disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTCTLW::DISABLE)
    }
    #[doc = "Test_J mode."]
    #[inline]
    pub fn j(self) -> &'a mut W {
        self.variant(TSTCTLW::J)
    }
    #[doc = "Test_K mode."]
    #[inline]
    pub fn k(self) -> &'a mut W {
        self.variant(TSTCTLW::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline]
    pub fn se0nak(self) -> &'a mut W {
        self.variant(TSTCTLW::SE0NAK)
    }
    #[doc = "Test_Packet mode."]
    #[inline]
    pub fn packet(self) -> &'a mut W {
        self.variant(TSTCTLW::PACKET)
    }
    #[doc = "Test_Force_Enable."]
    #[inline]
    pub fn force(self) -> &'a mut W {
        self.variant(TSTCTLW::FORCE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SGNPINNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _SGNPINNAKW<'a> {
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
pub struct _CGNPINNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _CGNPINNAKW<'a> {
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
#[doc = r" Proxy"]
pub struct _SGOUTNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _SGOUTNAKW<'a> {
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
pub struct _CGOUTNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _CGOUTNAKW<'a> {
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
pub struct _PWRONPRGDONEW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRONPRGDONEW<'a> {
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
pub struct _IGNRFRMNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _IGNRFRMNUMW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NAKONBBLEW<'a> {
    w: &'a mut W,
}
impl<'a> _NAKONBBLEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline]
    pub fn rmtwkupsig(&self) -> RMTWKUPSIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RMTWKUPSIGR { bits }
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline]
    pub fn sftdiscon(&self) -> SFTDISCONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTDISCONR { bits }
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline]
    pub fn gnpinnaksts(&self) -> GNPINNAKSTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GNPINNAKSTSR { bits }
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline]
    pub fn goutnaksts(&self) -> GOUTNAKSTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GOUTNAKSTSR { bits }
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline]
    pub fn tstctl(&self) -> TSTCTLR {
        TSTCTLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline]
    pub fn pwronprgdone(&self) -> PWRONPRGDONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWRONPRGDONER { bits }
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline]
    pub fn ignrfrmnum(&self) -> IGNRFRMNUMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IGNRFRMNUMR { bits }
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline]
    pub fn nakonbble(&self) -> NAKONBBLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NAKONBBLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline]
    pub fn rmtwkupsig(&mut self) -> _RMTWKUPSIGW {
        _RMTWKUPSIGW { w: self }
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline]
    pub fn sftdiscon(&mut self) -> _SFTDISCONW {
        _SFTDISCONW { w: self }
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline]
    pub fn tstctl(&mut self) -> _TSTCTLW {
        _TSTCTLW { w: self }
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline]
    pub fn sgnpinnak(&mut self) -> _SGNPINNAKW {
        _SGNPINNAKW { w: self }
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline]
    pub fn cgnpinnak(&mut self) -> _CGNPINNAKW {
        _CGNPINNAKW { w: self }
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline]
    pub fn sgoutnak(&mut self) -> _SGOUTNAKW {
        _SGOUTNAKW { w: self }
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline]
    pub fn cgoutnak(&mut self) -> _CGOUTNAKW {
        _CGOUTNAKW { w: self }
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline]
    pub fn pwronprgdone(&mut self) -> _PWRONPRGDONEW {
        _PWRONPRGDONEW { w: self }
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline]
    pub fn ignrfrmnum(&mut self) -> _IGNRFRMNUMW {
        _IGNRFRMNUMW { w: self }
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline]
    pub fn nakonbble(&mut self) -> _NAKONBBLEW {
        _NAKONBBLEW { w: self }
    }
}
