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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Up-count mode"]
    UP,
    #[doc = "Down-count mode"]
    DOWN,
    #[doc = "Up/down-count mode"]
    UPDOWN,
    #[doc = "Quadrature decoder mode"]
    QDEC,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::UP => 0,
            MODER::DOWN => 1,
            MODER::UPDOWN => 2,
            MODER::QDEC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::UP,
            1 => MODER::DOWN,
            2 => MODER::UPDOWN,
            3 => MODER::QDEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == MODER::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline]
    pub fn is_down(&self) -> bool {
        *self == MODER::DOWN
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline]
    pub fn is_updown(&self) -> bool {
        *self == MODER::UPDOWN
    }
    #[doc = "Checks if the value of the field is `QDEC`"]
    #[inline]
    pub fn is_qdec(&self) -> bool {
        *self == MODER::QDEC
    }
}
#[doc = r" Value of the field"]
pub struct SYNCR {
    bits: bool,
}
impl SYNCR {
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
pub struct OSMENR {
    bits: bool,
}
impl OSMENR {
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
pub struct QDMR {
    bits: bool,
}
impl QDMR {
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
pub struct DEBUGRUNR {
    bits: bool,
}
impl DEBUGRUNR {
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
pub struct DMACLRACTR {
    bits: bool,
}
impl DMACLRACTR {
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
#[doc = "Possible values of the field `RISEA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RISEAR {
    #[doc = "No action"]
    NONE,
    #[doc = "Start counter without reload"]
    START,
    #[doc = "Stop counter without reload"]
    STOP,
    #[doc = "Reload and start counter"]
    RELOADSTART,
}
impl RISEAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RISEAR::NONE => 0,
            RISEAR::START => 1,
            RISEAR::STOP => 2,
            RISEAR::RELOADSTART => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RISEAR {
        match value {
            0 => RISEAR::NONE,
            1 => RISEAR::START,
            2 => RISEAR::STOP,
            3 => RISEAR::RELOADSTART,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == RISEAR::NONE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == RISEAR::START
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == RISEAR::STOP
    }
    #[doc = "Checks if the value of the field is `RELOADSTART`"]
    #[inline]
    pub fn is_reloadstart(&self) -> bool {
        *self == RISEAR::RELOADSTART
    }
}
#[doc = "Possible values of the field `FALLA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FALLAR {
    #[doc = "No action"]
    NONE,
    #[doc = "Start counter without reload"]
    START,
    #[doc = "Stop counter without reload"]
    STOP,
    #[doc = "Reload and start counter"]
    RELOADSTART,
}
impl FALLAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FALLAR::NONE => 0,
            FALLAR::START => 1,
            FALLAR::STOP => 2,
            FALLAR::RELOADSTART => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FALLAR {
        match value {
            0 => FALLAR::NONE,
            1 => FALLAR::START,
            2 => FALLAR::STOP,
            3 => FALLAR::RELOADSTART,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == FALLAR::NONE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == FALLAR::START
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == FALLAR::STOP
    }
    #[doc = "Checks if the value of the field is `RELOADSTART`"]
    #[inline]
    pub fn is_reloadstart(&self) -> bool {
        *self == FALLAR::RELOADSTART
    }
}
#[doc = r" Value of the field"]
pub struct X2CNTR {
    bits: bool,
}
impl X2CNTR {
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
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "Prescaled HFPERCLK"]
    PRESCHFPERCLK,
    #[doc = "Compare/Capture Channel 1 Input"]
    CC1,
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    TIMEROUF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSELR::PRESCHFPERCLK => 0,
            CLKSELR::CC1 => 1,
            CLKSELR::TIMEROUF => 2,
            CLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSELR {
        match value {
            0 => CLKSELR::PRESCHFPERCLK,
            1 => CLKSELR::CC1,
            2 => CLKSELR::TIMEROUF,
            i => CLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCHFPERCLK`"]
    #[inline]
    pub fn is_preschfperclk(&self) -> bool {
        *self == CLKSELR::PRESCHFPERCLK
    }
    #[doc = "Checks if the value of the field is `CC1`"]
    #[inline]
    pub fn is_cc1(&self) -> bool {
        *self == CLKSELR::CC1
    }
    #[doc = "Checks if the value of the field is `TIMEROUF`"]
    #[inline]
    pub fn is_timerouf(&self) -> bool {
        *self == CLKSELR::TIMEROUF
    }
}
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "The HFPERCLK is undivided"]
    DIV1,
    #[doc = "The HFPERCLK is divided by 2"]
    DIV2,
    #[doc = "The HFPERCLK is divided by 4"]
    DIV4,
    #[doc = "The HFPERCLK is divided by 8"]
    DIV8,
    #[doc = "The HFPERCLK is divided by 16"]
    DIV16,
    #[doc = "The HFPERCLK is divided by 32"]
    DIV32,
    #[doc = "The HFPERCLK is divided by 64"]
    DIV64,
    #[doc = "The HFPERCLK is divided by 128"]
    DIV128,
    #[doc = "The HFPERCLK is divided by 256"]
    DIV256,
    #[doc = "The HFPERCLK is divided by 512"]
    DIV512,
    #[doc = "The HFPERCLK is divided by 1024"]
    DIV1024,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::DIV1 => 0,
            PRESCR::DIV2 => 1,
            PRESCR::DIV4 => 2,
            PRESCR::DIV8 => 3,
            PRESCR::DIV16 => 4,
            PRESCR::DIV32 => 5,
            PRESCR::DIV64 => 6,
            PRESCR::DIV128 => 7,
            PRESCR::DIV256 => 8,
            PRESCR::DIV512 => 9,
            PRESCR::DIV1024 => 10,
            PRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::DIV1,
            1 => PRESCR::DIV2,
            2 => PRESCR::DIV4,
            3 => PRESCR::DIV8,
            4 => PRESCR::DIV16,
            5 => PRESCR::DIV32,
            6 => PRESCR::DIV64,
            7 => PRESCR::DIV128,
            8 => PRESCR::DIV256,
            9 => PRESCR::DIV512,
            10 => PRESCR::DIV1024,
            i => PRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PRESCR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == PRESCR::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCR::DIV1024
    }
}
#[doc = r" Value of the field"]
pub struct ATIR {
    bits: bool,
}
impl ATIR {
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
pub struct RSSCOISTR {
    bits: bool,
}
impl RSSCOISTR {
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
    #[doc = "Up-count mode"]
    UP,
    #[doc = "Down-count mode"]
    DOWN,
    #[doc = "Up/down-count mode"]
    UPDOWN,
    #[doc = "Quadrature decoder mode"]
    QDEC,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::UP => 0,
            MODEW::DOWN => 1,
            MODEW::UPDOWN => 2,
            MODEW::QDEC => 3,
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
    #[doc = "Up-count mode"]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(MODEW::UP)
    }
    #[doc = "Down-count mode"]
    #[inline]
    pub fn down(self) -> &'a mut W {
        self.variant(MODEW::DOWN)
    }
    #[doc = "Up/down-count mode"]
    #[inline]
    pub fn updown(self) -> &'a mut W {
        self.variant(MODEW::UPDOWN)
    }
    #[doc = "Quadrature decoder mode"]
    #[inline]
    pub fn qdec(self) -> &'a mut W {
        self.variant(MODEW::QDEC)
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
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
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
pub struct _OSMENW<'a> {
    w: &'a mut W,
}
impl<'a> _OSMENW<'a> {
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
pub struct _QDMW<'a> {
    w: &'a mut W,
}
impl<'a> _QDMW<'a> {
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
pub struct _DEBUGRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGRUNW<'a> {
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
pub struct _DMACLRACTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMACLRACTW<'a> {
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
#[doc = "Values that can be written to the field `RISEA`"]
pub enum RISEAW {
    #[doc = "No action"]
    NONE,
    #[doc = "Start counter without reload"]
    START,
    #[doc = "Stop counter without reload"]
    STOP,
    #[doc = "Reload and start counter"]
    RELOADSTART,
}
impl RISEAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RISEAW::NONE => 0,
            RISEAW::START => 1,
            RISEAW::STOP => 2,
            RISEAW::RELOADSTART => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RISEAW<'a> {
    w: &'a mut W,
}
impl<'a> _RISEAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RISEAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(RISEAW::NONE)
    }
    #[doc = "Start counter without reload"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(RISEAW::START)
    }
    #[doc = "Stop counter without reload"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(RISEAW::STOP)
    }
    #[doc = "Reload and start counter"]
    #[inline]
    pub fn reloadstart(self) -> &'a mut W {
        self.variant(RISEAW::RELOADSTART)
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
#[doc = "Values that can be written to the field `FALLA`"]
pub enum FALLAW {
    #[doc = "No action"]
    NONE,
    #[doc = "Start counter without reload"]
    START,
    #[doc = "Stop counter without reload"]
    STOP,
    #[doc = "Reload and start counter"]
    RELOADSTART,
}
impl FALLAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FALLAW::NONE => 0,
            FALLAW::START => 1,
            FALLAW::STOP => 2,
            FALLAW::RELOADSTART => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FALLAW<'a> {
    w: &'a mut W,
}
impl<'a> _FALLAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FALLAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(FALLAW::NONE)
    }
    #[doc = "Start counter without reload"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(FALLAW::START)
    }
    #[doc = "Stop counter without reload"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(FALLAW::STOP)
    }
    #[doc = "Reload and start counter"]
    #[inline]
    pub fn reloadstart(self) -> &'a mut W {
        self.variant(FALLAW::RELOADSTART)
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
#[doc = r" Proxy"]
pub struct _X2CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _X2CNTW<'a> {
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
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "Prescaled HFPERCLK"]
    PRESCHFPERCLK,
    #[doc = "Compare/Capture Channel 1 Input"]
    CC1,
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    TIMEROUF,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSELW::PRESCHFPERCLK => 0,
            CLKSELW::CC1 => 1,
            CLKSELW::TIMEROUF => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Prescaled HFPERCLK"]
    #[inline]
    pub fn preschfperclk(self) -> &'a mut W {
        self.variant(CLKSELW::PRESCHFPERCLK)
    }
    #[doc = "Compare/Capture Channel 1 Input"]
    #[inline]
    pub fn cc1(self) -> &'a mut W {
        self.variant(CLKSELW::CC1)
    }
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    #[inline]
    pub fn timerouf(self) -> &'a mut W {
        self.variant(CLKSELW::TIMEROUF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
pub enum PRESCW {
    #[doc = "The HFPERCLK is undivided"]
    DIV1,
    #[doc = "The HFPERCLK is divided by 2"]
    DIV2,
    #[doc = "The HFPERCLK is divided by 4"]
    DIV4,
    #[doc = "The HFPERCLK is divided by 8"]
    DIV8,
    #[doc = "The HFPERCLK is divided by 16"]
    DIV16,
    #[doc = "The HFPERCLK is divided by 32"]
    DIV32,
    #[doc = "The HFPERCLK is divided by 64"]
    DIV64,
    #[doc = "The HFPERCLK is divided by 128"]
    DIV128,
    #[doc = "The HFPERCLK is divided by 256"]
    DIV256,
    #[doc = "The HFPERCLK is divided by 512"]
    DIV512,
    #[doc = "The HFPERCLK is divided by 1024"]
    DIV1024,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::DIV1 => 0,
            PRESCW::DIV2 => 1,
            PRESCW::DIV4 => 2,
            PRESCW::DIV8 => 3,
            PRESCW::DIV16 => 4,
            PRESCW::DIV32 => 5,
            PRESCW::DIV64 => 6,
            PRESCW::DIV128 => 7,
            PRESCW::DIV256 => 8,
            PRESCW::DIV512 => 9,
            PRESCW::DIV1024 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The HFPERCLK is undivided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCW::DIV1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCW::DIV2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCW::DIV4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCW::DIV8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCW::DIV16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCW::DIV32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCW::DIV64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCW::DIV128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCW::DIV256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCW::DIV512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCW::DIV1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ATIW<'a> {
    w: &'a mut W,
}
impl<'a> _ATIW<'a> {
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
#[doc = r" Proxy"]
pub struct _RSSCOISTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSSCOISTW<'a> {
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNCR { bits }
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline]
    pub fn osmen(&self) -> OSMENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSMENR { bits }
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline]
    pub fn qdm(&self) -> QDMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        QDMR { bits }
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&self) -> DEBUGRUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUGRUNR { bits }
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline]
    pub fn dmaclract(&self) -> DMACLRACTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMACLRACTR { bits }
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline]
    pub fn risea(&self) -> RISEAR {
        RISEAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline]
    pub fn falla(&self) -> FALLAR {
        FALLAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline]
    pub fn x2cnt(&self) -> X2CNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        X2CNTR { bits }
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline]
    pub fn ati(&self) -> ATIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ATIR { bits }
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output initial State"]
    #[inline]
    pub fn rsscoist(&self) -> RSSCOISTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSSCOISTR { bits }
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
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline]
    pub fn osmen(&mut self) -> _OSMENW {
        _OSMENW { w: self }
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline]
    pub fn qdm(&mut self) -> _QDMW {
        _QDMW { w: self }
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline]
    pub fn debugrun(&mut self) -> _DEBUGRUNW {
        _DEBUGRUNW { w: self }
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline]
    pub fn dmaclract(&mut self) -> _DMACLRACTW {
        _DMACLRACTW { w: self }
    }
    #[doc = "Bits 8:9 - Timer Rising Input Edge Action"]
    #[inline]
    pub fn risea(&mut self) -> _RISEAW {
        _RISEAW { w: self }
    }
    #[doc = "Bits 10:11 - Timer Falling Input Edge Action"]
    #[inline]
    pub fn falla(&mut self) -> _FALLAW {
        _FALLAW { w: self }
    }
    #[doc = "Bit 13 - 2x Count Mode"]
    #[inline]
    pub fn x2cnt(&mut self) -> _X2CNTW {
        _X2CNTW { w: self }
    }
    #[doc = "Bits 16:17 - Clock Source Select"]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bits 24:27 - Prescaler Setting"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
    #[doc = "Bit 28 - Always Track Inputs"]
    #[inline]
    pub fn ati(&mut self) -> _ATIW {
        _ATIW { w: self }
    }
    #[doc = "Bit 29 - Reload-Start Sets Compare Output initial State"]
    #[inline]
    pub fn rsscoist(&mut self) -> _RSSCOISTW {
        _RSSCOISTW { w: self }
    }
}
