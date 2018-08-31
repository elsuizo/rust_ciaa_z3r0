#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXDOUBLE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RXDATA0R {
    bits: u8,
}
impl RXDATA0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXDATA1R {
    bits: u8,
}
impl RXDATA1R {
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
    #[doc = "Bits 0:7 - RX Data 0"]
    #[inline]
    pub fn rxdata0(&self) -> RXDATA0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXDATA0R { bits }
    }
    #[doc = "Bits 8:15 - RX Data 1"]
    #[inline]
    pub fn rxdata1(&self) -> RXDATA1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXDATA1R { bits }
    }
}
