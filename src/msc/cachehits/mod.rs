#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CACHEHITS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CACHEHITSR {
    bits: u32,
}
impl CACHEHITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:19 - Cache hits since last performance counter start command."]
    #[inline]
    pub fn cachehits(&self) -> CACHEHITSR {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CACHEHITSR { bits }
    }
}
