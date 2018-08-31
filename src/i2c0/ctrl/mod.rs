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
pub struct SLAVER {
    bits: bool,
}
impl SLAVER {
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
pub struct AUTOACKR {
    bits: bool,
}
impl AUTOACKR {
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
pub struct AUTOSER {
    bits: bool,
}
impl AUTOSER {
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
pub struct AUTOSNR {
    bits: bool,
}
impl AUTOSNR {
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
pub struct ARBDISR {
    bits: bool,
}
impl ARBDISR {
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
pub struct GCAMENR {
    bits: bool,
}
impl GCAMENR {
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
#[doc = "Possible values of the field `CLHR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLHRR {
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    STANDARD,
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    ASYMMETRIC,
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    FAST,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLHRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLHRR::STANDARD => 0,
            CLHRR::ASYMMETRIC => 1,
            CLHRR::FAST => 2,
            CLHRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLHRR {
        match value {
            0 => CLHRR::STANDARD,
            1 => CLHRR::ASYMMETRIC,
            2 => CLHRR::FAST,
            i => CLHRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == CLHRR::STANDARD
    }
    #[doc = "Checks if the value of the field is `ASYMMETRIC`"]
    #[inline]
    pub fn is_asymmetric(&self) -> bool {
        *self == CLHRR::ASYMMETRIC
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline]
    pub fn is_fast(&self) -> bool {
        *self == CLHRR::FAST
    }
}
#[doc = "Possible values of the field `BITO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITOR {
    #[doc = "Timeout disabled"]
    OFF,
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40PCC,
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80PCC,
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160PCC,
}
impl BITOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BITOR::OFF => 0,
            BITOR::_40PCC => 1,
            BITOR::_80PCC => 2,
            BITOR::_160PCC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BITOR {
        match value {
            0 => BITOR::OFF,
            1 => BITOR::_40PCC,
            2 => BITOR::_80PCC,
            3 => BITOR::_160PCC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == BITOR::OFF
    }
    #[doc = "Checks if the value of the field is `_40PCC`"]
    #[inline]
    pub fn is_40pcc(&self) -> bool {
        *self == BITOR::_40PCC
    }
    #[doc = "Checks if the value of the field is `_80PCC`"]
    #[inline]
    pub fn is_80pcc(&self) -> bool {
        *self == BITOR::_80PCC
    }
    #[doc = "Checks if the value of the field is `_160PCC`"]
    #[inline]
    pub fn is_160pcc(&self) -> bool {
        *self == BITOR::_160PCC
    }
}
#[doc = r" Value of the field"]
pub struct GIBITOR {
    bits: bool,
}
impl GIBITOR {
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
#[doc = "Possible values of the field `CLTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLTOR {
    #[doc = "Timeout disabled"]
    OFF,
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40PCC,
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80PCC,
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160PCC,
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    _320PPC,
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    _1024PPC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLTOR::OFF => 0,
            CLTOR::_40PCC => 1,
            CLTOR::_80PCC => 2,
            CLTOR::_160PCC => 3,
            CLTOR::_320PPC => 4,
            CLTOR::_1024PPC => 5,
            CLTOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLTOR {
        match value {
            0 => CLTOR::OFF,
            1 => CLTOR::_40PCC,
            2 => CLTOR::_80PCC,
            3 => CLTOR::_160PCC,
            4 => CLTOR::_320PPC,
            5 => CLTOR::_1024PPC,
            i => CLTOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == CLTOR::OFF
    }
    #[doc = "Checks if the value of the field is `_40PCC`"]
    #[inline]
    pub fn is_40pcc(&self) -> bool {
        *self == CLTOR::_40PCC
    }
    #[doc = "Checks if the value of the field is `_80PCC`"]
    #[inline]
    pub fn is_80pcc(&self) -> bool {
        *self == CLTOR::_80PCC
    }
    #[doc = "Checks if the value of the field is `_160PCC`"]
    #[inline]
    pub fn is_160pcc(&self) -> bool {
        *self == CLTOR::_160PCC
    }
    #[doc = "Checks if the value of the field is `_320PPC`"]
    #[inline]
    pub fn is_320ppc(&self) -> bool {
        *self == CLTOR::_320PPC
    }
    #[doc = "Checks if the value of the field is `_1024PPC`"]
    #[inline]
    pub fn is_1024ppc(&self) -> bool {
        *self == CLTOR::_1024PPC
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
pub struct _SLAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVEW<'a> {
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
pub struct _AUTOACKW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOACKW<'a> {
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
pub struct _AUTOSEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSEW<'a> {
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
pub struct _AUTOSNW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSNW<'a> {
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
pub struct _ARBDISW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBDISW<'a> {
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
pub struct _GCAMENW<'a> {
    w: &'a mut W,
}
impl<'a> _GCAMENW<'a> {
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
#[doc = "Values that can be written to the field `CLHR`"]
pub enum CLHRW {
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    STANDARD,
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    ASYMMETRIC,
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    FAST,
}
impl CLHRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLHRW::STANDARD => 0,
            CLHRW::ASYMMETRIC => 1,
            CLHRW::FAST => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLHRW<'a> {
    w: &'a mut W,
}
impl<'a> _CLHRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLHRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(CLHRW::STANDARD)
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    #[inline]
    pub fn asymmetric(self) -> &'a mut W {
        self.variant(CLHRW::ASYMMETRIC)
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    #[inline]
    pub fn fast(self) -> &'a mut W {
        self.variant(CLHRW::FAST)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BITO`"]
pub enum BITOW {
    #[doc = "Timeout disabled"]
    OFF,
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40PCC,
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80PCC,
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160PCC,
}
impl BITOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BITOW::OFF => 0,
            BITOW::_40PCC => 1,
            BITOW::_80PCC => 2,
            BITOW::_160PCC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITOW<'a> {
    w: &'a mut W,
}
impl<'a> _BITOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timeout disabled"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(BITOW::OFF)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline]
    pub fn _40pcc(self) -> &'a mut W {
        self.variant(BITOW::_40PCC)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline]
    pub fn _80pcc(self) -> &'a mut W {
        self.variant(BITOW::_80PCC)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline]
    pub fn _160pcc(self) -> &'a mut W {
        self.variant(BITOW::_160PCC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GIBITOW<'a> {
    w: &'a mut W,
}
impl<'a> _GIBITOW<'a> {
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
#[doc = "Values that can be written to the field `CLTO`"]
pub enum CLTOW {
    #[doc = "Timeout disabled"]
    OFF,
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40PCC,
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80PCC,
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160PCC,
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    _320PPC,
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    _1024PPC,
}
impl CLTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLTOW::OFF => 0,
            CLTOW::_40PCC => 1,
            CLTOW::_80PCC => 2,
            CLTOW::_160PCC => 3,
            CLTOW::_320PPC => 4,
            CLTOW::_1024PPC => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLTOW<'a> {
    w: &'a mut W,
}
impl<'a> _CLTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLTOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timeout disabled"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(CLTOW::OFF)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline]
    pub fn _40pcc(self) -> &'a mut W {
        self.variant(CLTOW::_40PCC)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline]
    pub fn _80pcc(self) -> &'a mut W {
        self.variant(CLTOW::_80PCC)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline]
    pub fn _160pcc(self) -> &'a mut W {
        self.variant(CLTOW::_160PCC)
    }
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    #[inline]
    pub fn _320ppc(self) -> &'a mut W {
        self.variant(CLTOW::_320PPC)
    }
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    #[inline]
    pub fn _1024ppc(self) -> &'a mut W {
        self.variant(CLTOW::_1024PPC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 0 - I2C Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 1 - Addressable as Slave"]
    #[inline]
    pub fn slave(&self) -> SLAVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLAVER { bits }
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline]
    pub fn autoack(&self) -> AUTOACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOACKR { bits }
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline]
    pub fn autose(&self) -> AUTOSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOSER { bits }
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline]
    pub fn autosn(&self) -> AUTOSNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOSNR { bits }
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline]
    pub fn arbdis(&self) -> ARBDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARBDISR { bits }
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline]
    pub fn gcamen(&self) -> GCAMENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GCAMENR { bits }
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline]
    pub fn clhr(&self) -> CLHRR {
        CLHRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline]
    pub fn bito(&self) -> BITOR {
        BITOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline]
    pub fn gibito(&self) -> GIBITOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GIBITOR { bits }
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline]
    pub fn clto(&self) -> CLTOR {
        CLTOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - I2C Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Addressable as Slave"]
    #[inline]
    pub fn slave(&mut self) -> _SLAVEW {
        _SLAVEW { w: self }
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline]
    pub fn autoack(&mut self) -> _AUTOACKW {
        _AUTOACKW { w: self }
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline]
    pub fn autose(&mut self) -> _AUTOSEW {
        _AUTOSEW { w: self }
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline]
    pub fn autosn(&mut self) -> _AUTOSNW {
        _AUTOSNW { w: self }
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline]
    pub fn arbdis(&mut self) -> _ARBDISW {
        _ARBDISW { w: self }
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline]
    pub fn gcamen(&mut self) -> _GCAMENW {
        _GCAMENW { w: self }
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline]
    pub fn clhr(&mut self) -> _CLHRW {
        _CLHRW { w: self }
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline]
    pub fn bito(&mut self) -> _BITOW {
        _BITOW { w: self }
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline]
    pub fn gibito(&mut self) -> _GIBITOW {
        _GIBITOW { w: self }
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline]
    pub fn clto(&mut self) -> _CLTOW {
        _CLTOW { w: self }
    }
}
