#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PC_CTRL {
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
#[doc = "Possible values of the field `DRIVEMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRIVEMODER {
    #[doc = "6 mA drive current"]
    STANDARD,
    #[doc = "0.1 mA drive current"]
    LOWEST,
    #[doc = "20 mA drive current"]
    HIGH,
    #[doc = "1 mA drive current"]
    LOW,
}
impl DRIVEMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRIVEMODER::STANDARD => 0,
            DRIVEMODER::LOWEST => 1,
            DRIVEMODER::HIGH => 2,
            DRIVEMODER::LOW => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRIVEMODER {
        match value {
            0 => DRIVEMODER::STANDARD,
            1 => DRIVEMODER::LOWEST,
            2 => DRIVEMODER::HIGH,
            3 => DRIVEMODER::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == DRIVEMODER::STANDARD
    }
    #[doc = "Checks if the value of the field is `LOWEST`"]
    #[inline]
    pub fn is_lowest(&self) -> bool {
        *self == DRIVEMODER::LOWEST
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == DRIVEMODER::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == DRIVEMODER::LOW
    }
}
#[doc = "Values that can be written to the field `DRIVEMODE`"]
pub enum DRIVEMODEW {
    #[doc = "6 mA drive current"]
    STANDARD,
    #[doc = "0.1 mA drive current"]
    LOWEST,
    #[doc = "20 mA drive current"]
    HIGH,
    #[doc = "1 mA drive current"]
    LOW,
}
impl DRIVEMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DRIVEMODEW::STANDARD => 0,
            DRIVEMODEW::LOWEST => 1,
            DRIVEMODEW::HIGH => 2,
            DRIVEMODEW::LOW => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRIVEMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DRIVEMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRIVEMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "6 mA drive current"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(DRIVEMODEW::STANDARD)
    }
    #[doc = "0.1 mA drive current"]
    #[inline]
    pub fn lowest(self) -> &'a mut W {
        self.variant(DRIVEMODEW::LOWEST)
    }
    #[doc = "20 mA drive current"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(DRIVEMODEW::HIGH)
    }
    #[doc = "1 mA drive current"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(DRIVEMODEW::LOW)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline]
    pub fn drivemode(&self) -> DRIVEMODER {
        DRIVEMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline]
    pub fn drivemode(&mut self) -> _DRIVEMODEW {
        _DRIVEMODEW { w: self }
    }
}
