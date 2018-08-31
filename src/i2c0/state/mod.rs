#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct BUSYR {
    bits: bool,
}
impl BUSYR {
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
pub struct MASTERR {
    bits: bool,
}
impl MASTERR {
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
pub struct TRANSMITTERR {
    bits: bool,
}
impl TRANSMITTERR {
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
pub struct NACKEDR {
    bits: bool,
}
impl NACKEDR {
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
pub struct BUSHOLDR {
    bits: bool,
}
impl BUSHOLDR {
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
    #[doc = "No transmission is being performed."]
    IDLE,
    #[doc = "Waiting for idle. Will send a start condition as soon as the bus is idle."]
    WAIT,
    #[doc = "Start transmitted or received"]
    START,
    #[doc = "Address transmitted or received"]
    ADDR,
    #[doc = "Address ack/nack transmitted or received"]
    ADDRACK,
    #[doc = "Data transmitted or received"]
    DATA,
    #[doc = "Data ack/nack transmitted or received"]
    DATAACK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::IDLE => 0,
            STATER::WAIT => 1,
            STATER::START => 2,
            STATER::ADDR => 3,
            STATER::ADDRACK => 4,
            STATER::DATA => 5,
            STATER::DATAACK => 6,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            0 => STATER::IDLE,
            1 => STATER::WAIT,
            2 => STATER::START,
            3 => STATER::ADDR,
            4 => STATER::ADDRACK,
            5 => STATER::DATA,
            6 => STATER::DATAACK,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == STATER::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == STATER::WAIT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == STATER::START
    }
    #[doc = "Checks if the value of the field is `ADDR`"]
    #[inline]
    pub fn is_addr(&self) -> bool {
        *self == STATER::ADDR
    }
    #[doc = "Checks if the value of the field is `ADDRACK`"]
    #[inline]
    pub fn is_addrack(&self) -> bool {
        *self == STATER::ADDRACK
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline]
    pub fn is_data(&self) -> bool {
        *self == STATER::DATA
    }
    #[doc = "Checks if the value of the field is `DATAACK`"]
    #[inline]
    pub fn is_dataack(&self) -> bool {
        *self == STATER::DATAACK
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Bus Busy"]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSYR { bits }
    }
    #[doc = "Bit 1 - Master"]
    #[inline]
    pub fn master(&self) -> MASTERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MASTERR { bits }
    }
    #[doc = "Bit 2 - Transmitter"]
    #[inline]
    pub fn transmitter(&self) -> TRANSMITTERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRANSMITTERR { bits }
    }
    #[doc = "Bit 3 - Nack Received"]
    #[inline]
    pub fn nacked(&self) -> NACKEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NACKEDR { bits }
    }
    #[doc = "Bit 4 - Bus Held"]
    #[inline]
    pub fn bushold(&self) -> BUSHOLDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSHOLDR { bits }
    }
    #[doc = "Bits 5:7 - Transmission State"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
