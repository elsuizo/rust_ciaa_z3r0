#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EM4WUEN {
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
#[doc = "Possible values of the field `EM4WUEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM4WUENR {
    #[doc = "Enable em4 wakeup on pin A0"]
    A0,
    #[doc = "Enable em4 wakeup on pin C9"]
    C9,
    #[doc = "Enable em4 wakeup on pin F1"]
    F1,
    #[doc = "Enable em4 wakeup on pin F2"]
    F2,
    #[doc = "Enable em4 wakeup on pin E13"]
    E13,
    #[doc = "Enable em4 wakeup on pin C4"]
    C4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM4WUENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM4WUENR::A0 => 1,
            EM4WUENR::C9 => 4,
            EM4WUENR::F1 => 8,
            EM4WUENR::F2 => 16,
            EM4WUENR::E13 => 32,
            EM4WUENR::C4 => 64,
            EM4WUENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM4WUENR {
        match value {
            1 => EM4WUENR::A0,
            4 => EM4WUENR::C9,
            8 => EM4WUENR::F1,
            16 => EM4WUENR::F2,
            32 => EM4WUENR::E13,
            64 => EM4WUENR::C4,
            i => EM4WUENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUENR::A0
    }
    #[doc = "Checks if the value of the field is `C9`"]
    #[inline]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUENR::C9
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUENR::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUENR::F2
    }
    #[doc = "Checks if the value of the field is `E13`"]
    #[inline]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUENR::E13
    }
    #[doc = "Checks if the value of the field is `C4`"]
    #[inline]
    pub fn is_c4(&self) -> bool {
        *self == EM4WUENR::C4
    }
}
#[doc = "Values that can be written to the field `EM4WUEN`"]
pub enum EM4WUENW {
    #[doc = "Enable em4 wakeup on pin A0"]
    A0,
    #[doc = "Enable em4 wakeup on pin C9"]
    C9,
    #[doc = "Enable em4 wakeup on pin F1"]
    F1,
    #[doc = "Enable em4 wakeup on pin F2"]
    F2,
    #[doc = "Enable em4 wakeup on pin E13"]
    E13,
    #[doc = "Enable em4 wakeup on pin C4"]
    C4,
}
impl EM4WUENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EM4WUENW::A0 => 1,
            EM4WUENW::C9 => 4,
            EM4WUENW::F1 => 8,
            EM4WUENW::F2 => 16,
            EM4WUENW::E13 => 32,
            EM4WUENW::C4 => 64,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EM4WUENW<'a> {
    w: &'a mut W,
}
impl<'a> _EM4WUENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EM4WUENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Enable em4 wakeup on pin A0"]
    #[inline]
    pub fn a0(self) -> &'a mut W {
        self.variant(EM4WUENW::A0)
    }
    #[doc = "Enable em4 wakeup on pin C9"]
    #[inline]
    pub fn c9(self) -> &'a mut W {
        self.variant(EM4WUENW::C9)
    }
    #[doc = "Enable em4 wakeup on pin F1"]
    #[inline]
    pub fn f1(self) -> &'a mut W {
        self.variant(EM4WUENW::F1)
    }
    #[doc = "Enable em4 wakeup on pin F2"]
    #[inline]
    pub fn f2(self) -> &'a mut W {
        self.variant(EM4WUENW::F2)
    }
    #[doc = "Enable em4 wakeup on pin E13"]
    #[inline]
    pub fn e13(self) -> &'a mut W {
        self.variant(EM4WUENW::E13)
    }
    #[doc = "Enable em4 wakeup on pin C4"]
    #[inline]
    pub fn c4(self) -> &'a mut W {
        self.variant(EM4WUENW::C4)
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
    #[doc = "Bits 0:6 - EM4 Wake-up enable"]
    #[inline]
    pub fn em4wuen(&self) -> EM4WUENR {
        EM4WUENR::_from({
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
    #[doc = "Bits 0:6 - EM4 Wake-up enable"]
    #[inline]
    pub fn em4wuen(&mut self) -> _EM4WUENW {
        _EM4WUENW { w: self }
    }
}
