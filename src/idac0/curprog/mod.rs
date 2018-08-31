#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CURPROG {
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
#[doc = "Possible values of the field `RANGESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGESELR {
    #[doc = "Current range set to 0 - 1.6 uA."]
    RANGE0,
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    RANGE1,
    #[doc = "Current range set to 0.5 - 16 uA."]
    RANGE2,
    #[doc = "Current range set to 2 - 64 uA."]
    RANGE3,
}
impl RANGESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RANGESELR::RANGE0 => 0,
            RANGESELR::RANGE1 => 1,
            RANGESELR::RANGE2 => 2,
            RANGESELR::RANGE3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RANGESELR {
        match value {
            0 => RANGESELR::RANGE0,
            1 => RANGESELR::RANGE1,
            2 => RANGESELR::RANGE2,
            3 => RANGESELR::RANGE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RANGE0`"]
    #[inline]
    pub fn is_range0(&self) -> bool {
        *self == RANGESELR::RANGE0
    }
    #[doc = "Checks if the value of the field is `RANGE1`"]
    #[inline]
    pub fn is_range1(&self) -> bool {
        *self == RANGESELR::RANGE1
    }
    #[doc = "Checks if the value of the field is `RANGE2`"]
    #[inline]
    pub fn is_range2(&self) -> bool {
        *self == RANGESELR::RANGE2
    }
    #[doc = "Checks if the value of the field is `RANGE3`"]
    #[inline]
    pub fn is_range3(&self) -> bool {
        *self == RANGESELR::RANGE3
    }
}
#[doc = r" Value of the field"]
pub struct STEPSELR {
    bits: u8,
}
impl STEPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `RANGESEL`"]
pub enum RANGESELW {
    #[doc = "Current range set to 0 - 1.6 uA."]
    RANGE0,
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    RANGE1,
    #[doc = "Current range set to 0.5 - 16 uA."]
    RANGE2,
    #[doc = "Current range set to 2 - 64 uA."]
    RANGE3,
}
impl RANGESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RANGESELW::RANGE0 => 0,
            RANGESELW::RANGE1 => 1,
            RANGESELW::RANGE2 => 2,
            RANGESELW::RANGE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RANGESELW<'a> {
    w: &'a mut W,
}
impl<'a> _RANGESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RANGESELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Current range set to 0 - 1.6 uA."]
    #[inline]
    pub fn range0(self) -> &'a mut W {
        self.variant(RANGESELW::RANGE0)
    }
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    #[inline]
    pub fn range1(self) -> &'a mut W {
        self.variant(RANGESELW::RANGE1)
    }
    #[doc = "Current range set to 0.5 - 16 uA."]
    #[inline]
    pub fn range2(self) -> &'a mut W {
        self.variant(RANGESELW::RANGE2)
    }
    #[doc = "Current range set to 2 - 64 uA."]
    #[inline]
    pub fn range3(self) -> &'a mut W {
        self.variant(RANGESELW::RANGE3)
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
pub struct _STEPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _STEPSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline]
    pub fn rangesel(&self) -> RANGESELR {
        RANGESELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline]
    pub fn stepsel(&self) -> STEPSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STEPSELR { bits }
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
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline]
    pub fn rangesel(&mut self) -> _RANGESELW {
        _RANGESELW { w: self }
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline]
    pub fn stepsel(&mut self) -> _STEPSELW {
        _STEPSELW { w: self }
    }
}
