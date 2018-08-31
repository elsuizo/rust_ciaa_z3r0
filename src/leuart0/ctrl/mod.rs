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
pub struct DATABITSR {
    bits: bool,
}
impl DATABITSR {
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
#[doc = "Possible values of the field `PARITY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYR {
    #[doc = "Parity bits are not used"]
    NONE,
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    EVEN,
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    ODD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PARITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARITYR::NONE => 0,
            PARITYR::EVEN => 2,
            PARITYR::ODD => 3,
            PARITYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARITYR {
        match value {
            0 => PARITYR::NONE,
            2 => PARITYR::EVEN,
            3 => PARITYR::ODD,
            i => PARITYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PARITYR::NONE
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == PARITYR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == PARITYR::ODD
    }
}
#[doc = r" Value of the field"]
pub struct STOPBITSR {
    bits: bool,
}
impl STOPBITSR {
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
pub struct INVR {
    bits: bool,
}
impl INVR {
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
pub struct SFUBRXR {
    bits: bool,
}
impl SFUBRXR {
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
pub struct RXDMAWUR {
    bits: bool,
}
impl RXDMAWUR {
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
pub struct TXDMAWUR {
    bits: bool,
}
impl TXDMAWUR {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATABITSW<'a> {
    w: &'a mut W,
}
impl<'a> _DATABITSW<'a> {
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
#[doc = "Values that can be written to the field `PARITY`"]
pub enum PARITYW {
    #[doc = "Parity bits are not used"]
    NONE,
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    EVEN,
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    ODD,
}
impl PARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARITYW::NONE => 0,
            PARITYW::EVEN => 2,
            PARITYW::ODD => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Parity bits are not used"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PARITYW::NONE)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(PARITYW::EVEN)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARITYW::ODD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STOPBITSW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPBITSW<'a> {
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
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
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
        const OFFSET: u8 = 6;
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SFUBRXW<'a> {
    w: &'a mut W,
}
impl<'a> _SFUBRXW<'a> {
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
        const OFFSET: u8 = 9;
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
        const OFFSET: u8 = 10;
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXDMAWUW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMAWUW<'a> {
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
pub struct _TXDMAWUW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAWUW<'a> {
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline]
    pub fn autotri(&self) -> AUTOTRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOTRIR { bits }
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline]
    pub fn databits(&self) -> DATABITSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATABITSR { bits }
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline]
    pub fn parity(&self) -> PARITYR {
        PARITYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline]
    pub fn stopbits(&self) -> STOPBITSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOPBITSR { bits }
    }
    #[doc = "Bit 5 - Invert Input And Output"]
    #[inline]
    pub fn inv(&self) -> INVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVR { bits }
    }
    #[doc = "Bit 6 - Clear RX DMA On Error"]
    #[inline]
    pub fn errsdma(&self) -> ERRSDMAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRSDMAR { bits }
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline]
    pub fn loopbk(&self) -> LOOPBKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOOPBKR { bits }
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline]
    pub fn sfubrx(&self) -> SFUBRXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFUBRXR { bits }
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline]
    pub fn mpm(&self) -> MPMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MPMR { bits }
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline]
    pub fn mpab(&self) -> MPABR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MPABR { bits }
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline]
    pub fn bit8dv(&self) -> BIT8DVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BIT8DVR { bits }
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline]
    pub fn rxdmawu(&self) -> RXDMAWUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXDMAWUR { bits }
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline]
    pub fn txdmawu(&self) -> TXDMAWUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXDMAWUR { bits }
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline]
    pub fn txdelay(&self) -> TXDELAYR {
        TXDELAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - Automatic Transmitter Tristate"]
    #[inline]
    pub fn autotri(&mut self) -> _AUTOTRIW {
        _AUTOTRIW { w: self }
    }
    #[doc = "Bit 1 - Data-Bit Mode"]
    #[inline]
    pub fn databits(&mut self) -> _DATABITSW {
        _DATABITSW { w: self }
    }
    #[doc = "Bits 2:3 - Parity-Bit Mode"]
    #[inline]
    pub fn parity(&mut self) -> _PARITYW {
        _PARITYW { w: self }
    }
    #[doc = "Bit 4 - Stop-Bit Mode"]
    #[inline]
    pub fn stopbits(&mut self) -> _STOPBITSW {
        _STOPBITSW { w: self }
    }
    #[doc = "Bit 5 - Invert Input And Output"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 6 - Clear RX DMA On Error"]
    #[inline]
    pub fn errsdma(&mut self) -> _ERRSDMAW {
        _ERRSDMAW { w: self }
    }
    #[doc = "Bit 7 - Loopback Enable"]
    #[inline]
    pub fn loopbk(&mut self) -> _LOOPBKW {
        _LOOPBKW { w: self }
    }
    #[doc = "Bit 8 - Start-Frame UnBlock RX"]
    #[inline]
    pub fn sfubrx(&mut self) -> _SFUBRXW {
        _SFUBRXW { w: self }
    }
    #[doc = "Bit 9 - Multi-Processor Mode"]
    #[inline]
    pub fn mpm(&mut self) -> _MPMW {
        _MPMW { w: self }
    }
    #[doc = "Bit 10 - Multi-Processor Address-Bit"]
    #[inline]
    pub fn mpab(&mut self) -> _MPABW {
        _MPABW { w: self }
    }
    #[doc = "Bit 11 - Bit 8 Default Value"]
    #[inline]
    pub fn bit8dv(&mut self) -> _BIT8DVW {
        _BIT8DVW { w: self }
    }
    #[doc = "Bit 12 - RX DMA Wakeup"]
    #[inline]
    pub fn rxdmawu(&mut self) -> _RXDMAWUW {
        _RXDMAWUW { w: self }
    }
    #[doc = "Bit 13 - TX DMA Wakeup"]
    #[inline]
    pub fn txdmawu(&mut self) -> _TXDMAWUW {
        _TXDMAWUW { w: self }
    }
    #[doc = "Bits 14:15 - TX Delay Transmission"]
    #[inline]
    pub fn txdelay(&mut self) -> _TXDELAYW {
        _TXDELAYW { w: self }
    }
}
