#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTIPSELH {
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
#[doc = "Possible values of the field `EXTIPSEL8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL8R {
    #[doc = "Port A pin 8 selected for external interrupt 8"]
    PORTA,
    #[doc = "Port B pin 8 selected for external interrupt 8"]
    PORTB,
    #[doc = "Port C pin 8 selected for external interrupt 8"]
    PORTC,
    #[doc = "Port D pin 8 selected for external interrupt 8"]
    PORTD,
    #[doc = "Port E pin 8 selected for external interrupt 8"]
    PORTE,
    #[doc = "Port F pin 8 selected for external interrupt 8"]
    PORTF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL8R::PORTA => 0,
            EXTIPSEL8R::PORTB => 1,
            EXTIPSEL8R::PORTC => 2,
            EXTIPSEL8R::PORTD => 3,
            EXTIPSEL8R::PORTE => 4,
            EXTIPSEL8R::PORTF => 5,
            EXTIPSEL8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL8R {
        match value {
            0 => EXTIPSEL8R::PORTA,
            1 => EXTIPSEL8R::PORTB,
            2 => EXTIPSEL8R::PORTC,
            3 => EXTIPSEL8R::PORTD,
            4 => EXTIPSEL8R::PORTE,
            5 => EXTIPSEL8R::PORTF,
            i => EXTIPSEL8R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL8R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL8R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL8R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL8R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL8R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL8R::PORTF
    }
}
#[doc = "Possible values of the field `EXTIPSEL9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL9R {
    #[doc = "Port A pin 9 selected for external interrupt 9"]
    PORTA,
    #[doc = "Port B pin 9 selected for external interrupt 9"]
    PORTB,
    #[doc = "Port C pin 9 selected for external interrupt 9"]
    PORTC,
    #[doc = "Port D pin 9 selected for external interrupt 9"]
    PORTD,
    #[doc = "Port E pin 9 selected for external interrupt 9"]
    PORTE,
    #[doc = "Port F pin 9 selected for external interrupt 9"]
    PORTF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL9R::PORTA => 0,
            EXTIPSEL9R::PORTB => 1,
            EXTIPSEL9R::PORTC => 2,
            EXTIPSEL9R::PORTD => 3,
            EXTIPSEL9R::PORTE => 4,
            EXTIPSEL9R::PORTF => 5,
            EXTIPSEL9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL9R {
        match value {
            0 => EXTIPSEL9R::PORTA,
            1 => EXTIPSEL9R::PORTB,
            2 => EXTIPSEL9R::PORTC,
            3 => EXTIPSEL9R::PORTD,
            4 => EXTIPSEL9R::PORTE,
            5 => EXTIPSEL9R::PORTF,
            i => EXTIPSEL9R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL9R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL9R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL9R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL9R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL9R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL9R::PORTF
    }
}
#[doc = "Possible values of the field `EXTIPSEL10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL10R {
    #[doc = "Port A pin 10 selected for external interrupt 10"]
    PORTA,
    #[doc = "Port B pin 10 selected for external interrupt 10"]
    PORTB,
    #[doc = "Port C pin 10 selected for external interrupt 10"]
    PORTC,
    #[doc = "Port D pin 10 selected for external interrupt 10"]
    PORTD,
    #[doc = "Port E pin 10 selected for external interrupt 10"]
    PORTE,
    #[doc = "Port F pin 10 selected for external interrupt 10"]
    PORTF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL10R::PORTA => 0,
            EXTIPSEL10R::PORTB => 1,
            EXTIPSEL10R::PORTC => 2,
            EXTIPSEL10R::PORTD => 3,
            EXTIPSEL10R::PORTE => 4,
            EXTIPSEL10R::PORTF => 5,
            EXTIPSEL10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL10R {
        match value {
            0 => EXTIPSEL10R::PORTA,
            1 => EXTIPSEL10R::PORTB,
            2 => EXTIPSEL10R::PORTC,
            3 => EXTIPSEL10R::PORTD,
            4 => EXTIPSEL10R::PORTE,
            5 => EXTIPSEL10R::PORTF,
            i => EXTIPSEL10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL10R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL10R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL10R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL10R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL10R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL10R::PORTF
    }
}
#[doc = "Possible values of the field `EXTIPSEL11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL11R {
    #[doc = "Port A pin 11 selected for external interrupt 11"]
    PORTA,
    #[doc = "Port B pin 11 selected for external interrupt 11"]
    PORTB,
    #[doc = "Port C pin 11 selected for external interrupt 11"]
    PORTC,
    #[doc = "Port D pin 11 selected for external interrupt 11"]
    PORTD,
    #[doc = "Port E pin 11 selected for external interrupt 11"]
    PORTE,
    #[doc = "Port F pin 11 selected for external interrupt 11"]
    PORTF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL11R::PORTA => 0,
            EXTIPSEL11R::PORTB => 1,
            EXTIPSEL11R::PORTC => 2,
            EXTIPSEL11R::PORTD => 3,
            EXTIPSEL11R::PORTE => 4,
            EXTIPSEL11R::PORTF => 5,
            EXTIPSEL11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL11R {
        match value {
            0 => EXTIPSEL11R::PORTA,
            1 => EXTIPSEL11R::PORTB,
            2 => EXTIPSEL11R::PORTC,
            3 => EXTIPSEL11R::PORTD,
            4 => EXTIPSEL11R::PORTE,
            5 => EXTIPSEL11R::PORTF,
            i => EXTIPSEL11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL11R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL11R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL11R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL11R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL11R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL11R::PORTF
    }
}
#[doc = "Possible values of the field `EXTIPSEL12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL12R {
    #[doc = "Port A pin 12 selected for external interrupt 12"]
    PORTA,
    #[doc = "Port B pin 12 selected for external interrupt 12"]
    PORTB,
    #[doc = "Port C pin 12 selected for external interrupt 12"]
    PORTC,
    #[doc = "Port D pin 12 selected for external interrupt 12"]
    PORTD,
    #[doc = "Port E pin 12 selected for external interrupt 12"]
    PORTE,
    #[doc = "Port F pin 12 selected for external interrupt 12"]
    PORTF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL12R::PORTA => 0,
            EXTIPSEL12R::PORTB => 1,
            EXTIPSEL12R::PORTC => 2,
            EXTIPSEL12R::PORTD => 3,
            EXTIPSEL12R::PORTE => 4,
            EXTIPSEL12R::PORTF => 5,
            EXTIPSEL12R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL12R {
        match value {
            0 => EXTIPSEL12R::PORTA,
            1 => EXTIPSEL12R::PORTB,
            2 => EXTIPSEL12R::PORTC,
            3 => EXTIPSEL12R::PORTD,
            4 => EXTIPSEL12R::PORTE,
            5 => EXTIPSEL12R::PORTF,
            i => EXTIPSEL12R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL12R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL12R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL12R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL12R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL12R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL12R::PORTF
    }
}
#[doc = "Possible values of the field `EXTIPSEL13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL13R {
    #[doc = "Port A pin 13 selected for external interrupt 13"]
    PORTA,
    #[doc = "Port B pin 13 selected for external interrupt 13"]
    PORTB,
    #[doc = "Port C pin 13 selected for external interrupt 13"]
    PORTC,
    #[doc = "Port D pin 13 selected for external interrupt 13"]
    PORTD,
    #[doc = "Port E pin 13 selected for external interrupt 13"]
    PORTE,
    #[doc = "Port F pin 13 selected for external interrupt 13"]
    PORTF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL13R::PORTA => 0,
            EXTIPSEL13R::PORTB => 1,
            EXTIPSEL13R::PORTC => 2,
            EXTIPSEL13R::PORTD => 3,
            EXTIPSEL13R::PORTE => 4,
            EXTIPSEL13R::PORTF => 5,
            EXTIPSEL13R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL13R {
        match value {
            0 => EXTIPSEL13R::PORTA,
            1 => EXTIPSEL13R::PORTB,
            2 => EXTIPSEL13R::PORTC,
            3 => EXTIPSEL13R::PORTD,
            4 => EXTIPSEL13R::PORTE,
            5 => EXTIPSEL13R::PORTF,
            i => EXTIPSEL13R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL13R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL13R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL13R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL13R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL13R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL13R::PORTF
    }
}
#[doc = "Possible values of the field `EXTIPSEL14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL14R {
    #[doc = "Port A pin 14 selected for external interrupt 14"]
    PORTA,
    #[doc = "Port B pin 14 selected for external interrupt 14"]
    PORTB,
    #[doc = "Port C pin 14 selected for external interrupt 14"]
    PORTC,
    #[doc = "Port D pin 14 selected for external interrupt 14"]
    PORTD,
    #[doc = "Port E pin 14 selected for external interrupt 14"]
    PORTE,
    #[doc = "Port F pin 14 selected for external interrupt 14"]
    PORTF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL14R::PORTA => 0,
            EXTIPSEL14R::PORTB => 1,
            EXTIPSEL14R::PORTC => 2,
            EXTIPSEL14R::PORTD => 3,
            EXTIPSEL14R::PORTE => 4,
            EXTIPSEL14R::PORTF => 5,
            EXTIPSEL14R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL14R {
        match value {
            0 => EXTIPSEL14R::PORTA,
            1 => EXTIPSEL14R::PORTB,
            2 => EXTIPSEL14R::PORTC,
            3 => EXTIPSEL14R::PORTD,
            4 => EXTIPSEL14R::PORTE,
            5 => EXTIPSEL14R::PORTF,
            i => EXTIPSEL14R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL14R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL14R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL14R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL14R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL14R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL14R::PORTF
    }
}
#[doc = "Possible values of the field `EXTIPSEL15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL15R {
    #[doc = "Port A pin 15 selected for external interrupt 15"]
    PORTA,
    #[doc = "Port B pin 15 selected for external interrupt 15"]
    PORTB,
    #[doc = "Port C pin 15 selected for external interrupt 15"]
    PORTC,
    #[doc = "Port D pin 15 selected for external interrupt 15"]
    PORTD,
    #[doc = "Port E pin 15 selected for external interrupt 15"]
    PORTE,
    #[doc = "Port F pin 15 selected for external interrupt 15"]
    PORTF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL15R::PORTA => 0,
            EXTIPSEL15R::PORTB => 1,
            EXTIPSEL15R::PORTC => 2,
            EXTIPSEL15R::PORTD => 3,
            EXTIPSEL15R::PORTE => 4,
            EXTIPSEL15R::PORTF => 5,
            EXTIPSEL15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL15R {
        match value {
            0 => EXTIPSEL15R::PORTA,
            1 => EXTIPSEL15R::PORTB,
            2 => EXTIPSEL15R::PORTC,
            3 => EXTIPSEL15R::PORTD,
            4 => EXTIPSEL15R::PORTE,
            5 => EXTIPSEL15R::PORTF,
            i => EXTIPSEL15R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL15R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL15R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL15R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL15R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL15R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL15R::PORTF
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL8`"]
pub enum EXTIPSEL8W {
    #[doc = "Port A pin 8 selected for external interrupt 8"]
    PORTA,
    #[doc = "Port B pin 8 selected for external interrupt 8"]
    PORTB,
    #[doc = "Port C pin 8 selected for external interrupt 8"]
    PORTC,
    #[doc = "Port D pin 8 selected for external interrupt 8"]
    PORTD,
    #[doc = "Port E pin 8 selected for external interrupt 8"]
    PORTE,
    #[doc = "Port F pin 8 selected for external interrupt 8"]
    PORTF,
}
impl EXTIPSEL8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL8W::PORTA => 0,
            EXTIPSEL8W::PORTB => 1,
            EXTIPSEL8W::PORTC => 2,
            EXTIPSEL8W::PORTD => 3,
            EXTIPSEL8W::PORTE => 4,
            EXTIPSEL8W::PORTF => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL8W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A pin 8 selected for external interrupt 8"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL8W::PORTA)
    }
    #[doc = "Port B pin 8 selected for external interrupt 8"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL8W::PORTB)
    }
    #[doc = "Port C pin 8 selected for external interrupt 8"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL8W::PORTC)
    }
    #[doc = "Port D pin 8 selected for external interrupt 8"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL8W::PORTD)
    }
    #[doc = "Port E pin 8 selected for external interrupt 8"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL8W::PORTE)
    }
    #[doc = "Port F pin 8 selected for external interrupt 8"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL8W::PORTF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL9`"]
pub enum EXTIPSEL9W {
    #[doc = "Port A pin 9 selected for external interrupt 9"]
    PORTA,
    #[doc = "Port B pin 9 selected for external interrupt 9"]
    PORTB,
    #[doc = "Port C pin 9 selected for external interrupt 9"]
    PORTC,
    #[doc = "Port D pin 9 selected for external interrupt 9"]
    PORTD,
    #[doc = "Port E pin 9 selected for external interrupt 9"]
    PORTE,
    #[doc = "Port F pin 9 selected for external interrupt 9"]
    PORTF,
}
impl EXTIPSEL9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL9W::PORTA => 0,
            EXTIPSEL9W::PORTB => 1,
            EXTIPSEL9W::PORTC => 2,
            EXTIPSEL9W::PORTD => 3,
            EXTIPSEL9W::PORTE => 4,
            EXTIPSEL9W::PORTF => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL9W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A pin 9 selected for external interrupt 9"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL9W::PORTA)
    }
    #[doc = "Port B pin 9 selected for external interrupt 9"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL9W::PORTB)
    }
    #[doc = "Port C pin 9 selected for external interrupt 9"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL9W::PORTC)
    }
    #[doc = "Port D pin 9 selected for external interrupt 9"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL9W::PORTD)
    }
    #[doc = "Port E pin 9 selected for external interrupt 9"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL9W::PORTE)
    }
    #[doc = "Port F pin 9 selected for external interrupt 9"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL9W::PORTF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL10`"]
pub enum EXTIPSEL10W {
    #[doc = "Port A pin 10 selected for external interrupt 10"]
    PORTA,
    #[doc = "Port B pin 10 selected for external interrupt 10"]
    PORTB,
    #[doc = "Port C pin 10 selected for external interrupt 10"]
    PORTC,
    #[doc = "Port D pin 10 selected for external interrupt 10"]
    PORTD,
    #[doc = "Port E pin 10 selected for external interrupt 10"]
    PORTE,
    #[doc = "Port F pin 10 selected for external interrupt 10"]
    PORTF,
}
impl EXTIPSEL10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL10W::PORTA => 0,
            EXTIPSEL10W::PORTB => 1,
            EXTIPSEL10W::PORTC => 2,
            EXTIPSEL10W::PORTD => 3,
            EXTIPSEL10W::PORTE => 4,
            EXTIPSEL10W::PORTF => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL10W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A pin 10 selected for external interrupt 10"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL10W::PORTA)
    }
    #[doc = "Port B pin 10 selected for external interrupt 10"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL10W::PORTB)
    }
    #[doc = "Port C pin 10 selected for external interrupt 10"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL10W::PORTC)
    }
    #[doc = "Port D pin 10 selected for external interrupt 10"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL10W::PORTD)
    }
    #[doc = "Port E pin 10 selected for external interrupt 10"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL10W::PORTE)
    }
    #[doc = "Port F pin 10 selected for external interrupt 10"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL10W::PORTF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL11`"]
pub enum EXTIPSEL11W {
    #[doc = "Port A pin 11 selected for external interrupt 11"]
    PORTA,
    #[doc = "Port B pin 11 selected for external interrupt 11"]
    PORTB,
    #[doc = "Port C pin 11 selected for external interrupt 11"]
    PORTC,
    #[doc = "Port D pin 11 selected for external interrupt 11"]
    PORTD,
    #[doc = "Port E pin 11 selected for external interrupt 11"]
    PORTE,
    #[doc = "Port F pin 11 selected for external interrupt 11"]
    PORTF,
}
impl EXTIPSEL11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL11W::PORTA => 0,
            EXTIPSEL11W::PORTB => 1,
            EXTIPSEL11W::PORTC => 2,
            EXTIPSEL11W::PORTD => 3,
            EXTIPSEL11W::PORTE => 4,
            EXTIPSEL11W::PORTF => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL11W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A pin 11 selected for external interrupt 11"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL11W::PORTA)
    }
    #[doc = "Port B pin 11 selected for external interrupt 11"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL11W::PORTB)
    }
    #[doc = "Port C pin 11 selected for external interrupt 11"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL11W::PORTC)
    }
    #[doc = "Port D pin 11 selected for external interrupt 11"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL11W::PORTD)
    }
    #[doc = "Port E pin 11 selected for external interrupt 11"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL11W::PORTE)
    }
    #[doc = "Port F pin 11 selected for external interrupt 11"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL11W::PORTF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL12`"]
pub enum EXTIPSEL12W {
    #[doc = "Port A pin 12 selected for external interrupt 12"]
    PORTA,
    #[doc = "Port B pin 12 selected for external interrupt 12"]
    PORTB,
    #[doc = "Port C pin 12 selected for external interrupt 12"]
    PORTC,
    #[doc = "Port D pin 12 selected for external interrupt 12"]
    PORTD,
    #[doc = "Port E pin 12 selected for external interrupt 12"]
    PORTE,
    #[doc = "Port F pin 12 selected for external interrupt 12"]
    PORTF,
}
impl EXTIPSEL12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL12W::PORTA => 0,
            EXTIPSEL12W::PORTB => 1,
            EXTIPSEL12W::PORTC => 2,
            EXTIPSEL12W::PORTD => 3,
            EXTIPSEL12W::PORTE => 4,
            EXTIPSEL12W::PORTF => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL12W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A pin 12 selected for external interrupt 12"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL12W::PORTA)
    }
    #[doc = "Port B pin 12 selected for external interrupt 12"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL12W::PORTB)
    }
    #[doc = "Port C pin 12 selected for external interrupt 12"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL12W::PORTC)
    }
    #[doc = "Port D pin 12 selected for external interrupt 12"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL12W::PORTD)
    }
    #[doc = "Port E pin 12 selected for external interrupt 12"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL12W::PORTE)
    }
    #[doc = "Port F pin 12 selected for external interrupt 12"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL12W::PORTF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL13`"]
pub enum EXTIPSEL13W {
    #[doc = "Port A pin 13 selected for external interrupt 13"]
    PORTA,
    #[doc = "Port B pin 13 selected for external interrupt 13"]
    PORTB,
    #[doc = "Port C pin 13 selected for external interrupt 13"]
    PORTC,
    #[doc = "Port D pin 13 selected for external interrupt 13"]
    PORTD,
    #[doc = "Port E pin 13 selected for external interrupt 13"]
    PORTE,
    #[doc = "Port F pin 13 selected for external interrupt 13"]
    PORTF,
}
impl EXTIPSEL13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL13W::PORTA => 0,
            EXTIPSEL13W::PORTB => 1,
            EXTIPSEL13W::PORTC => 2,
            EXTIPSEL13W::PORTD => 3,
            EXTIPSEL13W::PORTE => 4,
            EXTIPSEL13W::PORTF => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL13W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A pin 13 selected for external interrupt 13"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL13W::PORTA)
    }
    #[doc = "Port B pin 13 selected for external interrupt 13"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL13W::PORTB)
    }
    #[doc = "Port C pin 13 selected for external interrupt 13"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL13W::PORTC)
    }
    #[doc = "Port D pin 13 selected for external interrupt 13"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL13W::PORTD)
    }
    #[doc = "Port E pin 13 selected for external interrupt 13"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL13W::PORTE)
    }
    #[doc = "Port F pin 13 selected for external interrupt 13"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL13W::PORTF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL14`"]
pub enum EXTIPSEL14W {
    #[doc = "Port A pin 14 selected for external interrupt 14"]
    PORTA,
    #[doc = "Port B pin 14 selected for external interrupt 14"]
    PORTB,
    #[doc = "Port C pin 14 selected for external interrupt 14"]
    PORTC,
    #[doc = "Port D pin 14 selected for external interrupt 14"]
    PORTD,
    #[doc = "Port E pin 14 selected for external interrupt 14"]
    PORTE,
    #[doc = "Port F pin 14 selected for external interrupt 14"]
    PORTF,
}
impl EXTIPSEL14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL14W::PORTA => 0,
            EXTIPSEL14W::PORTB => 1,
            EXTIPSEL14W::PORTC => 2,
            EXTIPSEL14W::PORTD => 3,
            EXTIPSEL14W::PORTE => 4,
            EXTIPSEL14W::PORTF => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL14W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A pin 14 selected for external interrupt 14"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL14W::PORTA)
    }
    #[doc = "Port B pin 14 selected for external interrupt 14"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL14W::PORTB)
    }
    #[doc = "Port C pin 14 selected for external interrupt 14"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL14W::PORTC)
    }
    #[doc = "Port D pin 14 selected for external interrupt 14"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL14W::PORTD)
    }
    #[doc = "Port E pin 14 selected for external interrupt 14"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL14W::PORTE)
    }
    #[doc = "Port F pin 14 selected for external interrupt 14"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL14W::PORTF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL15`"]
pub enum EXTIPSEL15W {
    #[doc = "Port A pin 15 selected for external interrupt 15"]
    PORTA,
    #[doc = "Port B pin 15 selected for external interrupt 15"]
    PORTB,
    #[doc = "Port C pin 15 selected for external interrupt 15"]
    PORTC,
    #[doc = "Port D pin 15 selected for external interrupt 15"]
    PORTD,
    #[doc = "Port E pin 15 selected for external interrupt 15"]
    PORTE,
    #[doc = "Port F pin 15 selected for external interrupt 15"]
    PORTF,
}
impl EXTIPSEL15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL15W::PORTA => 0,
            EXTIPSEL15W::PORTB => 1,
            EXTIPSEL15W::PORTC => 2,
            EXTIPSEL15W::PORTD => 3,
            EXTIPSEL15W::PORTE => 4,
            EXTIPSEL15W::PORTF => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL15W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A pin 15 selected for external interrupt 15"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL15W::PORTA)
    }
    #[doc = "Port B pin 15 selected for external interrupt 15"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL15W::PORTB)
    }
    #[doc = "Port C pin 15 selected for external interrupt 15"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL15W::PORTC)
    }
    #[doc = "Port D pin 15 selected for external interrupt 15"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL15W::PORTD)
    }
    #[doc = "Port E pin 15 selected for external interrupt 15"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL15W::PORTE)
    }
    #[doc = "Port F pin 15 selected for external interrupt 15"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL15W::PORTF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - External Interrupt 8 Port Select"]
    #[inline]
    pub fn extipsel8(&self) -> EXTIPSEL8R {
        EXTIPSEL8R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - External Interrupt 9 Port Select"]
    #[inline]
    pub fn extipsel9(&self) -> EXTIPSEL9R {
        EXTIPSEL9R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - External Interrupt 10 Port Select"]
    #[inline]
    pub fn extipsel10(&self) -> EXTIPSEL10R {
        EXTIPSEL10R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - External Interrupt 11 Port Select"]
    #[inline]
    pub fn extipsel11(&self) -> EXTIPSEL11R {
        EXTIPSEL11R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - External Interrupt 12 Port Select"]
    #[inline]
    pub fn extipsel12(&self) -> EXTIPSEL12R {
        EXTIPSEL12R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - External Interrupt 13 Port Select"]
    #[inline]
    pub fn extipsel13(&self) -> EXTIPSEL13R {
        EXTIPSEL13R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - External Interrupt 14 Port Select"]
    #[inline]
    pub fn extipsel14(&self) -> EXTIPSEL14R {
        EXTIPSEL14R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - External Interrupt 15 Port Select"]
    #[inline]
    pub fn extipsel15(&self) -> EXTIPSEL15R {
        EXTIPSEL15R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:2 - External Interrupt 8 Port Select"]
    #[inline]
    pub fn extipsel8(&mut self) -> _EXTIPSEL8W {
        _EXTIPSEL8W { w: self }
    }
    #[doc = "Bits 4:6 - External Interrupt 9 Port Select"]
    #[inline]
    pub fn extipsel9(&mut self) -> _EXTIPSEL9W {
        _EXTIPSEL9W { w: self }
    }
    #[doc = "Bits 8:10 - External Interrupt 10 Port Select"]
    #[inline]
    pub fn extipsel10(&mut self) -> _EXTIPSEL10W {
        _EXTIPSEL10W { w: self }
    }
    #[doc = "Bits 12:14 - External Interrupt 11 Port Select"]
    #[inline]
    pub fn extipsel11(&mut self) -> _EXTIPSEL11W {
        _EXTIPSEL11W { w: self }
    }
    #[doc = "Bits 16:18 - External Interrupt 12 Port Select"]
    #[inline]
    pub fn extipsel12(&mut self) -> _EXTIPSEL12W {
        _EXTIPSEL12W { w: self }
    }
    #[doc = "Bits 20:22 - External Interrupt 13 Port Select"]
    #[inline]
    pub fn extipsel13(&mut self) -> _EXTIPSEL13W {
        _EXTIPSEL13W { w: self }
    }
    #[doc = "Bits 24:26 - External Interrupt 14 Port Select"]
    #[inline]
    pub fn extipsel14(&mut self) -> _EXTIPSEL14W {
        _EXTIPSEL14W { w: self }
    }
    #[doc = "Bits 28:30 - External Interrupt 15 Port Select"]
    #[inline]
    pub fn extipsel15(&mut self) -> _EXTIPSEL15W {
        _EXTIPSEL15W { w: self }
    }
}
