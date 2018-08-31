#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ENR {
    bits: bool,
}
impl ENR {
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
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "Idle"]
    IDLE,
    #[doc = "Reading channel controller data"]
    RDCHCTRLDATA,
    #[doc = "Reading source data end pointer"]
    RDSRCENDPTR,
    #[doc = "Reading destination data end pointer"]
    RDDSTENDPTR,
    #[doc = "Reading source data"]
    RDSRCDATA,
    #[doc = "Writing destination data"]
    WRDSTDATA,
    #[doc = "Waiting for DMA request to clear"]
    WAITREQCLR,
    #[doc = "Writing channel controller data"]
    WRCHCTRLDATA,
    #[doc = "Stalled"]
    STALLED,
    #[doc = "Done"]
    DONE,
    #[doc = "Peripheral scatter-gather transition"]
    PERSCATTRANS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::IDLE => 0,
            STATER::RDCHCTRLDATA => 1,
            STATER::RDSRCENDPTR => 2,
            STATER::RDDSTENDPTR => 3,
            STATER::RDSRCDATA => 4,
            STATER::WRDSTDATA => 5,
            STATER::WAITREQCLR => 6,
            STATER::WRCHCTRLDATA => 7,
            STATER::STALLED => 8,
            STATER::DONE => 9,
            STATER::PERSCATTRANS => 10,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            0 => STATER::IDLE,
            1 => STATER::RDCHCTRLDATA,
            2 => STATER::RDSRCENDPTR,
            3 => STATER::RDDSTENDPTR,
            4 => STATER::RDSRCDATA,
            5 => STATER::WRDSTDATA,
            6 => STATER::WAITREQCLR,
            7 => STATER::WRCHCTRLDATA,
            8 => STATER::STALLED,
            9 => STATER::DONE,
            10 => STATER::PERSCATTRANS,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == STATER::IDLE
    }
    #[doc = "Checks if the value of the field is `RDCHCTRLDATA`"]
    #[inline]
    pub fn is_rdchctrldata(&self) -> bool {
        *self == STATER::RDCHCTRLDATA
    }
    #[doc = "Checks if the value of the field is `RDSRCENDPTR`"]
    #[inline]
    pub fn is_rdsrcendptr(&self) -> bool {
        *self == STATER::RDSRCENDPTR
    }
    #[doc = "Checks if the value of the field is `RDDSTENDPTR`"]
    #[inline]
    pub fn is_rddstendptr(&self) -> bool {
        *self == STATER::RDDSTENDPTR
    }
    #[doc = "Checks if the value of the field is `RDSRCDATA`"]
    #[inline]
    pub fn is_rdsrcdata(&self) -> bool {
        *self == STATER::RDSRCDATA
    }
    #[doc = "Checks if the value of the field is `WRDSTDATA`"]
    #[inline]
    pub fn is_wrdstdata(&self) -> bool {
        *self == STATER::WRDSTDATA
    }
    #[doc = "Checks if the value of the field is `WAITREQCLR`"]
    #[inline]
    pub fn is_waitreqclr(&self) -> bool {
        *self == STATER::WAITREQCLR
    }
    #[doc = "Checks if the value of the field is `WRCHCTRLDATA`"]
    #[inline]
    pub fn is_wrchctrldata(&self) -> bool {
        *self == STATER::WRCHCTRLDATA
    }
    #[doc = "Checks if the value of the field is `STALLED`"]
    #[inline]
    pub fn is_stalled(&self) -> bool {
        *self == STATER::STALLED
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline]
    pub fn is_done(&self) -> bool {
        *self == STATER::DONE
    }
    #[doc = "Checks if the value of the field is `PERSCATTRANS`"]
    #[inline]
    pub fn is_perscattrans(&self) -> bool {
        *self == STATER::PERSCATTRANS
    }
}
#[doc = r" Value of the field"]
pub struct CHNUMR {
    bits: u8,
}
impl CHNUMR {
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
    #[doc = "Bit 0 - DMA Enable Status"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bits 4:7 - Control Current State"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:20 - Channel Number"]
    #[inline]
    pub fn chnum(&self) -> CHNUMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHNUMR { bits }
    }
}
