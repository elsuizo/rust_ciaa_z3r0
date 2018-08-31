#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CC2_CTRL {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Compare/Capture channel turned off"]
    OFF,
    #[doc = "Input capture"]
    INPUTCAPTURE,
    #[doc = "Output compare"]
    OUTPUTCOMPARE,
    #[doc = "Pulse-Width Modulation"]
    PWM,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::OFF => 0,
            MODER::INPUTCAPTURE => 1,
            MODER::OUTPUTCOMPARE => 2,
            MODER::PWM => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::OFF,
            1 => MODER::INPUTCAPTURE,
            2 => MODER::OUTPUTCOMPARE,
            3 => MODER::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == MODER::OFF
    }
    #[doc = "Checks if the value of the field is `INPUTCAPTURE`"]
    #[inline]
    pub fn is_inputcapture(&self) -> bool {
        *self == MODER::INPUTCAPTURE
    }
    #[doc = "Checks if the value of the field is `OUTPUTCOMPARE`"]
    #[inline]
    pub fn is_outputcompare(&self) -> bool {
        *self == MODER::OUTPUTCOMPARE
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == MODER::PWM
    }
}
#[doc = r" Value of the field"]
pub struct OUTINVR {
    bits: bool,
}
impl OUTINVR {
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
pub struct COISTR {
    bits: bool,
}
impl COISTR {
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
#[doc = "Possible values of the field `CMOA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMOAR {
    #[doc = "No action on compare match"]
    NONE,
    #[doc = "Toggle output on compare match"]
    TOGGLE,
    #[doc = "Clear output on compare match"]
    CLEAR,
    #[doc = "Set output on compare match"]
    SET,
}
impl CMOAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMOAR::NONE => 0,
            CMOAR::TOGGLE => 1,
            CMOAR::CLEAR => 2,
            CMOAR::SET => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMOAR {
        match value {
            0 => CMOAR::NONE,
            1 => CMOAR::TOGGLE,
            2 => CMOAR::CLEAR,
            3 => CMOAR::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == CMOAR::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == CMOAR::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CMOAR::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == CMOAR::SET
    }
}
#[doc = "Possible values of the field `COFOA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COFOAR {
    #[doc = "No action on counter overflow"]
    NONE,
    #[doc = "Toggle output on counter overflow"]
    TOGGLE,
    #[doc = "Clear output on counter overflow"]
    CLEAR,
    #[doc = "Set output on counter overflow"]
    SET,
}
impl COFOAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COFOAR::NONE => 0,
            COFOAR::TOGGLE => 1,
            COFOAR::CLEAR => 2,
            COFOAR::SET => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COFOAR {
        match value {
            0 => COFOAR::NONE,
            1 => COFOAR::TOGGLE,
            2 => COFOAR::CLEAR,
            3 => COFOAR::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == COFOAR::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == COFOAR::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == COFOAR::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == COFOAR::SET
    }
}
#[doc = "Possible values of the field `CUFOA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CUFOAR {
    #[doc = "No action on counter underflow"]
    NONE,
    #[doc = "Toggle output on counter underflow"]
    TOGGLE,
    #[doc = "Clear output on counter underflow"]
    CLEAR,
    #[doc = "Set output on counter underflow"]
    SET,
}
impl CUFOAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CUFOAR::NONE => 0,
            CUFOAR::TOGGLE => 1,
            CUFOAR::CLEAR => 2,
            CUFOAR::SET => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CUFOAR {
        match value {
            0 => CUFOAR::NONE,
            1 => CUFOAR::TOGGLE,
            2 => CUFOAR::CLEAR,
            3 => CUFOAR::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == CUFOAR::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == CUFOAR::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CUFOAR::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == CUFOAR::SET
    }
}
#[doc = "Possible values of the field `PRSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRSSELR {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRSSELR::PRSCH0 => 0,
            PRSSELR::PRSCH1 => 1,
            PRSSELR::PRSCH2 => 2,
            PRSSELR::PRSCH3 => 3,
            PRSSELR::PRSCH4 => 4,
            PRSSELR::PRSCH5 => 5,
            PRSSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRSSELR {
        match value {
            0 => PRSSELR::PRSCH0,
            1 => PRSSELR::PRSCH1,
            2 => PRSSELR::PRSCH2,
            3 => PRSSELR::PRSCH3,
            4 => PRSSELR::PRSCH4,
            5 => PRSSELR::PRSCH5,
            i => PRSSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSELR::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSELR::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSELR::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSELR::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSELR::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSELR::PRSCH5
    }
}
#[doc = r" Value of the field"]
pub struct INSELR {
    bits: bool,
}
impl INSELR {
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
pub struct FILTR {
    bits: bool,
}
impl FILTR {
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
#[doc = "Possible values of the field `ICEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEDGER {
    #[doc = "Rising edges detected"]
    RISING,
    #[doc = "Falling edges detected"]
    FALLING,
    #[doc = "Both edges detected"]
    BOTH,
    #[doc = "No edge detection, signal is left as it is"]
    NONE,
}
impl ICEDGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICEDGER::RISING => 0,
            ICEDGER::FALLING => 1,
            ICEDGER::BOTH => 2,
            ICEDGER::NONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICEDGER {
        match value {
            0 => ICEDGER::RISING,
            1 => ICEDGER::FALLING,
            2 => ICEDGER::BOTH,
            3 => ICEDGER::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == ICEDGER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == ICEDGER::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == ICEDGER::BOTH
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ICEDGER::NONE
    }
}
#[doc = "Possible values of the field `ICEVCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEVCTRLR {
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every capture"]
    EVERYEDGE,
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every second capture"]
    EVERYSECONDEDGE,
    #[doc = "PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    RISING,
    #[doc = "PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    FALLING,
}
impl ICEVCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICEVCTRLR::EVERYEDGE => 0,
            ICEVCTRLR::EVERYSECONDEDGE => 1,
            ICEVCTRLR::RISING => 2,
            ICEVCTRLR::FALLING => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICEVCTRLR {
        match value {
            0 => ICEVCTRLR::EVERYEDGE,
            1 => ICEVCTRLR::EVERYSECONDEDGE,
            2 => ICEVCTRLR::RISING,
            3 => ICEVCTRLR::FALLING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EVERYEDGE`"]
    #[inline]
    pub fn is_everyedge(&self) -> bool {
        *self == ICEVCTRLR::EVERYEDGE
    }
    #[doc = "Checks if the value of the field is `EVERYSECONDEDGE`"]
    #[inline]
    pub fn is_everysecondedge(&self) -> bool {
        *self == ICEVCTRLR::EVERYSECONDEDGE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == ICEVCTRLR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == ICEVCTRLR::FALLING
    }
}
#[doc = r" Value of the field"]
pub struct PRSCONFR {
    bits: bool,
}
impl PRSCONFR {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Compare/Capture channel turned off"]
    OFF,
    #[doc = "Input capture"]
    INPUTCAPTURE,
    #[doc = "Output compare"]
    OUTPUTCOMPARE,
    #[doc = "Pulse-Width Modulation"]
    PWM,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::OFF => 0,
            MODEW::INPUTCAPTURE => 1,
            MODEW::OUTPUTCOMPARE => 2,
            MODEW::PWM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Compare/Capture channel turned off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(MODEW::OFF)
    }
    #[doc = "Input capture"]
    #[inline]
    pub fn inputcapture(self) -> &'a mut W {
        self.variant(MODEW::INPUTCAPTURE)
    }
    #[doc = "Output compare"]
    #[inline]
    pub fn outputcompare(self) -> &'a mut W {
        self.variant(MODEW::OUTPUTCOMPARE)
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODEW::PWM)
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
pub struct _OUTINVW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTINVW<'a> {
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
pub struct _COISTW<'a> {
    w: &'a mut W,
}
impl<'a> _COISTW<'a> {
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
#[doc = "Values that can be written to the field `CMOA`"]
pub enum CMOAW {
    #[doc = "No action on compare match"]
    NONE,
    #[doc = "Toggle output on compare match"]
    TOGGLE,
    #[doc = "Clear output on compare match"]
    CLEAR,
    #[doc = "Set output on compare match"]
    SET,
}
impl CMOAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMOAW::NONE => 0,
            CMOAW::TOGGLE => 1,
            CMOAW::CLEAR => 2,
            CMOAW::SET => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMOAW<'a> {
    w: &'a mut W,
}
impl<'a> _CMOAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMOAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action on compare match"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(CMOAW::NONE)
    }
    #[doc = "Toggle output on compare match"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CMOAW::TOGGLE)
    }
    #[doc = "Clear output on compare match"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMOAW::CLEAR)
    }
    #[doc = "Set output on compare match"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CMOAW::SET)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COFOA`"]
pub enum COFOAW {
    #[doc = "No action on counter overflow"]
    NONE,
    #[doc = "Toggle output on counter overflow"]
    TOGGLE,
    #[doc = "Clear output on counter overflow"]
    CLEAR,
    #[doc = "Set output on counter overflow"]
    SET,
}
impl COFOAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COFOAW::NONE => 0,
            COFOAW::TOGGLE => 1,
            COFOAW::CLEAR => 2,
            COFOAW::SET => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COFOAW<'a> {
    w: &'a mut W,
}
impl<'a> _COFOAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COFOAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action on counter overflow"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(COFOAW::NONE)
    }
    #[doc = "Toggle output on counter overflow"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(COFOAW::TOGGLE)
    }
    #[doc = "Clear output on counter overflow"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(COFOAW::CLEAR)
    }
    #[doc = "Set output on counter overflow"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(COFOAW::SET)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CUFOA`"]
pub enum CUFOAW {
    #[doc = "No action on counter underflow"]
    NONE,
    #[doc = "Toggle output on counter underflow"]
    TOGGLE,
    #[doc = "Clear output on counter underflow"]
    CLEAR,
    #[doc = "Set output on counter underflow"]
    SET,
}
impl CUFOAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CUFOAW::NONE => 0,
            CUFOAW::TOGGLE => 1,
            CUFOAW::CLEAR => 2,
            CUFOAW::SET => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CUFOAW<'a> {
    w: &'a mut W,
}
impl<'a> _CUFOAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CUFOAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action on counter underflow"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(CUFOAW::NONE)
    }
    #[doc = "Toggle output on counter underflow"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CUFOAW::TOGGLE)
    }
    #[doc = "Clear output on counter underflow"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CUFOAW::CLEAR)
    }
    #[doc = "Set output on counter underflow"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CUFOAW::SET)
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
#[doc = "Values that can be written to the field `PRSSEL`"]
pub enum PRSSELW {
    #[doc = "PRS Channel 0 selected as input"]
    PRSCH0,
    #[doc = "PRS Channel 1 selected as input"]
    PRSCH1,
    #[doc = "PRS Channel 2 selected as input"]
    PRSCH2,
    #[doc = "PRS Channel 3 selected as input"]
    PRSCH3,
    #[doc = "PRS Channel 4 selected as input"]
    PRSCH4,
    #[doc = "PRS Channel 5 selected as input"]
    PRSCH5,
}
impl PRSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRSSELW::PRSCH0 => 0,
            PRSSELW::PRSCH1 => 1,
            PRSSELW::PRSCH2 => 2,
            PRSSELW::PRSCH3 => 3,
            PRSSELW::PRSCH4 => 4,
            PRSSELW::PRSCH5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRSSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSELW::PRSCH5)
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
#[doc = r" Proxy"]
pub struct _INSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INSELW<'a> {
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
pub struct _FILTW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTW<'a> {
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
#[doc = "Values that can be written to the field `ICEDGE`"]
pub enum ICEDGEW {
    #[doc = "Rising edges detected"]
    RISING,
    #[doc = "Falling edges detected"]
    FALLING,
    #[doc = "Both edges detected"]
    BOTH,
    #[doc = "No edge detection, signal is left as it is"]
    NONE,
}
impl ICEDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICEDGEW::RISING => 0,
            ICEDGEW::FALLING => 1,
            ICEDGEW::BOTH => 2,
            ICEDGEW::NONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _ICEDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICEDGEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Rising edges detected"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(ICEDGEW::RISING)
    }
    #[doc = "Falling edges detected"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(ICEDGEW::FALLING)
    }
    #[doc = "Both edges detected"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(ICEDGEW::BOTH)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ICEDGEW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICEVCTRL`"]
pub enum ICEVCTRLW {
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every capture"]
    EVERYEDGE,
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every second capture"]
    EVERYSECONDEDGE,
    #[doc = "PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    RISING,
    #[doc = "PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    FALLING,
}
impl ICEVCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICEVCTRLW::EVERYEDGE => 0,
            ICEVCTRLW::EVERYSECONDEDGE => 1,
            ICEVCTRLW::RISING => 2,
            ICEVCTRLW::FALLING => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICEVCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ICEVCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICEVCTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every capture"]
    #[inline]
    pub fn everyedge(self) -> &'a mut W {
        self.variant(ICEVCTRLW::EVERYEDGE)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every second capture"]
    #[inline]
    pub fn everysecondedge(self) -> &'a mut W {
        self.variant(ICEVCTRLW::EVERYSECONDEDGE)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(ICEVCTRLW::RISING)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(ICEVCTRLW::FALLING)
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
pub struct _PRSCONFW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSCONFW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline]
    pub fn outinv(&self) -> OUTINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTINVR { bits }
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline]
    pub fn coist(&self) -> COISTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COISTR { bits }
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline]
    pub fn cmoa(&self) -> CMOAR {
        CMOAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline]
    pub fn cofoa(&self) -> COFOAR {
        COFOAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline]
    pub fn cufoa(&self) -> CUFOAR {
        CUFOAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline]
    pub fn prssel(&self) -> PRSSELR {
        PRSSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Input Selection"]
    #[inline]
    pub fn insel(&self) -> INSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INSELR { bits }
    }
    #[doc = "Bit 21 - Digital Filter"]
    #[inline]
    pub fn filt(&self) -> FILTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTR { bits }
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline]
    pub fn icedge(&self) -> ICEDGER {
        ICEDGER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline]
    pub fn icevctrl(&self) -> ICEVCTRLR {
        ICEVCTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - PRS Configuration"]
    #[inline]
    pub fn prsconf(&self) -> PRSCONFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSCONFR { bits }
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
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline]
    pub fn outinv(&mut self) -> _OUTINVW {
        _OUTINVW { w: self }
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline]
    pub fn coist(&mut self) -> _COISTW {
        _COISTW { w: self }
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline]
    pub fn cmoa(&mut self) -> _CMOAW {
        _CMOAW { w: self }
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline]
    pub fn cofoa(&mut self) -> _COFOAW {
        _COFOAW { w: self }
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline]
    pub fn cufoa(&mut self) -> _CUFOAW {
        _CUFOAW { w: self }
    }
    #[doc = "Bits 16:18 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline]
    pub fn prssel(&mut self) -> _PRSSELW {
        _PRSSELW { w: self }
    }
    #[doc = "Bit 20 - Input Selection"]
    #[inline]
    pub fn insel(&mut self) -> _INSELW {
        _INSELW { w: self }
    }
    #[doc = "Bit 21 - Digital Filter"]
    #[inline]
    pub fn filt(&mut self) -> _FILTW {
        _FILTW { w: self }
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline]
    pub fn icedge(&mut self) -> _ICEDGEW {
        _ICEDGEW { w: self }
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline]
    pub fn icevctrl(&mut self) -> _ICEVCTRLW {
        _ICEVCTRLW { w: self }
    }
    #[doc = "Bit 28 - PRS Configuration"]
    #[inline]
    pub fn prsconf(&mut self) -> _PRSCONFW {
        _PRSCONFW { w: self }
    }
}
