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
pub struct SYNCR {
    bits: bool,
}
impl SYNCR {
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
pub struct LOOPBKR {
    bits: bool,
}
impl LOOPBKR {
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
pub struct CCENR {
    bits: bool,
}
impl CCENR {
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
pub struct MPMR {
    bits: bool,
}
impl MPMR {
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
pub struct MPABR {
    bits: bool,
}
impl MPABR {
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
#[doc = "Possible values of the field `OVS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSR {
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    X16,
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    X8,
    #[doc = "6X oversampling in asynchronous mode"]
    X6,
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    X4,
}
impl OVSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OVSR::X16 => 0,
            OVSR::X8 => 1,
            OVSR::X6 => 2,
            OVSR::X4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OVSR {
        match value {
            0 => OVSR::X16,
            1 => OVSR::X8,
            2 => OVSR::X6,
            3 => OVSR::X4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline]
    pub fn is_x16(&self) -> bool {
        *self == OVSR::X16
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline]
    pub fn is_x8(&self) -> bool {
        *self == OVSR::X8
    }
    #[doc = "Checks if the value of the field is `X6`"]
    #[inline]
    pub fn is_x6(&self) -> bool {
        *self == OVSR::X6
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline]
    pub fn is_x4(&self) -> bool {
        *self == OVSR::X4
    }
}
#[doc = r" Value of the field"]
pub struct CLKPOLR {
    bits: bool,
}
impl CLKPOLR {
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
pub struct CLKPHAR {
    bits: bool,
}
impl CLKPHAR {
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
pub struct MSBFR {
    bits: bool,
}
impl MSBFR {
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
pub struct CSMAR {
    bits: bool,
}
impl CSMAR {
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
pub struct TXBILR {
    bits: bool,
}
impl TXBILR {
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
pub struct RXINVR {
    bits: bool,
}
impl RXINVR {
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
pub struct TXINVR {
    bits: bool,
}
impl TXINVR {
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
pub struct CSINVR {
    bits: bool,
}
impl CSINVR {
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
pub struct AUTOCSR {
    bits: bool,
}
impl AUTOCSR {
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
pub struct AUTOTRIR {
    bits: bool,
}
impl AUTOTRIR {
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
pub struct SCMODER {
    bits: bool,
}
impl SCMODER {
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
pub struct SCRETRANSR {
    bits: bool,
}
impl SCRETRANSR {
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
pub struct SKIPPERRFR {
    bits: bool,
}
impl SKIPPERRFR {
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
pub struct BIT8DVR {
    bits: bool,
}
impl BIT8DVR {
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
pub struct ERRSDMAR {
    bits: bool,
}
impl ERRSDMAR {
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
pub struct ERRSRXR {
    bits: bool,
}
impl ERRSRXR {
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
pub struct ERRSTXR {
    bits: bool,
}
impl ERRSTXR {
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
pub struct SSSEARLYR {
    bits: bool,
}
impl SSSEARLYR {
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
#[doc = "Possible values of the field `TXDELAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDELAYR {
    #[doc = "Frames are transmitted immediately"]
    NONE,
    #[doc = "Transmission of new frames are delayed by a single baud period"]
    SINGLE,
    #[doc = "Transmission of new frames are delayed by two baud periods"]
    DOUBLE,
    #[doc = "Transmission of new frames are delayed by three baud periods"]
    TRIPLE,
}
impl TXDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXDELAYR::NONE => 0,
            TXDELAYR::SINGLE => 1,
            TXDELAYR::DOUBLE => 2,
            TXDELAYR::TRIPLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXDELAYR {
        match value {
            0 => TXDELAYR::NONE,
            1 => TXDELAYR::SINGLE,
            2 => TXDELAYR::DOUBLE,
            3 => TXDELAYR::TRIPLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == TXDELAYR::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == TXDELAYR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline]
    pub fn is_double(&self) -> bool {
        *self == TXDELAYR::DOUBLE
    }
    #[doc = "Checks if the value of the field is `TRIPLE`"]
    #[inline]
    pub fn is_triple(&self) -> bool {
        *self == TXDELAYR::TRIPLE
    }
}
#[doc = r" Value of the field"]
pub struct BYTESWAPR {
    bits: bool,
}
impl BYTESWAPR {
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
pub struct AUTOTXR {
    bits: bool,
}
impl AUTOTXR {
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
pub struct MVDISR {
    bits: bool,
}
impl MVDISR {
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
pub struct SMSDELAYR {
    bits: bool,
}
impl SMSDELAYR {
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
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
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
pub struct _LOOPBKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPBKW<'a> {
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
pub struct _CCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCENW<'a> {
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
pub struct _MPMW<'a> {
    w: &'a mut W,
}
impl<'a> _MPMW<'a> {
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
pub struct _MPABW<'a> {
    w: &'a mut W,
}
impl<'a> _MPABW<'a> {
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
#[doc = "Values that can be written to the field `OVS`"]
pub enum OVSW {
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    X16,
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    X8,
    #[doc = "6X oversampling in asynchronous mode"]
    X6,
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    X4,
}
impl OVSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OVSW::X16 => 0,
            OVSW::X8 => 1,
            OVSW::X6 => 2,
            OVSW::X4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVSW<'a> {
    w: &'a mut W,
}
impl<'a> _OVSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVSW::X16)
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVSW::X8)
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline]
    pub fn x6(self) -> &'a mut W {
        self.variant(OVSW::X6)
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVSW::X4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKPOLW<'a> {
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
pub struct _CLKPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKPHAW<'a> {
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
pub struct _MSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBFW<'a> {
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
pub struct _CSMAW<'a> {
    w: &'a mut W,
}
impl<'a> _CSMAW<'a> {
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
pub struct _TXBILW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBILW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXINVW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TXINVW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSINVW<'a> {
    w: &'a mut W,
}
impl<'a> _CSINVW<'a> {
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
pub struct _AUTOCSW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOCSW<'a> {
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
pub struct _AUTOTRIW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOTRIW<'a> {
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
pub struct _SCMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SCMODEW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCRETRANSW<'a> {
    w: &'a mut W,
}
impl<'a> _SCRETRANSW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SKIPPERRFW<'a> {
    w: &'a mut W,
}
impl<'a> _SKIPPERRFW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BIT8DVW<'a> {
    w: &'a mut W,
}
impl<'a> _BIT8DVW<'a> {
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
pub struct _ERRSDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRSDMAW<'a> {
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
pub struct _ERRSRXW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRSRXW<'a> {
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
#[doc = r" Proxy"]
pub struct _ERRSTXW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRSTXW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSSEARLYW<'a> {
    w: &'a mut W,
}
impl<'a> _SSSEARLYW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXDELAY`"]
pub enum TXDELAYW {
    #[doc = "Frames are transmitted immediately"]
    NONE,
    #[doc = "Transmission of new frames are delayed by a single baud period"]
    SINGLE,
    #[doc = "Transmission of new frames are delayed by two baud periods"]
    DOUBLE,
    #[doc = "Transmission of new frames are delayed by three baud periods"]
    TRIPLE,
}
impl TXDELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXDELAYW::NONE => 0,
            TXDELAYW::SINGLE => 1,
            TXDELAYW::DOUBLE => 2,
            TXDELAYW::TRIPLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDELAYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Frames are transmitted immediately"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(TXDELAYW::NONE)
    }
    #[doc = "Transmission of new frames are delayed by a single baud period"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(TXDELAYW::SINGLE)
    }
    #[doc = "Transmission of new frames are delayed by two baud periods"]
    #[inline]
    pub fn double(self) -> &'a mut W {
        self.variant(TXDELAYW::DOUBLE)
    }
    #[doc = "Transmission of new frames are delayed by three baud periods"]
    #[inline]
    pub fn triple(self) -> &'a mut W {
        self.variant(TXDELAYW::TRIPLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYTESWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _BYTESWAPW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOTXW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOTXW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MVDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MVDISW<'a> {
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
pub struct _SMSDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _SMSDELAYW<'a> {
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
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNCR { bits }
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline]
    pub fn loopbk(&self) -> LOOPBKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOOPBKR { bits }
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline]
    pub fn ccen(&self) -> CCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CCENR { bits }
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline]
    pub fn mpm(&self) -> MPMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MPMR { bits }
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline]
    pub fn mpab(&self) -> MPABR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MPABR { bits }
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline]
    pub fn ovs(&self) -> OVSR {
        OVSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline]
    pub fn clkpol(&self) -> CLKPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKPOLR { bits }
    }
    #[doc = "Bit 9 - Clock Edge For Setup/Sample"]
    #[inline]
    pub fn clkpha(&self) -> CLKPHAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKPHAR { bits }
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline]
    pub fn msbf(&self) -> MSBFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSBFR { bits }
    }
    #[doc = "Bit 11 - Action On Slave-Select In Master Mode"]
    #[inline]
    pub fn csma(&self) -> CSMAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSMAR { bits }
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline]
    pub fn txbil(&self) -> TXBILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXBILR { bits }
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline]
    pub fn rxinv(&self) -> RXINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXINVR { bits }
    }
    #[doc = "Bit 14 - Transmitter output Invert"]
    #[inline]
    pub fn txinv(&self) -> TXINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXINVR { bits }
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline]
    pub fn csinv(&self) -> CSINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSINVR { bits }
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline]
    pub fn autocs(&self) -> AUTOCSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOCSR { bits }
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline]
    pub fn autotri(&self) -> AUTOTRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOTRIR { bits }
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline]
    pub fn scmode(&self) -> SCMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCMODER { bits }
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline]
    pub fn scretrans(&self) -> SCRETRANSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCRETRANSR { bits }
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline]
    pub fn skipperrf(&self) -> SKIPPERRFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SKIPPERRFR { bits }
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline]
    pub fn bit8dv(&self) -> BIT8DVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BIT8DVR { bits }
    }
    #[doc = "Bit 22 - Halt DMA On Error"]
    #[inline]
    pub fn errsdma(&self) -> ERRSDMAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRSDMAR { bits }
    }
    #[doc = "Bit 23 - Disable RX On Error"]
    #[inline]
    pub fn errsrx(&self) -> ERRSRXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRSRXR { bits }
    }
    #[doc = "Bit 24 - Disable TX On Error"]
    #[inline]
    pub fn errstx(&self) -> ERRSTXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRSTXR { bits }
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline]
    pub fn sssearly(&self) -> SSSEARLYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SSSEARLYR { bits }
    }
    #[doc = "Bits 26:27 - TX Delay Transmission"]
    #[inline]
    pub fn txdelay(&self) -> TXDELAYR {
        TXDELAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Byteswap In Double Accesses"]
    #[inline]
    pub fn byteswap(&self) -> BYTESWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYTESWAPR { bits }
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline]
    pub fn autotx(&self) -> AUTOTXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOTXR { bits }
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline]
    pub fn mvdis(&self) -> MVDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MVDISR { bits }
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline]
    pub fn smsdelay(&self) -> SMSDELAYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SMSDELAYR { bits }
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
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline]
    pub fn loopbk(&mut self) -> _LOOPBKW {
        _LOOPBKW { w: self }
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline]
    pub fn ccen(&mut self) -> _CCENW {
        _CCENW { w: self }
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline]
    pub fn mpm(&mut self) -> _MPMW {
        _MPMW { w: self }
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline]
    pub fn mpab(&mut self) -> _MPABW {
        _MPABW { w: self }
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline]
    pub fn ovs(&mut self) -> _OVSW {
        _OVSW { w: self }
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline]
    pub fn clkpol(&mut self) -> _CLKPOLW {
        _CLKPOLW { w: self }
    }
    #[doc = "Bit 9 - Clock Edge For Setup/Sample"]
    #[inline]
    pub fn clkpha(&mut self) -> _CLKPHAW {
        _CLKPHAW { w: self }
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline]
    pub fn msbf(&mut self) -> _MSBFW {
        _MSBFW { w: self }
    }
    #[doc = "Bit 11 - Action On Slave-Select In Master Mode"]
    #[inline]
    pub fn csma(&mut self) -> _CSMAW {
        _CSMAW { w: self }
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline]
    pub fn txbil(&mut self) -> _TXBILW {
        _TXBILW { w: self }
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline]
    pub fn rxinv(&mut self) -> _RXINVW {
        _RXINVW { w: self }
    }
    #[doc = "Bit 14 - Transmitter output Invert"]
    #[inline]
    pub fn txinv(&mut self) -> _TXINVW {
        _TXINVW { w: self }
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline]
    pub fn csinv(&mut self) -> _CSINVW {
        _CSINVW { w: self }
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline]
    pub fn autocs(&mut self) -> _AUTOCSW {
        _AUTOCSW { w: self }
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline]
    pub fn autotri(&mut self) -> _AUTOTRIW {
        _AUTOTRIW { w: self }
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline]
    pub fn scmode(&mut self) -> _SCMODEW {
        _SCMODEW { w: self }
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline]
    pub fn scretrans(&mut self) -> _SCRETRANSW {
        _SCRETRANSW { w: self }
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline]
    pub fn skipperrf(&mut self) -> _SKIPPERRFW {
        _SKIPPERRFW { w: self }
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline]
    pub fn bit8dv(&mut self) -> _BIT8DVW {
        _BIT8DVW { w: self }
    }
    #[doc = "Bit 22 - Halt DMA On Error"]
    #[inline]
    pub fn errsdma(&mut self) -> _ERRSDMAW {
        _ERRSDMAW { w: self }
    }
    #[doc = "Bit 23 - Disable RX On Error"]
    #[inline]
    pub fn errsrx(&mut self) -> _ERRSRXW {
        _ERRSRXW { w: self }
    }
    #[doc = "Bit 24 - Disable TX On Error"]
    #[inline]
    pub fn errstx(&mut self) -> _ERRSTXW {
        _ERRSTXW { w: self }
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline]
    pub fn sssearly(&mut self) -> _SSSEARLYW {
        _SSSEARLYW { w: self }
    }
    #[doc = "Bits 26:27 - TX Delay Transmission"]
    #[inline]
    pub fn txdelay(&mut self) -> _TXDELAYW {
        _TXDELAYW { w: self }
    }
    #[doc = "Bit 28 - Byteswap In Double Accesses"]
    #[inline]
    pub fn byteswap(&mut self) -> _BYTESWAPW {
        _BYTESWAPW { w: self }
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline]
    pub fn autotx(&mut self) -> _AUTOTXW {
        _AUTOTXW { w: self }
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline]
    pub fn mvdis(&mut self) -> _MVDISW {
        _MVDISW { w: self }
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline]
    pub fn smsdelay(&mut self) -> _SMSDELAYW {
        _SMSDELAYW { w: self }
    }
}
