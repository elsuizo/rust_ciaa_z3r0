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
pub struct SINGLEACTR {
    bits: bool,
}
impl SINGLEACTR {
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
pub struct SCANACTR {
    bits: bool,
}
impl SCANACTR {
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
pub struct SINGLEREFWARMR {
    bits: bool,
}
impl SINGLEREFWARMR {
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
pub struct SCANREFWARMR {
    bits: bool,
}
impl SCANREFWARMR {
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
pub struct WARMR {
    bits: bool,
}
impl WARMR {
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
pub struct SINGLEDVR {
    bits: bool,
}
impl SINGLEDVR {
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
pub struct SCANDVR {
    bits: bool,
}
impl SCANDVR {
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
#[doc = "Possible values of the field `SCANDATASRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCANDATASRCR {
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH0. Differential mode: SCANDATA result originates from ADCn_CH0-ADCn_CH1"]
    CH0,
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH1. Differential mode: SCANDATA result originates from ADCn_CH2_ADCn_CH3"]
    CH1,
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH2. Differential mode: SCANDATA result originates from ADCn_CH4-ADCn_CH5"]
    CH2,
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH3. Differential mode: SCANDATA result originates from ADCn_CH6-ADCn_CH7"]
    CH3,
    #[doc = "SCANDATA result originates from ADCn_CH4"]
    CH4,
    #[doc = "SCANDATA result originates from ADCn_CH5"]
    CH5,
    #[doc = "SCANDATA result originates from ADCn_CH6"]
    CH6,
    #[doc = "SCANDATA result originates from ADCn_CH7"]
    CH7,
}
impl SCANDATASRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCANDATASRCR::CH0 => 0,
            SCANDATASRCR::CH1 => 1,
            SCANDATASRCR::CH2 => 2,
            SCANDATASRCR::CH3 => 3,
            SCANDATASRCR::CH4 => 4,
            SCANDATASRCR::CH5 => 5,
            SCANDATASRCR::CH6 => 6,
            SCANDATASRCR::CH7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCANDATASRCR {
        match value {
            0 => SCANDATASRCR::CH0,
            1 => SCANDATASRCR::CH1,
            2 => SCANDATASRCR::CH2,
            3 => SCANDATASRCR::CH3,
            4 => SCANDATASRCR::CH4,
            5 => SCANDATASRCR::CH5,
            6 => SCANDATASRCR::CH6,
            7 => SCANDATASRCR::CH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline]
    pub fn is_ch0(&self) -> bool {
        *self == SCANDATASRCR::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline]
    pub fn is_ch1(&self) -> bool {
        *self == SCANDATASRCR::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline]
    pub fn is_ch2(&self) -> bool {
        *self == SCANDATASRCR::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline]
    pub fn is_ch3(&self) -> bool {
        *self == SCANDATASRCR::CH3
    }
    #[doc = "Checks if the value of the field is `CH4`"]
    #[inline]
    pub fn is_ch4(&self) -> bool {
        *self == SCANDATASRCR::CH4
    }
    #[doc = "Checks if the value of the field is `CH5`"]
    #[inline]
    pub fn is_ch5(&self) -> bool {
        *self == SCANDATASRCR::CH5
    }
    #[doc = "Checks if the value of the field is `CH6`"]
    #[inline]
    pub fn is_ch6(&self) -> bool {
        *self == SCANDATASRCR::CH6
    }
    #[doc = "Checks if the value of the field is `CH7`"]
    #[inline]
    pub fn is_ch7(&self) -> bool {
        *self == SCANDATASRCR::CH7
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Single Conversion Active"]
    #[inline]
    pub fn singleact(&self) -> SINGLEACTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLEACTR { bits }
    }
    #[doc = "Bit 1 - Scan Conversion Active"]
    #[inline]
    pub fn scanact(&self) -> SCANACTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCANACTR { bits }
    }
    #[doc = "Bit 8 - Single Reference Warmed Up"]
    #[inline]
    pub fn singlerefwarm(&self) -> SINGLEREFWARMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLEREFWARMR { bits }
    }
    #[doc = "Bit 9 - Scan Reference Warmed Up"]
    #[inline]
    pub fn scanrefwarm(&self) -> SCANREFWARMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCANREFWARMR { bits }
    }
    #[doc = "Bit 12 - ADC Warmed Up"]
    #[inline]
    pub fn warm(&self) -> WARMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WARMR { bits }
    }
    #[doc = "Bit 16 - Single Sample Data Valid"]
    #[inline]
    pub fn singledv(&self) -> SINGLEDVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SINGLEDVR { bits }
    }
    #[doc = "Bit 17 - Scan Data Valid"]
    #[inline]
    pub fn scandv(&self) -> SCANDVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCANDVR { bits }
    }
    #[doc = "Bits 24:26 - Scan Data Source"]
    #[inline]
    pub fn scandatasrc(&self) -> SCANDATASRCR {
        SCANDATASRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
