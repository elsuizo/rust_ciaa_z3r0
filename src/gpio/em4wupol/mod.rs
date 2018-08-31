#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EM4WUPOL {
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
#[doc = "Possible values of the field `EM4WUPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM4WUPOLR {
    #[doc = "Determines polarity on pin A0"]
    A0,
    #[doc = "Determines polarity on pin C9"]
    C9,
    #[doc = "Determines polarity on pin F1"]
    F1,
    #[doc = "Determines polarity on pin F2"]
    F2,
    #[doc = "Determines polarity on pin E13"]
    E13,
    #[doc = "Determines polarity on pin C4"]
    C4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM4WUPOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM4WUPOLR::A0 => 1,
            EM4WUPOLR::C9 => 4,
            EM4WUPOLR::F1 => 8,
            EM4WUPOLR::F2 => 16,
            EM4WUPOLR::E13 => 32,
            EM4WUPOLR::C4 => 64,
            EM4WUPOLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM4WUPOLR {
        match value {
            1 => EM4WUPOLR::A0,
            4 => EM4WUPOLR::C9,
            8 => EM4WUPOLR::F1,
            16 => EM4WUPOLR::F2,
            32 => EM4WUPOLR::E13,
            64 => EM4WUPOLR::C4,
            i => EM4WUPOLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUPOLR::A0
    }
    #[doc = "Checks if the value of the field is `C9`"]
    #[inline]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUPOLR::C9
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUPOLR::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUPOLR::F2
    }
    #[doc = "Checks if the value of the field is `E13`"]
    #[inline]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUPOLR::E13
    }
    #[doc = "Checks if the value of the field is `C4`"]
    #[inline]
    pub fn is_c4(&self) -> bool {
        *self == EM4WUPOLR::C4
    }
}
#[doc = "Values that can be written to the field `EM4WUPOL`"]
pub enum EM4WUPOLW {
    #[doc = "Determines polarity on pin A0"]
    A0,
    #[doc = "Determines polarity on pin C9"]
    C9,
    #[doc = "Determines polarity on pin F1"]
    F1,
    #[doc = "Determines polarity on pin F2"]
    F2,
    #[doc = "Determines polarity on pin E13"]
    E13,
    #[doc = "Determines polarity on pin C4"]
    C4,
}
impl EM4WUPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EM4WUPOLW::A0 => 1,
            EM4WUPOLW::C9 => 4,
            EM4WUPOLW::F1 => 8,
            EM4WUPOLW::F2 => 16,
            EM4WUPOLW::E13 => 32,
            EM4WUPOLW::C4 => 64,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EM4WUPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _EM4WUPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EM4WUPOLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Determines polarity on pin A0"]
    #[inline]
    pub fn a0(self) -> &'a mut W {
        self.variant(EM4WUPOLW::A0)
    }
    #[doc = "Determines polarity on pin C9"]
    #[inline]
    pub fn c9(self) -> &'a mut W {
        self.variant(EM4WUPOLW::C9)
    }
    #[doc = "Determines polarity on pin F1"]
    #[inline]
    pub fn f1(self) -> &'a mut W {
        self.variant(EM4WUPOLW::F1)
    }
    #[doc = "Determines polarity on pin F2"]
    #[inline]
    pub fn f2(self) -> &'a mut W {
        self.variant(EM4WUPOLW::F2)
    }
    #[doc = "Determines polarity on pin E13"]
    #[inline]
    pub fn e13(self) -> &'a mut W {
        self.variant(EM4WUPOLW::E13)
    }
    #[doc = "Determines polarity on pin C4"]
    #[inline]
    pub fn c4(self) -> &'a mut W {
        self.variant(EM4WUPOLW::C4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
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
    #[doc = "Bits 0:6 - EM4 Wake-up Polarity"]
    #[inline]
    pub fn em4wupol(&self) -> EM4WUPOLR {
        EM4WUPOLR::_from({
            const MASK: u8 = 127;
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
    #[doc = "Bits 0:6 - EM4 Wake-up Polarity"]
    #[inline]
    pub fn em4wupol(&mut self) -> _EM4WUPOLW {
        _EM4WUPOLW { w: self }
    }
}
