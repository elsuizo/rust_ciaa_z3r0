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
#[doc = "Possible values of the field `HFXOMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOMODER {
    #[doc = "4-25 MHz crystal oscillator."]
    XTAL,
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-25 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    BUFEXTCLK,
    #[doc = "Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HFXOMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFXOMODER::XTAL => 0,
            HFXOMODER::BUFEXTCLK => 1,
            HFXOMODER::DIGEXTCLK => 2,
            HFXOMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFXOMODER {
        match value {
            0 => HFXOMODER::XTAL,
            1 => HFXOMODER::BUFEXTCLK,
            2 => HFXOMODER::DIGEXTCLK,
            i => HFXOMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline]
    pub fn is_xtal(&self) -> bool {
        *self == HFXOMODER::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline]
    pub fn is_bufextclk(&self) -> bool {
        *self == HFXOMODER::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline]
    pub fn is_digextclk(&self) -> bool {
        *self == HFXOMODER::DIGEXTCLK
    }
}
#[doc = "Possible values of the field `HFXOBOOST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOBOOSTR {
    #[doc = "50 %."]
    _50PCENT,
    #[doc = "70 %."]
    _70PCENT,
    #[doc = "80 %."]
    _80PCENT,
    #[doc = "100 % (default)."]
    _100PCENT,
}
impl HFXOBOOSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFXOBOOSTR::_50PCENT => 0,
            HFXOBOOSTR::_70PCENT => 1,
            HFXOBOOSTR::_80PCENT => 2,
            HFXOBOOSTR::_100PCENT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFXOBOOSTR {
        match value {
            0 => HFXOBOOSTR::_50PCENT,
            1 => HFXOBOOSTR::_70PCENT,
            2 => HFXOBOOSTR::_80PCENT,
            3 => HFXOBOOSTR::_100PCENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_50PCENT`"]
    #[inline]
    pub fn is_50pcent(&self) -> bool {
        *self == HFXOBOOSTR::_50PCENT
    }
    #[doc = "Checks if the value of the field is `_70PCENT`"]
    #[inline]
    pub fn is_70pcent(&self) -> bool {
        *self == HFXOBOOSTR::_70PCENT
    }
    #[doc = "Checks if the value of the field is `_80PCENT`"]
    #[inline]
    pub fn is_80pcent(&self) -> bool {
        *self == HFXOBOOSTR::_80PCENT
    }
    #[doc = "Checks if the value of the field is `_100PCENT`"]
    #[inline]
    pub fn is_100pcent(&self) -> bool {
        *self == HFXOBOOSTR::_100PCENT
    }
}
#[doc = r" Value of the field"]
pub struct HFXOBUFCURR {
    bits: u8,
}
impl HFXOBUFCURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HFXOGLITCHDETENR {
    bits: bool,
}
impl HFXOGLITCHDETENR {
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
#[doc = "Possible values of the field `HFXOTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOTIMEOUTR {
    #[doc = "Timeout period of 8 cycles."]
    _8CYCLES,
    #[doc = "Timeout period of 256 cycles."]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles."]
    _1KCYCLES,
    #[doc = "Timeout period of 16384 cycles."]
    _16KCYCLES,
}
impl HFXOTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFXOTIMEOUTR::_8CYCLES => 0,
            HFXOTIMEOUTR::_256CYCLES => 1,
            HFXOTIMEOUTR::_1KCYCLES => 2,
            HFXOTIMEOUTR::_16KCYCLES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFXOTIMEOUTR {
        match value {
            0 => HFXOTIMEOUTR::_8CYCLES,
            1 => HFXOTIMEOUTR::_256CYCLES,
            2 => HFXOTIMEOUTR::_1KCYCLES,
            3 => HFXOTIMEOUTR::_16KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline]
    pub fn is_8cycles(&self) -> bool {
        *self == HFXOTIMEOUTR::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline]
    pub fn is_256cycles(&self) -> bool {
        *self == HFXOTIMEOUTR::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == HFXOTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == HFXOTIMEOUTR::_16KCYCLES
    }
}
#[doc = "Possible values of the field `LFXOMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXOMODER {
    #[doc = "32.768 kHz crystal oscillator."]
    XTAL,
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    BUFEXTCLK,
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LFXOMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFXOMODER::XTAL => 0,
            LFXOMODER::BUFEXTCLK => 1,
            LFXOMODER::DIGEXTCLK => 2,
            LFXOMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFXOMODER {
        match value {
            0 => LFXOMODER::XTAL,
            1 => LFXOMODER::BUFEXTCLK,
            2 => LFXOMODER::DIGEXTCLK,
            i => LFXOMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline]
    pub fn is_xtal(&self) -> bool {
        *self == LFXOMODER::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline]
    pub fn is_bufextclk(&self) -> bool {
        *self == LFXOMODER::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline]
    pub fn is_digextclk(&self) -> bool {
        *self == LFXOMODER::DIGEXTCLK
    }
}
#[doc = r" Value of the field"]
pub struct LFXOBOOSTR {
    bits: bool,
}
impl LFXOBOOSTR {
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
pub struct HFCLKDIVR {
    bits: u8,
}
impl HFCLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LFXOBUFCURR {
    bits: bool,
}
impl LFXOBUFCURR {
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
#[doc = "Possible values of the field `LFXOTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXOTIMEOUTR {
    #[doc = "Timeout period of 8 cycles."]
    _8CYCLES,
    #[doc = "Timeout period of 1024 cycles."]
    _1KCYCLES,
    #[doc = "Timeout period of 16384 cycles."]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles."]
    _32KCYCLES,
}
impl LFXOTIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFXOTIMEOUTR::_8CYCLES => 0,
            LFXOTIMEOUTR::_1KCYCLES => 1,
            LFXOTIMEOUTR::_16KCYCLES => 2,
            LFXOTIMEOUTR::_32KCYCLES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFXOTIMEOUTR {
        match value {
            0 => LFXOTIMEOUTR::_8CYCLES,
            1 => LFXOTIMEOUTR::_1KCYCLES,
            2 => LFXOTIMEOUTR::_16KCYCLES,
            3 => LFXOTIMEOUTR::_32KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline]
    pub fn is_8cycles(&self) -> bool {
        *self == LFXOTIMEOUTR::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFXOTIMEOUTR::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline]
    pub fn is_16kcycles(&self) -> bool {
        *self == LFXOTIMEOUTR::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline]
    pub fn is_32kcycles(&self) -> bool {
        *self == LFXOTIMEOUTR::_32KCYCLES
    }
}
#[doc = "Possible values of the field `CLKOUTSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL0R {
    #[doc = "HFRCO (directly from oscillator)."]
    HFRCO,
    #[doc = "HFXO (directly from oscillator)."]
    HFXO,
    #[doc = "HFCLK/2."]
    HFCLK2,
    #[doc = "HFCLK/4."]
    HFCLK4,
    #[doc = "HFCLK/8."]
    HFCLK8,
    #[doc = "HFCLK/16."]
    HFCLK16,
    #[doc = "ULFRCO (directly from oscillator)."]
    ULFRCO,
    #[doc = "AUXHFRCO (directly from oscillator)."]
    AUXHFRCO,
}
impl CLKOUTSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSEL0R::HFRCO => 0,
            CLKOUTSEL0R::HFXO => 1,
            CLKOUTSEL0R::HFCLK2 => 2,
            CLKOUTSEL0R::HFCLK4 => 3,
            CLKOUTSEL0R::HFCLK8 => 4,
            CLKOUTSEL0R::HFCLK16 => 5,
            CLKOUTSEL0R::ULFRCO => 6,
            CLKOUTSEL0R::AUXHFRCO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSEL0R {
        match value {
            0 => CLKOUTSEL0R::HFRCO,
            1 => CLKOUTSEL0R::HFXO,
            2 => CLKOUTSEL0R::HFCLK2,
            3 => CLKOUTSEL0R::HFCLK4,
            4 => CLKOUTSEL0R::HFCLK8,
            5 => CLKOUTSEL0R::HFCLK16,
            6 => CLKOUTSEL0R::ULFRCO,
            7 => CLKOUTSEL0R::AUXHFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline]
    pub fn is_hfrco(&self) -> bool {
        *self == CLKOUTSEL0R::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL0R::HFXO
    }
    #[doc = "Checks if the value of the field is `HFCLK2`"]
    #[inline]
    pub fn is_hfclk2(&self) -> bool {
        *self == CLKOUTSEL0R::HFCLK2
    }
    #[doc = "Checks if the value of the field is `HFCLK4`"]
    #[inline]
    pub fn is_hfclk4(&self) -> bool {
        *self == CLKOUTSEL0R::HFCLK4
    }
    #[doc = "Checks if the value of the field is `HFCLK8`"]
    #[inline]
    pub fn is_hfclk8(&self) -> bool {
        *self == CLKOUTSEL0R::HFCLK8
    }
    #[doc = "Checks if the value of the field is `HFCLK16`"]
    #[inline]
    pub fn is_hfclk16(&self) -> bool {
        *self == CLKOUTSEL0R::HFCLK16
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL0R::ULFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == CLKOUTSEL0R::AUXHFRCO
    }
}
#[doc = "Possible values of the field `CLKOUTSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL1R {
    #[doc = "LFRCO (directly from oscillator)."]
    LFRCO,
    #[doc = "LFXO (directly from oscillator)."]
    LFXO,
    #[doc = "HFCLK (undivided)."]
    HFCLK,
    #[doc = "LFXO (qualified)."]
    LFXOQ,
    #[doc = "HFXO (qualified)."]
    HFXOQ,
    #[doc = "LFRCO (qualified)."]
    LFRCOQ,
    #[doc = "HFRCO (qualified)."]
    HFRCOQ,
    #[doc = "AUXHFRCO (qualified)."]
    AUXHFRCOQ,
    #[doc = "USHFRCO"]
    USHFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKOUTSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSEL1R::LFRCO => 0,
            CLKOUTSEL1R::LFXO => 1,
            CLKOUTSEL1R::HFCLK => 2,
            CLKOUTSEL1R::LFXOQ => 3,
            CLKOUTSEL1R::HFXOQ => 4,
            CLKOUTSEL1R::LFRCOQ => 5,
            CLKOUTSEL1R::HFRCOQ => 6,
            CLKOUTSEL1R::AUXHFRCOQ => 7,
            CLKOUTSEL1R::USHFRCO => 8,
            CLKOUTSEL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSEL1R {
        match value {
            0 => CLKOUTSEL1R::LFRCO,
            1 => CLKOUTSEL1R::LFXO,
            2 => CLKOUTSEL1R::HFCLK,
            3 => CLKOUTSEL1R::LFXOQ,
            4 => CLKOUTSEL1R::HFXOQ,
            5 => CLKOUTSEL1R::LFRCOQ,
            6 => CLKOUTSEL1R::HFRCOQ,
            7 => CLKOUTSEL1R::AUXHFRCOQ,
            8 => CLKOUTSEL1R::USHFRCO,
            i => CLKOUTSEL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL1R::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL1R::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline]
    pub fn is_hfclk(&self) -> bool {
        *self == CLKOUTSEL1R::HFCLK
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL1R::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL1R::HFXOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL1R::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL1R::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL1R::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline]
    pub fn is_ushfrco(&self) -> bool {
        *self == CLKOUTSEL1R::USHFRCO
    }
}
#[doc = "Values that can be written to the field `HFXOMODE`"]
pub enum HFXOMODEW {
    #[doc = "4-25 MHz crystal oscillator."]
    XTAL,
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-25 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    BUFEXTCLK,
    #[doc = "Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK,
}
impl HFXOMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFXOMODEW::XTAL => 0,
            HFXOMODEW::BUFEXTCLK => 1,
            HFXOMODEW::DIGEXTCLK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFXOMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXOMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFXOMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4-25 MHz crystal oscillator."]
    #[inline]
    pub fn xtal(self) -> &'a mut W {
        self.variant(HFXOMODEW::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-25 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    #[inline]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(HFXOMODEW::BUFEXTCLK)
    }
    #[doc = "Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(HFXOMODEW::DIGEXTCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HFXOBOOST`"]
pub enum HFXOBOOSTW {
    #[doc = "50 %."]
    _50PCENT,
    #[doc = "70 %."]
    _70PCENT,
    #[doc = "80 %."]
    _80PCENT,
    #[doc = "100 % (default)."]
    _100PCENT,
}
impl HFXOBOOSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFXOBOOSTW::_50PCENT => 0,
            HFXOBOOSTW::_70PCENT => 1,
            HFXOBOOSTW::_80PCENT => 2,
            HFXOBOOSTW::_100PCENT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFXOBOOSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXOBOOSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFXOBOOSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "50 %."]
    #[inline]
    pub fn _50pcent(self) -> &'a mut W {
        self.variant(HFXOBOOSTW::_50PCENT)
    }
    #[doc = "70 %."]
    #[inline]
    pub fn _70pcent(self) -> &'a mut W {
        self.variant(HFXOBOOSTW::_70PCENT)
    }
    #[doc = "80 %."]
    #[inline]
    pub fn _80pcent(self) -> &'a mut W {
        self.variant(HFXOBOOSTW::_80PCENT)
    }
    #[doc = "100 % (default)."]
    #[inline]
    pub fn _100pcent(self) -> &'a mut W {
        self.variant(HFXOBOOSTW::_100PCENT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HFXOBUFCURW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXOBUFCURW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HFXOGLITCHDETENW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXOGLITCHDETENW<'a> {
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
#[doc = "Values that can be written to the field `HFXOTIMEOUT`"]
pub enum HFXOTIMEOUTW {
    #[doc = "Timeout period of 8 cycles."]
    _8CYCLES,
    #[doc = "Timeout period of 256 cycles."]
    _256CYCLES,
    #[doc = "Timeout period of 1024 cycles."]
    _1KCYCLES,
    #[doc = "Timeout period of 16384 cycles."]
    _16KCYCLES,
}
impl HFXOTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFXOTIMEOUTW::_8CYCLES => 0,
            HFXOTIMEOUTW::_256CYCLES => 1,
            HFXOTIMEOUTW::_1KCYCLES => 2,
            HFXOTIMEOUTW::_16KCYCLES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFXOTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXOTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFXOTIMEOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timeout period of 8 cycles."]
    #[inline]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUTW::_8CYCLES)
    }
    #[doc = "Timeout period of 256 cycles."]
    #[inline]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUTW::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUTW::_16KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LFXOMODE`"]
pub enum LFXOMODEW {
    #[doc = "32.768 kHz crystal oscillator."]
    XTAL,
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    BUFEXTCLK,
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK,
}
impl LFXOMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFXOMODEW::XTAL => 0,
            LFXOMODEW::BUFEXTCLK => 1,
            LFXOMODEW::DIGEXTCLK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFXOMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LFXOMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFXOMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32.768 kHz crystal oscillator."]
    #[inline]
    pub fn xtal(self) -> &'a mut W {
        self.variant(LFXOMODEW::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    #[inline]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(LFXOMODEW::BUFEXTCLK)
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(LFXOMODEW::DIGEXTCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LFXOBOOSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LFXOBOOSTW<'a> {
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
pub struct _HFCLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _HFCLKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LFXOBUFCURW<'a> {
    w: &'a mut W,
}
impl<'a> _LFXOBUFCURW<'a> {
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
#[doc = "Values that can be written to the field `LFXOTIMEOUT`"]
pub enum LFXOTIMEOUTW {
    #[doc = "Timeout period of 8 cycles."]
    _8CYCLES,
    #[doc = "Timeout period of 1024 cycles."]
    _1KCYCLES,
    #[doc = "Timeout period of 16384 cycles."]
    _16KCYCLES,
    #[doc = "Timeout period of 32768 cycles."]
    _32KCYCLES,
}
impl LFXOTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFXOTIMEOUTW::_8CYCLES => 0,
            LFXOTIMEOUTW::_1KCYCLES => 1,
            LFXOTIMEOUTW::_16KCYCLES => 2,
            LFXOTIMEOUTW::_32KCYCLES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFXOTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _LFXOTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFXOTIMEOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timeout period of 8 cycles."]
    #[inline]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUTW::_8CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUTW::_1KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUTW::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles."]
    #[inline]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUTW::_32KCYCLES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKOUTSEL0`"]
pub enum CLKOUTSEL0W {
    #[doc = "HFRCO (directly from oscillator)."]
    HFRCO,
    #[doc = "HFXO (directly from oscillator)."]
    HFXO,
    #[doc = "HFCLK/2."]
    HFCLK2,
    #[doc = "HFCLK/4."]
    HFCLK4,
    #[doc = "HFCLK/8."]
    HFCLK8,
    #[doc = "HFCLK/16."]
    HFCLK16,
    #[doc = "ULFRCO (directly from oscillator)."]
    ULFRCO,
    #[doc = "AUXHFRCO (directly from oscillator)."]
    AUXHFRCO,
}
impl CLKOUTSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSEL0W::HFRCO => 0,
            CLKOUTSEL0W::HFXO => 1,
            CLKOUTSEL0W::HFCLK2 => 2,
            CLKOUTSEL0W::HFCLK4 => 3,
            CLKOUTSEL0W::HFCLK8 => 4,
            CLKOUTSEL0W::HFCLK16 => 5,
            CLKOUTSEL0W::ULFRCO => 6,
            CLKOUTSEL0W::AUXHFRCO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSEL0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "HFRCO (directly from oscillator)."]
    #[inline]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFRCO)
    }
    #[doc = "HFXO (directly from oscillator)."]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFXO)
    }
    #[doc = "HFCLK/2."]
    #[inline]
    pub fn hfclk2(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFCLK2)
    }
    #[doc = "HFCLK/4."]
    #[inline]
    pub fn hfclk4(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFCLK4)
    }
    #[doc = "HFCLK/8."]
    #[inline]
    pub fn hfclk8(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFCLK8)
    }
    #[doc = "HFCLK/16."]
    #[inline]
    pub fn hfclk16(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::HFCLK16)
    }
    #[doc = "ULFRCO (directly from oscillator)."]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::ULFRCO)
    }
    #[doc = "AUXHFRCO (directly from oscillator)."]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0W::AUXHFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKOUTSEL1`"]
pub enum CLKOUTSEL1W {
    #[doc = "LFRCO (directly from oscillator)."]
    LFRCO,
    #[doc = "LFXO (directly from oscillator)."]
    LFXO,
    #[doc = "HFCLK (undivided)."]
    HFCLK,
    #[doc = "LFXO (qualified)."]
    LFXOQ,
    #[doc = "HFXO (qualified)."]
    HFXOQ,
    #[doc = "LFRCO (qualified)."]
    LFRCOQ,
    #[doc = "HFRCO (qualified)."]
    HFRCOQ,
    #[doc = "AUXHFRCO (qualified)."]
    AUXHFRCOQ,
    #[doc = "USHFRCO"]
    USHFRCO,
}
impl CLKOUTSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSEL1W::LFRCO => 0,
            CLKOUTSEL1W::LFXO => 1,
            CLKOUTSEL1W::HFCLK => 2,
            CLKOUTSEL1W::LFXOQ => 3,
            CLKOUTSEL1W::HFXOQ => 4,
            CLKOUTSEL1W::LFRCOQ => 5,
            CLKOUTSEL1W::HFRCOQ => 6,
            CLKOUTSEL1W::AUXHFRCOQ => 7,
            CLKOUTSEL1W::USHFRCO => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSEL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFRCO (directly from oscillator)."]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)."]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::LFXO)
    }
    #[doc = "HFCLK (undivided)."]
    #[inline]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::HFCLK)
    }
    #[doc = "LFXO (qualified)."]
    #[inline]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::LFXOQ)
    }
    #[doc = "HFXO (qualified)."]
    #[inline]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::HFXOQ)
    }
    #[doc = "LFRCO (qualified)."]
    #[inline]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::LFRCOQ)
    }
    #[doc = "HFRCO (qualified)."]
    #[inline]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)."]
    #[inline]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::AUXHFRCOQ)
    }
    #[doc = "USHFRCO"]
    #[inline]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1W::USHFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 23;
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
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline]
    pub fn hfxomode(&self) -> HFXOMODER {
        HFXOMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - HFXO Start-up Boost Current"]
    #[inline]
    pub fn hfxoboost(&self) -> HFXOBOOSTR {
        HFXOBOOSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - HFXO Boost Buffer Current"]
    #[inline]
    pub fn hfxobufcur(&self) -> HFXOBUFCURR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HFXOBUFCURR { bits }
    }
    #[doc = "Bit 7 - HFXO Glitch Detector Enable"]
    #[inline]
    pub fn hfxoglitchdeten(&self) -> HFXOGLITCHDETENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HFXOGLITCHDETENR { bits }
    }
    #[doc = "Bits 9:10 - HFXO Timeout"]
    #[inline]
    pub fn hfxotimeout(&self) -> HFXOTIMEOUTR {
        HFXOTIMEOUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:12 - LFXO Mode"]
    #[inline]
    pub fn lfxomode(&self) -> LFXOMODER {
        LFXOMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - LFXO Start-up Boost Current"]
    #[inline]
    pub fn lfxoboost(&self) -> LFXOBOOSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LFXOBOOSTR { bits }
    }
    #[doc = "Bits 14:16 - HFCLK Division"]
    #[inline]
    pub fn hfclkdiv(&self) -> HFCLKDIVR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HFCLKDIVR { bits }
    }
    #[doc = "Bit 17 - LFXO Boost Buffer Current"]
    #[inline]
    pub fn lfxobufcur(&self) -> LFXOBUFCURR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LFXOBUFCURR { bits }
    }
    #[doc = "Bits 18:19 - LFXO Timeout"]
    #[inline]
    pub fn lfxotimeout(&self) -> LFXOTIMEOUTR {
        LFXOTIMEOUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Clock Output Select 0"]
    #[inline]
    pub fn clkoutsel0(&self) -> CLKOUTSEL0R {
        CLKOUTSEL0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 23:26 - Clock Output Select 1"]
    #[inline]
    pub fn clkoutsel1(&self) -> CLKOUTSEL1R {
        CLKOUTSEL1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 796204 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline]
    pub fn hfxomode(&mut self) -> _HFXOMODEW {
        _HFXOMODEW { w: self }
    }
    #[doc = "Bits 2:3 - HFXO Start-up Boost Current"]
    #[inline]
    pub fn hfxoboost(&mut self) -> _HFXOBOOSTW {
        _HFXOBOOSTW { w: self }
    }
    #[doc = "Bits 5:6 - HFXO Boost Buffer Current"]
    #[inline]
    pub fn hfxobufcur(&mut self) -> _HFXOBUFCURW {
        _HFXOBUFCURW { w: self }
    }
    #[doc = "Bit 7 - HFXO Glitch Detector Enable"]
    #[inline]
    pub fn hfxoglitchdeten(&mut self) -> _HFXOGLITCHDETENW {
        _HFXOGLITCHDETENW { w: self }
    }
    #[doc = "Bits 9:10 - HFXO Timeout"]
    #[inline]
    pub fn hfxotimeout(&mut self) -> _HFXOTIMEOUTW {
        _HFXOTIMEOUTW { w: self }
    }
    #[doc = "Bits 11:12 - LFXO Mode"]
    #[inline]
    pub fn lfxomode(&mut self) -> _LFXOMODEW {
        _LFXOMODEW { w: self }
    }
    #[doc = "Bit 13 - LFXO Start-up Boost Current"]
    #[inline]
    pub fn lfxoboost(&mut self) -> _LFXOBOOSTW {
        _LFXOBOOSTW { w: self }
    }
    #[doc = "Bits 14:16 - HFCLK Division"]
    #[inline]
    pub fn hfclkdiv(&mut self) -> _HFCLKDIVW {
        _HFCLKDIVW { w: self }
    }
    #[doc = "Bit 17 - LFXO Boost Buffer Current"]
    #[inline]
    pub fn lfxobufcur(&mut self) -> _LFXOBUFCURW {
        _LFXOBUFCURW { w: self }
    }
    #[doc = "Bits 18:19 - LFXO Timeout"]
    #[inline]
    pub fn lfxotimeout(&mut self) -> _LFXOTIMEOUTW {
        _LFXOTIMEOUTW { w: self }
    }
    #[doc = "Bits 20:22 - Clock Output Select 0"]
    #[inline]
    pub fn clkoutsel0(&mut self) -> _CLKOUTSEL0W {
        _CLKOUTSEL0W { w: self }
    }
    #[doc = "Bits 23:26 - Clock Output Select 1"]
    #[inline]
    pub fn clkoutsel1(&mut self) -> _CLKOUTSEL1W {
        _CLKOUTSEL1W { w: self }
    }
}
