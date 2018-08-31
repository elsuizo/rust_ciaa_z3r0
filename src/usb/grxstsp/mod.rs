#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GRXSTSP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CHEPNUMR {
    bits: u8,
}
impl CHEPNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BCNTR {
    bits: u16,
}
impl BCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DPID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPIDR {
    #[doc = "DATA0 PID."]
    DATA0,
    #[doc = "DATA1 PID."]
    DATA1,
    #[doc = "DATA2 PID."]
    DATA2,
    #[doc = "MDATA PID."]
    MDATA,
}
impl DPIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DPIDR::DATA0 => 0,
            DPIDR::DATA1 => 1,
            DPIDR::DATA2 => 2,
            DPIDR::MDATA => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DPIDR {
        match value {
            0 => DPIDR::DATA0,
            1 => DPIDR::DATA1,
            2 => DPIDR::DATA2,
            3 => DPIDR::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == DPIDR::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == DPIDR::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline]
    pub fn is_data2(&self) -> bool {
        *self == DPIDR::DATA2
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline]
    pub fn is_mdata(&self) -> bool {
        *self == DPIDR::MDATA
    }
}
#[doc = "Possible values of the field `PKTSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKTSTSR {
    #[doc = "Device mode: Global OUT NAK (triggers an interrupt)."]
    GOUTNAK,
    #[doc = "Host mode: IN data packet received. Device mode: OUT data packet received."]
    PKTRCV,
    #[doc = "Host mode: IN transfer completed (triggers an interrupt). Device mode: OUT transfer completed (triggers an interrupt)."]
    XFERCOMPL,
    #[doc = "Device mode: SETUP transaction completed (triggers an interrupt)."]
    SETUPCOMPL,
    #[doc = "Host mode: Data toggle error (triggers an interrupt)."]
    TGLERR,
    #[doc = "Device mode: SETUP data packet received."]
    SETUPRCV,
    #[doc = "Host mode: Channel halted (triggers an interrupt)."]
    CHLT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PKTSTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PKTSTSR::GOUTNAK => 1,
            PKTSTSR::PKTRCV => 2,
            PKTSTSR::XFERCOMPL => 3,
            PKTSTSR::SETUPCOMPL => 4,
            PKTSTSR::TGLERR => 5,
            PKTSTSR::SETUPRCV => 6,
            PKTSTSR::CHLT => 7,
            PKTSTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PKTSTSR {
        match value {
            1 => PKTSTSR::GOUTNAK,
            2 => PKTSTSR::PKTRCV,
            3 => PKTSTSR::XFERCOMPL,
            4 => PKTSTSR::SETUPCOMPL,
            5 => PKTSTSR::TGLERR,
            6 => PKTSTSR::SETUPRCV,
            7 => PKTSTSR::CHLT,
            i => PKTSTSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GOUTNAK`"]
    #[inline]
    pub fn is_goutnak(&self) -> bool {
        *self == PKTSTSR::GOUTNAK
    }
    #[doc = "Checks if the value of the field is `PKTRCV`"]
    #[inline]
    pub fn is_pktrcv(&self) -> bool {
        *self == PKTSTSR::PKTRCV
    }
    #[doc = "Checks if the value of the field is `XFERCOMPL`"]
    #[inline]
    pub fn is_xfercompl(&self) -> bool {
        *self == PKTSTSR::XFERCOMPL
    }
    #[doc = "Checks if the value of the field is `SETUPCOMPL`"]
    #[inline]
    pub fn is_setupcompl(&self) -> bool {
        *self == PKTSTSR::SETUPCOMPL
    }
    #[doc = "Checks if the value of the field is `TGLERR`"]
    #[inline]
    pub fn is_tglerr(&self) -> bool {
        *self == PKTSTSR::TGLERR
    }
    #[doc = "Checks if the value of the field is `SETUPRCV`"]
    #[inline]
    pub fn is_setuprcv(&self) -> bool {
        *self == PKTSTSR::SETUPRCV
    }
    #[doc = "Checks if the value of the field is `CHLT`"]
    #[inline]
    pub fn is_chlt(&self) -> bool {
        *self == PKTSTSR::CHLT
    }
}
#[doc = r" Value of the field"]
pub struct FNR {
    bits: u8,
}
impl FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Channel Number host only / Endpoint Number"]
    #[inline]
    pub fn chepnum(&self) -> CHEPNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHEPNUMR { bits }
    }
    #[doc = "Bits 4:14 - Byte Count (host or device)"]
    #[inline]
    pub fn bcnt(&self) -> BCNTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BCNTR { bits }
    }
    #[doc = "Bits 15:16 - Data PID (host or device)"]
    #[inline]
    pub fn dpid(&self) -> DPIDR {
        DPIDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:20 - Packet Status (host or device)"]
    #[inline]
    pub fn pktsts(&self) -> PKTSTSR {
        PKTSTSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline]
    pub fn fn_(&self) -> FNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FNR { bits }
    }
}
