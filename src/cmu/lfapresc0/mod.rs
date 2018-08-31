#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFAPRESC0 {
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
#[doc = "Possible values of the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCR {
    #[doc = "LFACLKRTC = LFACLK"]
    DIV1,
    #[doc = "LFACLKRTC = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKRTC = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKRTC = LFACLK/8"]
    DIV8,
    #[doc = "LFACLKRTC = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKRTC = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKRTC = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKRTC = LFACLK/128"]
    DIV128,
    #[doc = "LFACLKRTC = LFACLK/256"]
    DIV256,
    #[doc = "LFACLKRTC = LFACLK/512"]
    DIV512,
    #[doc = "LFACLKRTC = LFACLK/1024"]
    DIV1024,
    #[doc = "LFACLKRTC = LFACLK/2048"]
    DIV2048,
    #[doc = "LFACLKRTC = LFACLK/4096"]
    DIV4096,
    #[doc = "LFACLKRTC = LFACLK/8192"]
    DIV8192,
    #[doc = "LFACLKRTC = LFACLK/16384"]
    DIV16384,
    #[doc = "LFACLKRTC = LFACLK/32768"]
    DIV32768,
}
impl RTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCR::DIV1 => 0,
            RTCR::DIV2 => 1,
            RTCR::DIV4 => 2,
            RTCR::DIV8 => 3,
            RTCR::DIV16 => 4,
            RTCR::DIV32 => 5,
            RTCR::DIV64 => 6,
            RTCR::DIV128 => 7,
            RTCR::DIV256 => 8,
            RTCR::DIV512 => 9,
            RTCR::DIV1024 => 10,
            RTCR::DIV2048 => 11,
            RTCR::DIV4096 => 12,
            RTCR::DIV8192 => 13,
            RTCR::DIV16384 => 14,
            RTCR::DIV32768 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCR {
        match value {
            0 => RTCR::DIV1,
            1 => RTCR::DIV2,
            2 => RTCR::DIV4,
            3 => RTCR::DIV8,
            4 => RTCR::DIV16,
            5 => RTCR::DIV32,
            6 => RTCR::DIV64,
            7 => RTCR::DIV128,
            8 => RTCR::DIV256,
            9 => RTCR::DIV512,
            10 => RTCR::DIV1024,
            11 => RTCR::DIV2048,
            12 => RTCR::DIV4096,
            13 => RTCR::DIV8192,
            14 => RTCR::DIV16384,
            15 => RTCR::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == RTCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == RTCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == RTCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == RTCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == RTCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == RTCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == RTCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == RTCR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == RTCR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == RTCR::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == RTCR::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline]
    pub fn is_div2048(&self) -> bool {
        *self == RTCR::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline]
    pub fn is_div4096(&self) -> bool {
        *self == RTCR::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline]
    pub fn is_div8192(&self) -> bool {
        *self == RTCR::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline]
    pub fn is_div16384(&self) -> bool {
        *self == RTCR::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline]
    pub fn is_div32768(&self) -> bool {
        *self == RTCR::DIV32768
    }
}
#[doc = "Values that can be written to the field `RTC`"]
pub enum RTCW {
    #[doc = "LFACLKRTC = LFACLK"]
    DIV1,
    #[doc = "LFACLKRTC = LFACLK/2"]
    DIV2,
    #[doc = "LFACLKRTC = LFACLK/4"]
    DIV4,
    #[doc = "LFACLKRTC = LFACLK/8"]
    DIV8,
    #[doc = "LFACLKRTC = LFACLK/16"]
    DIV16,
    #[doc = "LFACLKRTC = LFACLK/32"]
    DIV32,
    #[doc = "LFACLKRTC = LFACLK/64"]
    DIV64,
    #[doc = "LFACLKRTC = LFACLK/128"]
    DIV128,
    #[doc = "LFACLKRTC = LFACLK/256"]
    DIV256,
    #[doc = "LFACLKRTC = LFACLK/512"]
    DIV512,
    #[doc = "LFACLKRTC = LFACLK/1024"]
    DIV1024,
    #[doc = "LFACLKRTC = LFACLK/2048"]
    DIV2048,
    #[doc = "LFACLKRTC = LFACLK/4096"]
    DIV4096,
    #[doc = "LFACLKRTC = LFACLK/8192"]
    DIV8192,
    #[doc = "LFACLKRTC = LFACLK/16384"]
    DIV16384,
    #[doc = "LFACLKRTC = LFACLK/32768"]
    DIV32768,
}
impl RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCW::DIV1 => 0,
            RTCW::DIV2 => 1,
            RTCW::DIV4 => 2,
            RTCW::DIV8 => 3,
            RTCW::DIV16 => 4,
            RTCW::DIV32 => 5,
            RTCW::DIV64 => 6,
            RTCW::DIV128 => 7,
            RTCW::DIV256 => 8,
            RTCW::DIV512 => 9,
            RTCW::DIV1024 => 10,
            RTCW::DIV2048 => 11,
            RTCW::DIV4096 => 12,
            RTCW::DIV8192 => 13,
            RTCW::DIV16384 => 14,
            RTCW::DIV32768 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFACLKRTC = LFACLK"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(RTCW::DIV1)
    }
    #[doc = "LFACLKRTC = LFACLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTCW::DIV2)
    }
    #[doc = "LFACLKRTC = LFACLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTCW::DIV4)
    }
    #[doc = "LFACLKRTC = LFACLK/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(RTCW::DIV8)
    }
    #[doc = "LFACLKRTC = LFACLK/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(RTCW::DIV16)
    }
    #[doc = "LFACLKRTC = LFACLK/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(RTCW::DIV32)
    }
    #[doc = "LFACLKRTC = LFACLK/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(RTCW::DIV64)
    }
    #[doc = "LFACLKRTC = LFACLK/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(RTCW::DIV128)
    }
    #[doc = "LFACLKRTC = LFACLK/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(RTCW::DIV256)
    }
    #[doc = "LFACLKRTC = LFACLK/512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(RTCW::DIV512)
    }
    #[doc = "LFACLKRTC = LFACLK/1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(RTCW::DIV1024)
    }
    #[doc = "LFACLKRTC = LFACLK/2048"]
    #[inline]
    pub fn div2048(self) -> &'a mut W {
        self.variant(RTCW::DIV2048)
    }
    #[doc = "LFACLKRTC = LFACLK/4096"]
    #[inline]
    pub fn div4096(self) -> &'a mut W {
        self.variant(RTCW::DIV4096)
    }
    #[doc = "LFACLKRTC = LFACLK/8192"]
    #[inline]
    pub fn div8192(self) -> &'a mut W {
        self.variant(RTCW::DIV8192)
    }
    #[doc = "LFACLKRTC = LFACLK/16384"]
    #[inline]
    pub fn div16384(self) -> &'a mut W {
        self.variant(RTCW::DIV16384)
    }
    #[doc = "LFACLKRTC = LFACLK/32768"]
    #[inline]
    pub fn div32768(self) -> &'a mut W {
        self.variant(RTCW::DIV32768)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Real-Time Counter Prescaler"]
    #[inline]
    pub fn rtc(&self) -> RTCR {
        RTCR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Real-Time Counter Prescaler"]
    #[inline]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
}
