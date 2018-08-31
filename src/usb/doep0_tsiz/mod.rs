#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DOEP0_TSIZ {
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
#[doc = r" Value of the field"]
pub struct XFERSIZER {
    bits: u32,
}
impl XFERSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKTCNTR {
    bits: u16,
}
impl PKTCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXDPIDSUPCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDPIDSUPCNTR {
    #[doc = "DATA0 PID."]
    DATA0,
    #[doc = "DATA2 PID / 1 Packet."]
    DATA2,
    #[doc = "DATA1 PID / 2 Packets."]
    DATA1,
    #[doc = "MDATA PID / 3 Packets."]
    MDATA,
}
impl RXDPIDSUPCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXDPIDSUPCNTR::DATA0 => 0,
            RXDPIDSUPCNTR::DATA2 => 1,
            RXDPIDSUPCNTR::DATA1 => 2,
            RXDPIDSUPCNTR::MDATA => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXDPIDSUPCNTR {
        match value {
            0 => RXDPIDSUPCNTR::DATA0,
            1 => RXDPIDSUPCNTR::DATA2,
            2 => RXDPIDSUPCNTR::DATA1,
            3 => RXDPIDSUPCNTR::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == RXDPIDSUPCNTR::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline]
    pub fn is_data2(&self) -> bool {
        *self == RXDPIDSUPCNTR::DATA2
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == RXDPIDSUPCNTR::DATA1
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline]
    pub fn is_mdata(&self) -> bool {
        *self == RXDPIDSUPCNTR::MDATA
    }
}
#[doc = r" Proxy"]
pub struct _XFERSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _XFERSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PKTCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PKTCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 19;
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
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline]
    pub fn xfersize(&self) -> XFERSIZER {
        let bits = {
            const MASK: u32 = 524287;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        XFERSIZER { bits }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline]
    pub fn pktcnt(&self) -> PKTCNTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKTCNTR { bits }
    }
    #[doc = "Bits 29:30 - Receive Data PID / SETUP Packet Count"]
    #[inline]
    pub fn rxdpidsupcnt(&self) -> RXDPIDSUPCNTR {
        RXDPIDSUPCNTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline]
    pub fn xfersize(&mut self) -> _XFERSIZEW {
        _XFERSIZEW { w: self }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline]
    pub fn pktcnt(&mut self) -> _PKTCNTW {
        _PKTCNTW { w: self }
    }
}
