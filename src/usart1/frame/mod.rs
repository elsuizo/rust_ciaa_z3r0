#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRAME {
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
#[doc = "Possible values of the field `DATABITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATABITSR {
    #[doc = "Each frame contains 4 data bits"]
    FOUR,
    #[doc = "Each frame contains 5 data bits"]
    FIVE,
    #[doc = "Each frame contains 6 data bits"]
    SIX,
    #[doc = "Each frame contains 7 data bits"]
    SEVEN,
    #[doc = "Each frame contains 8 data bits"]
    EIGHT,
    #[doc = "Each frame contains 9 data bits"]
    NINE,
    #[doc = "Each frame contains 10 data bits"]
    TEN,
    #[doc = "Each frame contains 11 data bits"]
    ELEVEN,
    #[doc = "Each frame contains 12 data bits"]
    TWELVE,
    #[doc = "Each frame contains 13 data bits"]
    THIRTEEN,
    #[doc = "Each frame contains 14 data bits"]
    FOURTEEN,
    #[doc = "Each frame contains 15 data bits"]
    FIFTEEN,
    #[doc = "Each frame contains 16 data bits"]
    SIXTEEN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATABITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATABITSR::FOUR => 1,
            DATABITSR::FIVE => 2,
            DATABITSR::SIX => 3,
            DATABITSR::SEVEN => 4,
            DATABITSR::EIGHT => 5,
            DATABITSR::NINE => 6,
            DATABITSR::TEN => 7,
            DATABITSR::ELEVEN => 8,
            DATABITSR::TWELVE => 9,
            DATABITSR::THIRTEEN => 10,
            DATABITSR::FOURTEEN => 11,
            DATABITSR::FIFTEEN => 12,
            DATABITSR::SIXTEEN => 13,
            DATABITSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATABITSR {
        match value {
            1 => DATABITSR::FOUR,
            2 => DATABITSR::FIVE,
            3 => DATABITSR::SIX,
            4 => DATABITSR::SEVEN,
            5 => DATABITSR::EIGHT,
            6 => DATABITSR::NINE,
            7 => DATABITSR::TEN,
            8 => DATABITSR::ELEVEN,
            9 => DATABITSR::TWELVE,
            10 => DATABITSR::THIRTEEN,
            11 => DATABITSR::FOURTEEN,
            12 => DATABITSR::FIFTEEN,
            13 => DATABITSR::SIXTEEN,
            i => DATABITSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline]
    pub fn is_four(&self) -> bool {
        *self == DATABITSR::FOUR
    }
    #[doc = "Checks if the value of the field is `FIVE`"]
    #[inline]
    pub fn is_five(&self) -> bool {
        *self == DATABITSR::FIVE
    }
    #[doc = "Checks if the value of the field is `SIX`"]
    #[inline]
    pub fn is_six(&self) -> bool {
        *self == DATABITSR::SIX
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline]
    pub fn is_seven(&self) -> bool {
        *self == DATABITSR::SEVEN
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline]
    pub fn is_eight(&self) -> bool {
        *self == DATABITSR::EIGHT
    }
    #[doc = "Checks if the value of the field is `NINE`"]
    #[inline]
    pub fn is_nine(&self) -> bool {
        *self == DATABITSR::NINE
    }
    #[doc = "Checks if the value of the field is `TEN`"]
    #[inline]
    pub fn is_ten(&self) -> bool {
        *self == DATABITSR::TEN
    }
    #[doc = "Checks if the value of the field is `ELEVEN`"]
    #[inline]
    pub fn is_eleven(&self) -> bool {
        *self == DATABITSR::ELEVEN
    }
    #[doc = "Checks if the value of the field is `TWELVE`"]
    #[inline]
    pub fn is_twelve(&self) -> bool {
        *self == DATABITSR::TWELVE
    }
    #[doc = "Checks if the value of the field is `THIRTEEN`"]
    #[inline]
    pub fn is_thirteen(&self) -> bool {
        *self == DATABITSR::THIRTEEN
    }
    #[doc = "Checks if the value of the field is `FOURTEEN`"]
    #[inline]
    pub fn is_fourteen(&self) -> bool {
        *self == DATABITSR::FOURTEEN
    }
    #[doc = "Checks if the value of the field is `FIFTEEN`"]
    #[inline]
    pub fn is_fifteen(&self) -> bool {
        *self == DATABITSR::FIFTEEN
    }
    #[doc = "Checks if the value of the field is `SIXTEEN`"]
    #[inline]
    pub fn is_sixteen(&self) -> bool {
        *self == DATABITSR::SIXTEEN
    }
}
#[doc = "Possible values of the field `PARITY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYR {
    #[doc = "Parity bits are not used"]
    NONE,
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    EVEN,
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    ODD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PARITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARITYR::NONE => 0,
            PARITYR::EVEN => 2,
            PARITYR::ODD => 3,
            PARITYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARITYR {
        match value {
            0 => PARITYR::NONE,
            2 => PARITYR::EVEN,
            3 => PARITYR::ODD,
            i => PARITYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PARITYR::NONE
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == PARITYR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == PARITYR::ODD
    }
}
#[doc = "Possible values of the field `STOPBITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPBITSR {
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    HALF,
    #[doc = "One stop bit is generated and verified"]
    ONE,
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    ONEANDAHALF,
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    TWO,
}
impl STOPBITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STOPBITSR::HALF => 0,
            STOPBITSR::ONE => 1,
            STOPBITSR::ONEANDAHALF => 2,
            STOPBITSR::TWO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STOPBITSR {
        match value {
            0 => STOPBITSR::HALF,
            1 => STOPBITSR::ONE,
            2 => STOPBITSR::ONEANDAHALF,
            3 => STOPBITSR::TWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == STOPBITSR::HALF
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == STOPBITSR::ONE
    }
    #[doc = "Checks if the value of the field is `ONEANDAHALF`"]
    #[inline]
    pub fn is_oneandahalf(&self) -> bool {
        *self == STOPBITSR::ONEANDAHALF
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == STOPBITSR::TWO
    }
}
#[doc = "Values that can be written to the field `DATABITS`"]
pub enum DATABITSW {
    #[doc = "Each frame contains 4 data bits"]
    FOUR,
    #[doc = "Each frame contains 5 data bits"]
    FIVE,
    #[doc = "Each frame contains 6 data bits"]
    SIX,
    #[doc = "Each frame contains 7 data bits"]
    SEVEN,
    #[doc = "Each frame contains 8 data bits"]
    EIGHT,
    #[doc = "Each frame contains 9 data bits"]
    NINE,
    #[doc = "Each frame contains 10 data bits"]
    TEN,
    #[doc = "Each frame contains 11 data bits"]
    ELEVEN,
    #[doc = "Each frame contains 12 data bits"]
    TWELVE,
    #[doc = "Each frame contains 13 data bits"]
    THIRTEEN,
    #[doc = "Each frame contains 14 data bits"]
    FOURTEEN,
    #[doc = "Each frame contains 15 data bits"]
    FIFTEEN,
    #[doc = "Each frame contains 16 data bits"]
    SIXTEEN,
}
impl DATABITSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATABITSW::FOUR => 1,
            DATABITSW::FIVE => 2,
            DATABITSW::SIX => 3,
            DATABITSW::SEVEN => 4,
            DATABITSW::EIGHT => 5,
            DATABITSW::NINE => 6,
            DATABITSW::TEN => 7,
            DATABITSW::ELEVEN => 8,
            DATABITSW::TWELVE => 9,
            DATABITSW::THIRTEEN => 10,
            DATABITSW::FOURTEEN => 11,
            DATABITSW::FIFTEEN => 12,
            DATABITSW::SIXTEEN => 13,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATABITSW<'a> {
    w: &'a mut W,
}
impl<'a> _DATABITSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATABITSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Each frame contains 4 data bits"]
    #[inline]
    pub fn four(self) -> &'a mut W {
        self.variant(DATABITSW::FOUR)
    }
    #[doc = "Each frame contains 5 data bits"]
    #[inline]
    pub fn five(self) -> &'a mut W {
        self.variant(DATABITSW::FIVE)
    }
    #[doc = "Each frame contains 6 data bits"]
    #[inline]
    pub fn six(self) -> &'a mut W {
        self.variant(DATABITSW::SIX)
    }
    #[doc = "Each frame contains 7 data bits"]
    #[inline]
    pub fn seven(self) -> &'a mut W {
        self.variant(DATABITSW::SEVEN)
    }
    #[doc = "Each frame contains 8 data bits"]
    #[inline]
    pub fn eight(self) -> &'a mut W {
        self.variant(DATABITSW::EIGHT)
    }
    #[doc = "Each frame contains 9 data bits"]
    #[inline]
    pub fn nine(self) -> &'a mut W {
        self.variant(DATABITSW::NINE)
    }
    #[doc = "Each frame contains 10 data bits"]
    #[inline]
    pub fn ten(self) -> &'a mut W {
        self.variant(DATABITSW::TEN)
    }
    #[doc = "Each frame contains 11 data bits"]
    #[inline]
    pub fn eleven(self) -> &'a mut W {
        self.variant(DATABITSW::ELEVEN)
    }
    #[doc = "Each frame contains 12 data bits"]
    #[inline]
    pub fn twelve(self) -> &'a mut W {
        self.variant(DATABITSW::TWELVE)
    }
    #[doc = "Each frame contains 13 data bits"]
    #[inline]
    pub fn thirteen(self) -> &'a mut W {
        self.variant(DATABITSW::THIRTEEN)
    }
    #[doc = "Each frame contains 14 data bits"]
    #[inline]
    pub fn fourteen(self) -> &'a mut W {
        self.variant(DATABITSW::FOURTEEN)
    }
    #[doc = "Each frame contains 15 data bits"]
    #[inline]
    pub fn fifteen(self) -> &'a mut W {
        self.variant(DATABITSW::FIFTEEN)
    }
    #[doc = "Each frame contains 16 data bits"]
    #[inline]
    pub fn sixteen(self) -> &'a mut W {
        self.variant(DATABITSW::SIXTEEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PARITY`"]
pub enum PARITYW {
    #[doc = "Parity bits are not used"]
    NONE,
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    EVEN,
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    ODD,
}
impl PARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARITYW::NONE => 0,
            PARITYW::EVEN => 2,
            PARITYW::ODD => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Parity bits are not used"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PARITYW::NONE)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(PARITYW::EVEN)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARITYW::ODD)
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
#[doc = "Values that can be written to the field `STOPBITS`"]
pub enum STOPBITSW {
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    HALF,
    #[doc = "One stop bit is generated and verified"]
    ONE,
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    ONEANDAHALF,
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    TWO,
}
impl STOPBITSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STOPBITSW::HALF => 0,
            STOPBITSW::ONE => 1,
            STOPBITSW::ONEANDAHALF => 2,
            STOPBITSW::TWO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPBITSW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPBITSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPBITSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    #[inline]
    pub fn half(self) -> &'a mut W {
        self.variant(STOPBITSW::HALF)
    }
    #[doc = "One stop bit is generated and verified"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(STOPBITSW::ONE)
    }
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    #[inline]
    pub fn oneandahalf(self) -> &'a mut W {
        self.variant(STOPBITSW::ONEANDAHALF)
    }
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(STOPBITSW::TWO)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline]
    pub fn databits(&self) -> DATABITSR {
        DATABITSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline]
    pub fn parity(&self) -> PARITYR {
        PARITYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline]
    pub fn stopbits(&self) -> STOPBITSR {
        STOPBITSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4101 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline]
    pub fn databits(&mut self) -> _DATABITSW {
        _DATABITSW { w: self }
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline]
    pub fn parity(&mut self) -> _PARITYW {
        _PARITYW { w: self }
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline]
    pub fn stopbits(&mut self) -> _STOPBITSW {
        _STOPBITSW { w: self }
    }
}
