#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EM4WUCAUSE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `EM4WUCAUSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM4WUCAUSER {
    #[doc = "This bit indicates an em4 wake-up request occurred on pin A0"]
    A0,
    #[doc = "This bit indicates an em4 wake-up request occurred on pin C9"]
    C9,
    #[doc = "This bit indicates an em4 wake-up request occurred on pin F1"]
    F1,
    #[doc = "This bit indicates an em4 wake-up request occurred on pin F2"]
    F2,
    #[doc = "This bit indicates an em4 wake-up request occurred on pin E13"]
    E13,
    #[doc = "This bit indicates an em4 wake-up request occurred on pin C4"]
    C4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM4WUCAUSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM4WUCAUSER::A0 => 1,
            EM4WUCAUSER::C9 => 4,
            EM4WUCAUSER::F1 => 8,
            EM4WUCAUSER::F2 => 16,
            EM4WUCAUSER::E13 => 32,
            EM4WUCAUSER::C4 => 64,
            EM4WUCAUSER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM4WUCAUSER {
        match value {
            1 => EM4WUCAUSER::A0,
            4 => EM4WUCAUSER::C9,
            8 => EM4WUCAUSER::F1,
            16 => EM4WUCAUSER::F2,
            32 => EM4WUCAUSER::E13,
            64 => EM4WUCAUSER::C4,
            i => EM4WUCAUSER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `A0`"]
    #[inline]
    pub fn is_a0(&self) -> bool {
        *self == EM4WUCAUSER::A0
    }
    #[doc = "Checks if the value of the field is `C9`"]
    #[inline]
    pub fn is_c9(&self) -> bool {
        *self == EM4WUCAUSER::C9
    }
    #[doc = "Checks if the value of the field is `F1`"]
    #[inline]
    pub fn is_f1(&self) -> bool {
        *self == EM4WUCAUSER::F1
    }
    #[doc = "Checks if the value of the field is `F2`"]
    #[inline]
    pub fn is_f2(&self) -> bool {
        *self == EM4WUCAUSER::F2
    }
    #[doc = "Checks if the value of the field is `E13`"]
    #[inline]
    pub fn is_e13(&self) -> bool {
        *self == EM4WUCAUSER::E13
    }
    #[doc = "Checks if the value of the field is `C4`"]
    #[inline]
    pub fn is_c4(&self) -> bool {
        *self == EM4WUCAUSER::C4
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - EM4 wake-up cause"]
    #[inline]
    pub fn em4wucause(&self) -> EM4WUCAUSER {
        EM4WUCAUSER::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
