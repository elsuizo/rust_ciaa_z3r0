#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALCTRL {
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
#[doc = "Possible values of the field `UPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPSELR {
    #[doc = "Select HFXO as up-counter."]
    HFXO,
    #[doc = "Select LFXO as up-counter."]
    LFXO,
    #[doc = "Select HFRCO as up-counter."]
    HFRCO,
    #[doc = "Select LFRCO as up-counter."]
    LFRCO,
    #[doc = "Select AUXHFRCO as up-counter."]
    AUXHFRCO,
    #[doc = "Select USHFRCO as up-counter."]
    USHFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UPSELR::HFXO => 0,
            UPSELR::LFXO => 1,
            UPSELR::HFRCO => 2,
            UPSELR::LFRCO => 3,
            UPSELR::AUXHFRCO => 4,
            UPSELR::USHFRCO => 5,
            UPSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UPSELR {
        match value {
            0 => UPSELR::HFXO,
            1 => UPSELR::LFXO,
            2 => UPSELR::HFRCO,
            3 => UPSELR::LFRCO,
            4 => UPSELR::AUXHFRCO,
            5 => UPSELR::USHFRCO,
            i => UPSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == UPSELR::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == UPSELR::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline]
    pub fn is_hfrco(&self) -> bool {
        *self == UPSELR::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == UPSELR::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == UPSELR::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline]
    pub fn is_ushfrco(&self) -> bool {
        *self == UPSELR::USHFRCO
    }
}
#[doc = "Possible values of the field `DOWNSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWNSELR {
    #[doc = "Select HFCLK for down-counter."]
    HFCLK,
    #[doc = "Select HFXO for down-counter."]
    HFXO,
    #[doc = "Select LFXO for down-counter."]
    LFXO,
    #[doc = "Select HFRCO for down-counter."]
    HFRCO,
    #[doc = "Select LFRCO for down-counter."]
    LFRCO,
    #[doc = "Select AUXHFRCO for down-counter."]
    AUXHFRCO,
    #[doc = "Select USHFRCO for down-counter."]
    USHFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DOWNSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DOWNSELR::HFCLK => 0,
            DOWNSELR::HFXO => 1,
            DOWNSELR::LFXO => 2,
            DOWNSELR::HFRCO => 3,
            DOWNSELR::LFRCO => 4,
            DOWNSELR::AUXHFRCO => 5,
            DOWNSELR::USHFRCO => 6,
            DOWNSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DOWNSELR {
        match value {
            0 => DOWNSELR::HFCLK,
            1 => DOWNSELR::HFXO,
            2 => DOWNSELR::LFXO,
            3 => DOWNSELR::HFRCO,
            4 => DOWNSELR::LFRCO,
            5 => DOWNSELR::AUXHFRCO,
            6 => DOWNSELR::USHFRCO,
            i => DOWNSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline]
    pub fn is_hfclk(&self) -> bool {
        *self == DOWNSELR::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == DOWNSELR::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == DOWNSELR::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline]
    pub fn is_hfrco(&self) -> bool {
        *self == DOWNSELR::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == DOWNSELR::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DOWNSELR::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline]
    pub fn is_ushfrco(&self) -> bool {
        *self == DOWNSELR::USHFRCO
    }
}
#[doc = r" Value of the field"]
pub struct CONTR {
    bits: bool,
}
impl CONTR {
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
#[doc = "Values that can be written to the field `UPSEL`"]
pub enum UPSELW {
    #[doc = "Select HFXO as up-counter."]
    HFXO,
    #[doc = "Select LFXO as up-counter."]
    LFXO,
    #[doc = "Select HFRCO as up-counter."]
    HFRCO,
    #[doc = "Select LFRCO as up-counter."]
    LFRCO,
    #[doc = "Select AUXHFRCO as up-counter."]
    AUXHFRCO,
    #[doc = "Select USHFRCO as up-counter."]
    USHFRCO,
}
impl UPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UPSELW::HFXO => 0,
            UPSELW::LFXO => 1,
            UPSELW::HFRCO => 2,
            UPSELW::LFRCO => 3,
            UPSELW::AUXHFRCO => 4,
            UPSELW::USHFRCO => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _UPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select HFXO as up-counter."]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(UPSELW::HFXO)
    }
    #[doc = "Select LFXO as up-counter."]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(UPSELW::LFXO)
    }
    #[doc = "Select HFRCO as up-counter."]
    #[inline]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(UPSELW::HFRCO)
    }
    #[doc = "Select LFRCO as up-counter."]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(UPSELW::LFRCO)
    }
    #[doc = "Select AUXHFRCO as up-counter."]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(UPSELW::AUXHFRCO)
    }
    #[doc = "Select USHFRCO as up-counter."]
    #[inline]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(UPSELW::USHFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOWNSEL`"]
pub enum DOWNSELW {
    #[doc = "Select HFCLK for down-counter."]
    HFCLK,
    #[doc = "Select HFXO for down-counter."]
    HFXO,
    #[doc = "Select LFXO for down-counter."]
    LFXO,
    #[doc = "Select HFRCO for down-counter."]
    HFRCO,
    #[doc = "Select LFRCO for down-counter."]
    LFRCO,
    #[doc = "Select AUXHFRCO for down-counter."]
    AUXHFRCO,
    #[doc = "Select USHFRCO for down-counter."]
    USHFRCO,
}
impl DOWNSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DOWNSELW::HFCLK => 0,
            DOWNSELW::HFXO => 1,
            DOWNSELW::LFXO => 2,
            DOWNSELW::HFRCO => 3,
            DOWNSELW::LFRCO => 4,
            DOWNSELW::AUXHFRCO => 5,
            DOWNSELW::USHFRCO => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOWNSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DOWNSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOWNSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select HFCLK for down-counter."]
    #[inline]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DOWNSELW::HFCLK)
    }
    #[doc = "Select HFXO for down-counter."]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(DOWNSELW::HFXO)
    }
    #[doc = "Select LFXO for down-counter."]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(DOWNSELW::LFXO)
    }
    #[doc = "Select HFRCO for down-counter."]
    #[inline]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(DOWNSELW::HFRCO)
    }
    #[doc = "Select LFRCO for down-counter."]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(DOWNSELW::LFRCO)
    }
    #[doc = "Select AUXHFRCO for down-counter."]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DOWNSELW::AUXHFRCO)
    }
    #[doc = "Select USHFRCO for down-counter."]
    #[inline]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(DOWNSELW::USHFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline]
    pub fn upsel(&self) -> UPSELR {
        UPSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline]
    pub fn downsel(&self) -> DOWNSELR {
        DOWNSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONTR { bits }
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
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline]
    pub fn upsel(&mut self) -> _UPSELW {
        _UPSELW { w: self }
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline]
    pub fn downsel(&mut self) -> _DOWNSELW {
        _DOWNSELW { w: self }
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
}
