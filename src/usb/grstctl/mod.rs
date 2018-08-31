#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GRSTCTL {
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
pub struct CSFTRSTR {
    bits: bool,
}
impl CSFTRSTR {
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
pub struct PIUFSSFTRSTR {
    bits: bool,
}
impl PIUFSSFTRSTR {
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
pub struct RXFFLSHR {
    bits: bool,
}
impl RXFFLSHR {
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
pub struct TXFFLSHR {
    bits: bool,
}
impl TXFFLSHR {
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
#[doc = "Possible values of the field `TXFNUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFNUMR {
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    F0,
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    F1,
    #[doc = "Device mode: TXFIFO 2 flush."]
    F2,
    #[doc = "Device mode: TXFIFO 3 flush."]
    F3,
    #[doc = "Device mode: TXFIFO 4 flush."]
    F4,
    #[doc = "Device mode: TXFIFO 5 flush."]
    F5,
    #[doc = "Device mode: TXFIFO 6 flush."]
    F6,
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    FALL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXFNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXFNUMR::F0 => 0,
            TXFNUMR::F1 => 1,
            TXFNUMR::F2 => 2,
            TXFNUMR::F3 => 3,
            TXFNUMR::F4 => 4,
            TXFNUMR::F5 => 5,
            TXFNUMR::F6 => 6,
            TXFNUMR::FALL => 16,
            TXFNUMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXFNUMR {
        match value {
            0 => TXFNUMR::F0,
            1 => TXFNUMR::F1,
            2 => TXFNUMR::F2,
            3 => TXFNUMR::F3,
            4 => TXFNUMR::F4,
            5 => TXFNUMR::F5,
            6 => TXFNUMR::F6,
            16 => TXFNUMR::FALL,
            i => TXFNUMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `F0`"]
    #[inline]
    pub fn is_f0(&self) -> bool {
        *self == TXFNUMR::F0
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline]
    pub fn is_f1(&self) -> bool {
        *self == TXFNUMR::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline]
    pub fn is_f2(&self) -> bool {
        *self == TXFNUMR::F2
    }
    #[doc = "Checks if the value of the field is `F3`"]
    #[inline]
    pub fn is_f3(&self) -> bool {
        *self == TXFNUMR::F3
    }
    #[doc = "Checks if the value of the field is `F4`"]
    #[inline]
    pub fn is_f4(&self) -> bool {
        *self == TXFNUMR::F4
    }
    #[doc = "Checks if the value of the field is `F5`"]
    #[inline]
    pub fn is_f5(&self) -> bool {
        *self == TXFNUMR::F5
    }
    #[doc = "Checks if the value of the field is `F6`"]
    #[inline]
    pub fn is_f6(&self) -> bool {
        *self == TXFNUMR::F6
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == TXFNUMR::FALL
    }
}
#[doc = r" Value of the field"]
pub struct DMAREQR {
    bits: bool,
}
impl DMAREQR {
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
pub struct AHBIDLER {
    bits: bool,
}
impl AHBIDLER {
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
pub struct _CSFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CSFTRSTW<'a> {
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
pub struct _PIUFSSFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PIUFSSFTRSTW<'a> {
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
pub struct _RXFFLSHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFFLSHW<'a> {
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
#[doc = r" Proxy"]
pub struct _TXFFLSHW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFFLSHW<'a> {
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
#[doc = "Values that can be written to the field `TXFNUM`"]
pub enum TXFNUMW {
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    F0,
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    F1,
    #[doc = "Device mode: TXFIFO 2 flush."]
    F2,
    #[doc = "Device mode: TXFIFO 3 flush."]
    F3,
    #[doc = "Device mode: TXFIFO 4 flush."]
    F4,
    #[doc = "Device mode: TXFIFO 5 flush."]
    F5,
    #[doc = "Device mode: TXFIFO 6 flush."]
    F6,
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    FALL,
}
impl TXFNUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXFNUMW::F0 => 0,
            TXFNUMW::F1 => 1,
            TXFNUMW::F2 => 2,
            TXFNUMW::F3 => 3,
            TXFNUMW::F4 => 4,
            TXFNUMW::F5 => 5,
            TXFNUMW::F6 => 6,
            TXFNUMW::FALL => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFNUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFNUMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    #[inline]
    pub fn f0(self) -> &'a mut W {
        self.variant(TXFNUMW::F0)
    }
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    #[inline]
    pub fn f1(self) -> &'a mut W {
        self.variant(TXFNUMW::F1)
    }
    #[doc = "Device mode: TXFIFO 2 flush."]
    #[inline]
    pub fn f2(self) -> &'a mut W {
        self.variant(TXFNUMW::F2)
    }
    #[doc = "Device mode: TXFIFO 3 flush."]
    #[inline]
    pub fn f3(self) -> &'a mut W {
        self.variant(TXFNUMW::F3)
    }
    #[doc = "Device mode: TXFIFO 4 flush."]
    #[inline]
    pub fn f4(self) -> &'a mut W {
        self.variant(TXFNUMW::F4)
    }
    #[doc = "Device mode: TXFIFO 5 flush."]
    #[inline]
    pub fn f5(self) -> &'a mut W {
        self.variant(TXFNUMW::F5)
    }
    #[doc = "Device mode: TXFIFO 6 flush."]
    #[inline]
    pub fn f6(self) -> &'a mut W {
        self.variant(TXFNUMW::F6)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(TXFNUMW::FALL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline]
    pub fn csftrst(&self) -> CSFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSFTRSTR { bits }
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline]
    pub fn piufssftrst(&self) -> PIUFSSFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIUFSSFTRSTR { bits }
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline]
    pub fn rxfflsh(&self) -> RXFFLSHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFFLSHR { bits }
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline]
    pub fn txfflsh(&self) -> TXFFLSHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFFLSHR { bits }
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline]
    pub fn txfnum(&self) -> TXFNUMR {
        TXFNUMR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline]
    pub fn dmareq(&self) -> DMAREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAREQR { bits }
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline]
    pub fn ahbidle(&self) -> AHBIDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AHBIDLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline]
    pub fn csftrst(&mut self) -> _CSFTRSTW {
        _CSFTRSTW { w: self }
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline]
    pub fn piufssftrst(&mut self) -> _PIUFSSFTRSTW {
        _PIUFSSFTRSTW { w: self }
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline]
    pub fn rxfflsh(&mut self) -> _RXFFLSHW {
        _RXFFLSHW { w: self }
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline]
    pub fn txfflsh(&mut self) -> _TXFFLSHW {
        _TXFFLSHW { w: self }
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline]
    pub fn txfnum(&mut self) -> _TXFNUMW {
        _TXFNUMW { w: self }
    }
}
