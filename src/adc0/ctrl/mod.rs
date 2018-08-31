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
#[doc = "Possible values of the field `WARMUPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARMUPMODER {
    #[doc = "ADC is shut down after each conversion"]
    NORMAL,
    #[doc = "Bandgap references do not need warm up, but have reduced accuracy."]
    FASTBG,
    #[doc = "Reference selected for scan mode is kept warm."]
    KEEPSCANREFWARM,
    #[doc = "ADC is kept warmed up and scan reference is kept warm"]
    KEEPADCWARM,
}
impl WARMUPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WARMUPMODER::NORMAL => 0,
            WARMUPMODER::FASTBG => 1,
            WARMUPMODER::KEEPSCANREFWARM => 2,
            WARMUPMODER::KEEPADCWARM => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WARMUPMODER {
        match value {
            0 => WARMUPMODER::NORMAL,
            1 => WARMUPMODER::FASTBG,
            2 => WARMUPMODER::KEEPSCANREFWARM,
            3 => WARMUPMODER::KEEPADCWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `FASTBG`"]
    #[inline]
    pub fn is_fastbg(&self) -> bool {
        *self == WARMUPMODER::FASTBG
    }
    #[doc = "Checks if the value of the field is `KEEPSCANREFWARM`"]
    #[inline]
    pub fn is_keepscanrefwarm(&self) -> bool {
        *self == WARMUPMODER::KEEPSCANREFWARM
    }
    #[doc = "Checks if the value of the field is `KEEPADCWARM`"]
    #[inline]
    pub fn is_keepadcwarm(&self) -> bool {
        *self == WARMUPMODER::KEEPADCWARM
    }
}
#[doc = r" Value of the field"]
pub struct TAILGATER {
    bits: bool,
}
impl TAILGATER {
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
#[doc = "Possible values of the field `LPFMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPFMODER {
    #[doc = "No filter or decoupling capacitor"]
    BYPASS,
    #[doc = "On chip decoupling capacitor selected"]
    DECAP,
    #[doc = "On chip RC filter selected"]
    RCFILT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPFMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPFMODER::BYPASS => 0,
            LPFMODER::DECAP => 1,
            LPFMODER::RCFILT => 2,
            LPFMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPFMODER {
        match value {
            0 => LPFMODER::BYPASS,
            1 => LPFMODER::DECAP,
            2 => LPFMODER::RCFILT,
            i => LPFMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == LPFMODER::BYPASS
    }
    #[doc = "Checks if the value of the field is `DECAP`"]
    #[inline]
    pub fn is_decap(&self) -> bool {
        *self == LPFMODER::DECAP
    }
    #[doc = "Checks if the value of the field is `RCFILT`"]
    #[inline]
    pub fn is_rcfilt(&self) -> bool {
        *self == LPFMODER::RCFILT
    }
}
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "\"\""]
    NODIVISION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::NODIVISION => 0,
            PRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::NODIVISION,
            i => PRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESCR::NODIVISION
    }
}
#[doc = r" Value of the field"]
pub struct TIMEBASER {
    bits: u8,
}
impl TIMEBASER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `OVSRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSRSELR {
    #[doc = "2 samples for each conversion result"]
    X2,
    #[doc = "4 samples for each conversion result"]
    X4,
    #[doc = "8 samples for each conversion result"]
    X8,
    #[doc = "16 samples for each conversion result"]
    X16,
    #[doc = "32 samples for each conversion result"]
    X32,
    #[doc = "64 samples for each conversion result"]
    X64,
    #[doc = "128 samples for each conversion result"]
    X128,
    #[doc = "256 samples for each conversion result"]
    X256,
    #[doc = "512 samples for each conversion result"]
    X512,
    #[doc = "1024 samples for each conversion result"]
    X1024,
    #[doc = "2048 samples for each conversion result"]
    X2048,
    #[doc = "4096 samples for each conversion result"]
    X4096,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OVSRSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OVSRSELR::X2 => 0,
            OVSRSELR::X4 => 1,
            OVSRSELR::X8 => 2,
            OVSRSELR::X16 => 3,
            OVSRSELR::X32 => 4,
            OVSRSELR::X64 => 5,
            OVSRSELR::X128 => 6,
            OVSRSELR::X256 => 7,
            OVSRSELR::X512 => 8,
            OVSRSELR::X1024 => 9,
            OVSRSELR::X2048 => 10,
            OVSRSELR::X4096 => 11,
            OVSRSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OVSRSELR {
        match value {
            0 => OVSRSELR::X2,
            1 => OVSRSELR::X4,
            2 => OVSRSELR::X8,
            3 => OVSRSELR::X16,
            4 => OVSRSELR::X32,
            5 => OVSRSELR::X64,
            6 => OVSRSELR::X128,
            7 => OVSRSELR::X256,
            8 => OVSRSELR::X512,
            9 => OVSRSELR::X1024,
            10 => OVSRSELR::X2048,
            11 => OVSRSELR::X4096,
            i => OVSRSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline]
    pub fn is_x2(&self) -> bool {
        *self == OVSRSELR::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline]
    pub fn is_x4(&self) -> bool {
        *self == OVSRSELR::X4
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline]
    pub fn is_x8(&self) -> bool {
        *self == OVSRSELR::X8
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline]
    pub fn is_x16(&self) -> bool {
        *self == OVSRSELR::X16
    }
    #[doc = "Checks if the value of the field is `X32`"]
    #[inline]
    pub fn is_x32(&self) -> bool {
        *self == OVSRSELR::X32
    }
    #[doc = "Checks if the value of the field is `X64`"]
    #[inline]
    pub fn is_x64(&self) -> bool {
        *self == OVSRSELR::X64
    }
    #[doc = "Checks if the value of the field is `X128`"]
    #[inline]
    pub fn is_x128(&self) -> bool {
        *self == OVSRSELR::X128
    }
    #[doc = "Checks if the value of the field is `X256`"]
    #[inline]
    pub fn is_x256(&self) -> bool {
        *self == OVSRSELR::X256
    }
    #[doc = "Checks if the value of the field is `X512`"]
    #[inline]
    pub fn is_x512(&self) -> bool {
        *self == OVSRSELR::X512
    }
    #[doc = "Checks if the value of the field is `X1024`"]
    #[inline]
    pub fn is_x1024(&self) -> bool {
        *self == OVSRSELR::X1024
    }
    #[doc = "Checks if the value of the field is `X2048`"]
    #[inline]
    pub fn is_x2048(&self) -> bool {
        *self == OVSRSELR::X2048
    }
    #[doc = "Checks if the value of the field is `X4096`"]
    #[inline]
    pub fn is_x4096(&self) -> bool {
        *self == OVSRSELR::X4096
    }
}
#[doc = r" Value of the field"]
pub struct CHCONIDLER {
    bits: bool,
}
impl CHCONIDLER {
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
#[doc = "Values that can be written to the field `WARMUPMODE`"]
pub enum WARMUPMODEW {
    #[doc = "ADC is shut down after each conversion"]
    NORMAL,
    #[doc = "Bandgap references do not need warm up, but have reduced accuracy."]
    FASTBG,
    #[doc = "Reference selected for scan mode is kept warm."]
    KEEPSCANREFWARM,
    #[doc = "ADC is kept warmed up and scan reference is kept warm"]
    KEEPADCWARM,
}
impl WARMUPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WARMUPMODEW::NORMAL => 0,
            WARMUPMODEW::FASTBG => 1,
            WARMUPMODEW::KEEPSCANREFWARM => 2,
            WARMUPMODEW::KEEPADCWARM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WARMUPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WARMUPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WARMUPMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC is shut down after each conversion"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODEW::NORMAL)
    }
    #[doc = "Bandgap references do not need warm up, but have reduced accuracy."]
    #[inline]
    pub fn fastbg(self) -> &'a mut W {
        self.variant(WARMUPMODEW::FASTBG)
    }
    #[doc = "Reference selected for scan mode is kept warm."]
    #[inline]
    pub fn keepscanrefwarm(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPSCANREFWARM)
    }
    #[doc = "ADC is kept warmed up and scan reference is kept warm"]
    #[inline]
    pub fn keepadcwarm(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPADCWARM)
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
pub struct _TAILGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAILGATEW<'a> {
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
#[doc = "Values that can be written to the field `LPFMODE`"]
pub enum LPFMODEW {
    #[doc = "No filter or decoupling capacitor"]
    BYPASS,
    #[doc = "On chip decoupling capacitor selected"]
    DECAP,
    #[doc = "On chip RC filter selected"]
    RCFILT,
}
impl LPFMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPFMODEW::BYPASS => 0,
            LPFMODEW::DECAP => 1,
            LPFMODEW::RCFILT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPFMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPFMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPFMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No filter or decoupling capacitor"]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(LPFMODEW::BYPASS)
    }
    #[doc = "On chip decoupling capacitor selected"]
    #[inline]
    pub fn decap(self) -> &'a mut W {
        self.variant(LPFMODEW::DECAP)
    }
    #[doc = "On chip RC filter selected"]
    #[inline]
    pub fn rcfilt(self) -> &'a mut W {
        self.variant(LPFMODEW::RCFILT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
pub enum PRESCW {
    #[doc = "\"\""]
    NODIVISION,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::NODIVISION => 0,
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
    #[doc = "\"\""]
    #[inline]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESCW::NODIVISION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMEBASEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEBASEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVSRSEL`"]
pub enum OVSRSELW {
    #[doc = "2 samples for each conversion result"]
    X2,
    #[doc = "4 samples for each conversion result"]
    X4,
    #[doc = "8 samples for each conversion result"]
    X8,
    #[doc = "16 samples for each conversion result"]
    X16,
    #[doc = "32 samples for each conversion result"]
    X32,
    #[doc = "64 samples for each conversion result"]
    X64,
    #[doc = "128 samples for each conversion result"]
    X128,
    #[doc = "256 samples for each conversion result"]
    X256,
    #[doc = "512 samples for each conversion result"]
    X512,
    #[doc = "1024 samples for each conversion result"]
    X1024,
    #[doc = "2048 samples for each conversion result"]
    X2048,
    #[doc = "4096 samples for each conversion result"]
    X4096,
}
impl OVSRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OVSRSELW::X2 => 0,
            OVSRSELW::X4 => 1,
            OVSRSELW::X8 => 2,
            OVSRSELW::X16 => 3,
            OVSRSELW::X32 => 4,
            OVSRSELW::X64 => 5,
            OVSRSELW::X128 => 6,
            OVSRSELW::X256 => 7,
            OVSRSELW::X512 => 8,
            OVSRSELW::X1024 => 9,
            OVSRSELW::X2048 => 10,
            OVSRSELW::X4096 => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVSRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OVSRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVSRSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "2 samples for each conversion result"]
    #[inline]
    pub fn x2(self) -> &'a mut W {
        self.variant(OVSRSELW::X2)
    }
    #[doc = "4 samples for each conversion result"]
    #[inline]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVSRSELW::X4)
    }
    #[doc = "8 samples for each conversion result"]
    #[inline]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVSRSELW::X8)
    }
    #[doc = "16 samples for each conversion result"]
    #[inline]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVSRSELW::X16)
    }
    #[doc = "32 samples for each conversion result"]
    #[inline]
    pub fn x32(self) -> &'a mut W {
        self.variant(OVSRSELW::X32)
    }
    #[doc = "64 samples for each conversion result"]
    #[inline]
    pub fn x64(self) -> &'a mut W {
        self.variant(OVSRSELW::X64)
    }
    #[doc = "128 samples for each conversion result"]
    #[inline]
    pub fn x128(self) -> &'a mut W {
        self.variant(OVSRSELW::X128)
    }
    #[doc = "256 samples for each conversion result"]
    #[inline]
    pub fn x256(self) -> &'a mut W {
        self.variant(OVSRSELW::X256)
    }
    #[doc = "512 samples for each conversion result"]
    #[inline]
    pub fn x512(self) -> &'a mut W {
        self.variant(OVSRSELW::X512)
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline]
    pub fn x1024(self) -> &'a mut W {
        self.variant(OVSRSELW::X1024)
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline]
    pub fn x2048(self) -> &'a mut W {
        self.variant(OVSRSELW::X2048)
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline]
    pub fn x4096(self) -> &'a mut W {
        self.variant(OVSRSELW::X4096)
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
pub struct _CHCONIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHCONIDLEW<'a> {
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
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline]
    pub fn warmupmode(&self) -> WARMUPMODER {
        WARMUPMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Conversion Tailgating"]
    #[inline]
    pub fn tailgate(&self) -> TAILGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAILGATER { bits }
    }
    #[doc = "Bits 4:5 - Low Pass Filter Mode"]
    #[inline]
    pub fn lpfmode(&self) -> LPFMODER {
        LPFMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:14 - Prescaler Setting"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline]
    pub fn timebase(&self) -> TIMEBASER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMEBASER { bits }
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline]
    pub fn ovsrsel(&self) -> OVSRSELR {
        OVSRSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Input channel connected when ADC is IDLE"]
    #[inline]
    pub fn chconidle(&self) -> CHCONIDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHCONIDLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2031616 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline]
    pub fn warmupmode(&mut self) -> _WARMUPMODEW {
        _WARMUPMODEW { w: self }
    }
    #[doc = "Bit 3 - Conversion Tailgating"]
    #[inline]
    pub fn tailgate(&mut self) -> _TAILGATEW {
        _TAILGATEW { w: self }
    }
    #[doc = "Bits 4:5 - Low Pass Filter Mode"]
    #[inline]
    pub fn lpfmode(&mut self) -> _LPFMODEW {
        _LPFMODEW { w: self }
    }
    #[doc = "Bits 8:14 - Prescaler Setting"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline]
    pub fn timebase(&mut self) -> _TIMEBASEW {
        _TIMEBASEW { w: self }
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline]
    pub fn ovsrsel(&mut self) -> _OVSRSELW {
        _OVSRSELW { w: self }
    }
    #[doc = "Bit 28 - Input channel connected when ADC is IDLE"]
    #[inline]
    pub fn chconidle(&mut self) -> _CHCONIDLEW {
        _CHCONIDLEW { w: self }
    }
}
