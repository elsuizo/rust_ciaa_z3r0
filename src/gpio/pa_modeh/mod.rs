#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PA_MODEH {
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
#[doc = "Possible values of the field `MODE8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE8R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE8R::DISABLED => 0,
            MODE8R::INPUT => 1,
            MODE8R::INPUTPULL => 2,
            MODE8R::INPUTPULLFILTER => 3,
            MODE8R::PUSHPULL => 4,
            MODE8R::PUSHPULLDRIVE => 5,
            MODE8R::WIREDOR => 6,
            MODE8R::WIREDORPULLDOWN => 7,
            MODE8R::WIREDAND => 8,
            MODE8R::WIREDANDFILTER => 9,
            MODE8R::WIREDANDPULLUP => 10,
            MODE8R::WIREDANDPULLUPFILTER => 11,
            MODE8R::WIREDANDDRIVE => 12,
            MODE8R::WIREDANDDRIVEFILTER => 13,
            MODE8R::WIREDANDDRIVEPULLUP => 14,
            MODE8R::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE8R {
        match value {
            0 => MODE8R::DISABLED,
            1 => MODE8R::INPUT,
            2 => MODE8R::INPUTPULL,
            3 => MODE8R::INPUTPULLFILTER,
            4 => MODE8R::PUSHPULL,
            5 => MODE8R::PUSHPULLDRIVE,
            6 => MODE8R::WIREDOR,
            7 => MODE8R::WIREDORPULLDOWN,
            8 => MODE8R::WIREDAND,
            9 => MODE8R::WIREDANDFILTER,
            10 => MODE8R::WIREDANDPULLUP,
            11 => MODE8R::WIREDANDPULLUPFILTER,
            12 => MODE8R::WIREDANDDRIVE,
            13 => MODE8R::WIREDANDDRIVEFILTER,
            14 => MODE8R::WIREDANDDRIVEPULLUP,
            15 => MODE8R::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE8R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE8R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE8R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE8R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE8R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE8R::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE8R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE8R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE8R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE8R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE8R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE8R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE8R::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE8R::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE8R::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE8R::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE9R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE9R::DISABLED => 0,
            MODE9R::INPUT => 1,
            MODE9R::INPUTPULL => 2,
            MODE9R::INPUTPULLFILTER => 3,
            MODE9R::PUSHPULL => 4,
            MODE9R::PUSHPULLDRIVE => 5,
            MODE9R::WIREDOR => 6,
            MODE9R::WIREDORPULLDOWN => 7,
            MODE9R::WIREDAND => 8,
            MODE9R::WIREDANDFILTER => 9,
            MODE9R::WIREDANDPULLUP => 10,
            MODE9R::WIREDANDPULLUPFILTER => 11,
            MODE9R::WIREDANDDRIVE => 12,
            MODE9R::WIREDANDDRIVEFILTER => 13,
            MODE9R::WIREDANDDRIVEPULLUP => 14,
            MODE9R::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE9R {
        match value {
            0 => MODE9R::DISABLED,
            1 => MODE9R::INPUT,
            2 => MODE9R::INPUTPULL,
            3 => MODE9R::INPUTPULLFILTER,
            4 => MODE9R::PUSHPULL,
            5 => MODE9R::PUSHPULLDRIVE,
            6 => MODE9R::WIREDOR,
            7 => MODE9R::WIREDORPULLDOWN,
            8 => MODE9R::WIREDAND,
            9 => MODE9R::WIREDANDFILTER,
            10 => MODE9R::WIREDANDPULLUP,
            11 => MODE9R::WIREDANDPULLUPFILTER,
            12 => MODE9R::WIREDANDDRIVE,
            13 => MODE9R::WIREDANDDRIVEFILTER,
            14 => MODE9R::WIREDANDDRIVEPULLUP,
            15 => MODE9R::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE9R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE9R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE9R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE9R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE9R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE9R::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE9R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE9R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE9R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE9R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE9R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE9R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE9R::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE9R::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE9R::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE9R::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE10R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE10R::DISABLED => 0,
            MODE10R::INPUT => 1,
            MODE10R::INPUTPULL => 2,
            MODE10R::INPUTPULLFILTER => 3,
            MODE10R::PUSHPULL => 4,
            MODE10R::PUSHPULLDRIVE => 5,
            MODE10R::WIREDOR => 6,
            MODE10R::WIREDORPULLDOWN => 7,
            MODE10R::WIREDAND => 8,
            MODE10R::WIREDANDFILTER => 9,
            MODE10R::WIREDANDPULLUP => 10,
            MODE10R::WIREDANDPULLUPFILTER => 11,
            MODE10R::WIREDANDDRIVE => 12,
            MODE10R::WIREDANDDRIVEFILTER => 13,
            MODE10R::WIREDANDDRIVEPULLUP => 14,
            MODE10R::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE10R {
        match value {
            0 => MODE10R::DISABLED,
            1 => MODE10R::INPUT,
            2 => MODE10R::INPUTPULL,
            3 => MODE10R::INPUTPULLFILTER,
            4 => MODE10R::PUSHPULL,
            5 => MODE10R::PUSHPULLDRIVE,
            6 => MODE10R::WIREDOR,
            7 => MODE10R::WIREDORPULLDOWN,
            8 => MODE10R::WIREDAND,
            9 => MODE10R::WIREDANDFILTER,
            10 => MODE10R::WIREDANDPULLUP,
            11 => MODE10R::WIREDANDPULLUPFILTER,
            12 => MODE10R::WIREDANDDRIVE,
            13 => MODE10R::WIREDANDDRIVEFILTER,
            14 => MODE10R::WIREDANDDRIVEPULLUP,
            15 => MODE10R::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE10R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE10R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE10R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE10R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE10R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE10R::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE10R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE10R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE10R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE10R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE10R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE10R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE10R::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE10R::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE10R::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE10R::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE11R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE11R::DISABLED => 0,
            MODE11R::INPUT => 1,
            MODE11R::INPUTPULL => 2,
            MODE11R::INPUTPULLFILTER => 3,
            MODE11R::PUSHPULL => 4,
            MODE11R::PUSHPULLDRIVE => 5,
            MODE11R::WIREDOR => 6,
            MODE11R::WIREDORPULLDOWN => 7,
            MODE11R::WIREDAND => 8,
            MODE11R::WIREDANDFILTER => 9,
            MODE11R::WIREDANDPULLUP => 10,
            MODE11R::WIREDANDPULLUPFILTER => 11,
            MODE11R::WIREDANDDRIVE => 12,
            MODE11R::WIREDANDDRIVEFILTER => 13,
            MODE11R::WIREDANDDRIVEPULLUP => 14,
            MODE11R::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE11R {
        match value {
            0 => MODE11R::DISABLED,
            1 => MODE11R::INPUT,
            2 => MODE11R::INPUTPULL,
            3 => MODE11R::INPUTPULLFILTER,
            4 => MODE11R::PUSHPULL,
            5 => MODE11R::PUSHPULLDRIVE,
            6 => MODE11R::WIREDOR,
            7 => MODE11R::WIREDORPULLDOWN,
            8 => MODE11R::WIREDAND,
            9 => MODE11R::WIREDANDFILTER,
            10 => MODE11R::WIREDANDPULLUP,
            11 => MODE11R::WIREDANDPULLUPFILTER,
            12 => MODE11R::WIREDANDDRIVE,
            13 => MODE11R::WIREDANDDRIVEFILTER,
            14 => MODE11R::WIREDANDDRIVEPULLUP,
            15 => MODE11R::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE11R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE11R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE11R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE11R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE11R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE11R::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE11R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE11R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE11R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE11R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE11R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE11R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE11R::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE11R::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE11R::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE11R::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE12R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE12R::DISABLED => 0,
            MODE12R::INPUT => 1,
            MODE12R::INPUTPULL => 2,
            MODE12R::INPUTPULLFILTER => 3,
            MODE12R::PUSHPULL => 4,
            MODE12R::PUSHPULLDRIVE => 5,
            MODE12R::WIREDOR => 6,
            MODE12R::WIREDORPULLDOWN => 7,
            MODE12R::WIREDAND => 8,
            MODE12R::WIREDANDFILTER => 9,
            MODE12R::WIREDANDPULLUP => 10,
            MODE12R::WIREDANDPULLUPFILTER => 11,
            MODE12R::WIREDANDDRIVE => 12,
            MODE12R::WIREDANDDRIVEFILTER => 13,
            MODE12R::WIREDANDDRIVEPULLUP => 14,
            MODE12R::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE12R {
        match value {
            0 => MODE12R::DISABLED,
            1 => MODE12R::INPUT,
            2 => MODE12R::INPUTPULL,
            3 => MODE12R::INPUTPULLFILTER,
            4 => MODE12R::PUSHPULL,
            5 => MODE12R::PUSHPULLDRIVE,
            6 => MODE12R::WIREDOR,
            7 => MODE12R::WIREDORPULLDOWN,
            8 => MODE12R::WIREDAND,
            9 => MODE12R::WIREDANDFILTER,
            10 => MODE12R::WIREDANDPULLUP,
            11 => MODE12R::WIREDANDPULLUPFILTER,
            12 => MODE12R::WIREDANDDRIVE,
            13 => MODE12R::WIREDANDDRIVEFILTER,
            14 => MODE12R::WIREDANDDRIVEPULLUP,
            15 => MODE12R::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE12R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE12R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE12R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE12R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE12R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE12R::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE12R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE12R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE12R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE12R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE12R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE12R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE12R::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE12R::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE12R::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE12R::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE13R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE13R::DISABLED => 0,
            MODE13R::INPUT => 1,
            MODE13R::INPUTPULL => 2,
            MODE13R::INPUTPULLFILTER => 3,
            MODE13R::PUSHPULL => 4,
            MODE13R::PUSHPULLDRIVE => 5,
            MODE13R::WIREDOR => 6,
            MODE13R::WIREDORPULLDOWN => 7,
            MODE13R::WIREDAND => 8,
            MODE13R::WIREDANDFILTER => 9,
            MODE13R::WIREDANDPULLUP => 10,
            MODE13R::WIREDANDPULLUPFILTER => 11,
            MODE13R::WIREDANDDRIVE => 12,
            MODE13R::WIREDANDDRIVEFILTER => 13,
            MODE13R::WIREDANDDRIVEPULLUP => 14,
            MODE13R::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE13R {
        match value {
            0 => MODE13R::DISABLED,
            1 => MODE13R::INPUT,
            2 => MODE13R::INPUTPULL,
            3 => MODE13R::INPUTPULLFILTER,
            4 => MODE13R::PUSHPULL,
            5 => MODE13R::PUSHPULLDRIVE,
            6 => MODE13R::WIREDOR,
            7 => MODE13R::WIREDORPULLDOWN,
            8 => MODE13R::WIREDAND,
            9 => MODE13R::WIREDANDFILTER,
            10 => MODE13R::WIREDANDPULLUP,
            11 => MODE13R::WIREDANDPULLUPFILTER,
            12 => MODE13R::WIREDANDDRIVE,
            13 => MODE13R::WIREDANDDRIVEFILTER,
            14 => MODE13R::WIREDANDDRIVEPULLUP,
            15 => MODE13R::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE13R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE13R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE13R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE13R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE13R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE13R::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE13R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE13R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE13R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE13R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE13R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE13R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE13R::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE13R::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE13R::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE13R::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE14R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE14R::DISABLED => 0,
            MODE14R::INPUT => 1,
            MODE14R::INPUTPULL => 2,
            MODE14R::INPUTPULLFILTER => 3,
            MODE14R::PUSHPULL => 4,
            MODE14R::PUSHPULLDRIVE => 5,
            MODE14R::WIREDOR => 6,
            MODE14R::WIREDORPULLDOWN => 7,
            MODE14R::WIREDAND => 8,
            MODE14R::WIREDANDFILTER => 9,
            MODE14R::WIREDANDPULLUP => 10,
            MODE14R::WIREDANDPULLUPFILTER => 11,
            MODE14R::WIREDANDDRIVE => 12,
            MODE14R::WIREDANDDRIVEFILTER => 13,
            MODE14R::WIREDANDDRIVEPULLUP => 14,
            MODE14R::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE14R {
        match value {
            0 => MODE14R::DISABLED,
            1 => MODE14R::INPUT,
            2 => MODE14R::INPUTPULL,
            3 => MODE14R::INPUTPULLFILTER,
            4 => MODE14R::PUSHPULL,
            5 => MODE14R::PUSHPULLDRIVE,
            6 => MODE14R::WIREDOR,
            7 => MODE14R::WIREDORPULLDOWN,
            8 => MODE14R::WIREDAND,
            9 => MODE14R::WIREDANDFILTER,
            10 => MODE14R::WIREDANDPULLUP,
            11 => MODE14R::WIREDANDPULLUPFILTER,
            12 => MODE14R::WIREDANDDRIVE,
            13 => MODE14R::WIREDANDDRIVEFILTER,
            14 => MODE14R::WIREDANDDRIVEPULLUP,
            15 => MODE14R::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE14R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE14R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE14R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE14R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE14R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE14R::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE14R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE14R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE14R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE14R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE14R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE14R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE14R::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE14R::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE14R::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE14R::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Possible values of the field `MODE15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE15R {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE15R::DISABLED => 0,
            MODE15R::INPUT => 1,
            MODE15R::INPUTPULL => 2,
            MODE15R::INPUTPULLFILTER => 3,
            MODE15R::PUSHPULL => 4,
            MODE15R::PUSHPULLDRIVE => 5,
            MODE15R::WIREDOR => 6,
            MODE15R::WIREDORPULLDOWN => 7,
            MODE15R::WIREDAND => 8,
            MODE15R::WIREDANDFILTER => 9,
            MODE15R::WIREDANDPULLUP => 10,
            MODE15R::WIREDANDPULLUPFILTER => 11,
            MODE15R::WIREDANDDRIVE => 12,
            MODE15R::WIREDANDDRIVEFILTER => 13,
            MODE15R::WIREDANDDRIVEPULLUP => 14,
            MODE15R::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE15R {
        match value {
            0 => MODE15R::DISABLED,
            1 => MODE15R::INPUT,
            2 => MODE15R::INPUTPULL,
            3 => MODE15R::INPUTPULLFILTER,
            4 => MODE15R::PUSHPULL,
            5 => MODE15R::PUSHPULLDRIVE,
            6 => MODE15R::WIREDOR,
            7 => MODE15R::WIREDORPULLDOWN,
            8 => MODE15R::WIREDAND,
            9 => MODE15R::WIREDANDFILTER,
            10 => MODE15R::WIREDANDPULLUP,
            11 => MODE15R::WIREDANDPULLUPFILTER,
            12 => MODE15R::WIREDANDDRIVE,
            13 => MODE15R::WIREDANDDRIVEFILTER,
            14 => MODE15R::WIREDANDDRIVEPULLUP,
            15 => MODE15R::WIREDANDDRIVEPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODE15R::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == MODE15R::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE15R::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE15R::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE15R::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLDRIVE`"]
    #[inline]
    pub fn is_pushpulldrive(&self) -> bool {
        *self == MODE15R::PUSHPULLDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE15R::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE15R::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE15R::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE15R::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE15R::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE15R::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVE`"]
    #[inline]
    pub fn is_wiredanddrive(&self) -> bool {
        *self == MODE15R::WIREDANDDRIVE
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEFILTER`"]
    #[inline]
    pub fn is_wiredanddrivefilter(&self) -> bool {
        *self == MODE15R::WIREDANDDRIVEFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUP`"]
    #[inline]
    pub fn is_wiredanddrivepullup(&self) -> bool {
        *self == MODE15R::WIREDANDDRIVEPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDDRIVEPULLUPFILTER`"]
    #[inline]
    pub fn is_wiredanddrivepullupfilter(&self) -> bool {
        *self == MODE15R::WIREDANDDRIVEPULLUPFILTER
    }
}
#[doc = "Values that can be written to the field `MODE8`"]
pub enum MODE8W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE8W::DISABLED => 0,
            MODE8W::INPUT => 1,
            MODE8W::INPUTPULL => 2,
            MODE8W::INPUTPULLFILTER => 3,
            MODE8W::PUSHPULL => 4,
            MODE8W::PUSHPULLDRIVE => 5,
            MODE8W::WIREDOR => 6,
            MODE8W::WIREDORPULLDOWN => 7,
            MODE8W::WIREDAND => 8,
            MODE8W::WIREDANDFILTER => 9,
            MODE8W::WIREDANDPULLUP => 10,
            MODE8W::WIREDANDPULLUPFILTER => 11,
            MODE8W::WIREDANDDRIVE => 12,
            MODE8W::WIREDANDDRIVEFILTER => 13,
            MODE8W::WIREDANDDRIVEPULLUP => 14,
            MODE8W::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE8W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE8W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE8W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE8W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE8W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE8W::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE8W::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE8W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE8W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE8W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE8W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE8W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE8W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE8W::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE8W::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE8W::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE8W::WIREDANDDRIVEPULLUPFILTER)
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
#[doc = "Values that can be written to the field `MODE9`"]
pub enum MODE9W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE9W::DISABLED => 0,
            MODE9W::INPUT => 1,
            MODE9W::INPUTPULL => 2,
            MODE9W::INPUTPULLFILTER => 3,
            MODE9W::PUSHPULL => 4,
            MODE9W::PUSHPULLDRIVE => 5,
            MODE9W::WIREDOR => 6,
            MODE9W::WIREDORPULLDOWN => 7,
            MODE9W::WIREDAND => 8,
            MODE9W::WIREDANDFILTER => 9,
            MODE9W::WIREDANDPULLUP => 10,
            MODE9W::WIREDANDPULLUPFILTER => 11,
            MODE9W::WIREDANDDRIVE => 12,
            MODE9W::WIREDANDDRIVEFILTER => 13,
            MODE9W::WIREDANDDRIVEPULLUP => 14,
            MODE9W::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE9W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE9W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE9W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE9W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE9W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE9W::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE9W::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE9W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE9W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE9W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE9W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE9W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE9W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE9W::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE9W::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE9W::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE9W::WIREDANDDRIVEPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE10`"]
pub enum MODE10W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE10W::DISABLED => 0,
            MODE10W::INPUT => 1,
            MODE10W::INPUTPULL => 2,
            MODE10W::INPUTPULLFILTER => 3,
            MODE10W::PUSHPULL => 4,
            MODE10W::PUSHPULLDRIVE => 5,
            MODE10W::WIREDOR => 6,
            MODE10W::WIREDORPULLDOWN => 7,
            MODE10W::WIREDAND => 8,
            MODE10W::WIREDANDFILTER => 9,
            MODE10W::WIREDANDPULLUP => 10,
            MODE10W::WIREDANDPULLUPFILTER => 11,
            MODE10W::WIREDANDDRIVE => 12,
            MODE10W::WIREDANDDRIVEFILTER => 13,
            MODE10W::WIREDANDDRIVEPULLUP => 14,
            MODE10W::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE10W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE10W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE10W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE10W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE10W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE10W::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE10W::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE10W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE10W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE10W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE10W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE10W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE10W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE10W::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE10W::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE10W::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE10W::WIREDANDDRIVEPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE11`"]
pub enum MODE11W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE11W::DISABLED => 0,
            MODE11W::INPUT => 1,
            MODE11W::INPUTPULL => 2,
            MODE11W::INPUTPULLFILTER => 3,
            MODE11W::PUSHPULL => 4,
            MODE11W::PUSHPULLDRIVE => 5,
            MODE11W::WIREDOR => 6,
            MODE11W::WIREDORPULLDOWN => 7,
            MODE11W::WIREDAND => 8,
            MODE11W::WIREDANDFILTER => 9,
            MODE11W::WIREDANDPULLUP => 10,
            MODE11W::WIREDANDPULLUPFILTER => 11,
            MODE11W::WIREDANDDRIVE => 12,
            MODE11W::WIREDANDDRIVEFILTER => 13,
            MODE11W::WIREDANDDRIVEPULLUP => 14,
            MODE11W::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE11W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE11W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE11W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE11W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE11W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE11W::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE11W::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE11W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE11W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE11W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE11W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE11W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE11W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE11W::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE11W::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE11W::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE11W::WIREDANDDRIVEPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE12`"]
pub enum MODE12W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE12W::DISABLED => 0,
            MODE12W::INPUT => 1,
            MODE12W::INPUTPULL => 2,
            MODE12W::INPUTPULLFILTER => 3,
            MODE12W::PUSHPULL => 4,
            MODE12W::PUSHPULLDRIVE => 5,
            MODE12W::WIREDOR => 6,
            MODE12W::WIREDORPULLDOWN => 7,
            MODE12W::WIREDAND => 8,
            MODE12W::WIREDANDFILTER => 9,
            MODE12W::WIREDANDPULLUP => 10,
            MODE12W::WIREDANDPULLUPFILTER => 11,
            MODE12W::WIREDANDDRIVE => 12,
            MODE12W::WIREDANDDRIVEFILTER => 13,
            MODE12W::WIREDANDDRIVEPULLUP => 14,
            MODE12W::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE12W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE12W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE12W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE12W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE12W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE12W::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE12W::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE12W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE12W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE12W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE12W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE12W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE12W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE12W::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE12W::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE12W::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE12W::WIREDANDDRIVEPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE13`"]
pub enum MODE13W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE13W::DISABLED => 0,
            MODE13W::INPUT => 1,
            MODE13W::INPUTPULL => 2,
            MODE13W::INPUTPULLFILTER => 3,
            MODE13W::PUSHPULL => 4,
            MODE13W::PUSHPULLDRIVE => 5,
            MODE13W::WIREDOR => 6,
            MODE13W::WIREDORPULLDOWN => 7,
            MODE13W::WIREDAND => 8,
            MODE13W::WIREDANDFILTER => 9,
            MODE13W::WIREDANDPULLUP => 10,
            MODE13W::WIREDANDPULLUPFILTER => 11,
            MODE13W::WIREDANDDRIVE => 12,
            MODE13W::WIREDANDDRIVEFILTER => 13,
            MODE13W::WIREDANDDRIVEPULLUP => 14,
            MODE13W::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE13W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE13W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE13W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE13W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE13W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE13W::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE13W::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE13W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE13W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE13W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE13W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE13W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE13W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE13W::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE13W::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE13W::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE13W::WIREDANDDRIVEPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE14`"]
pub enum MODE14W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE14W::DISABLED => 0,
            MODE14W::INPUT => 1,
            MODE14W::INPUTPULL => 2,
            MODE14W::INPUTPULLFILTER => 3,
            MODE14W::PUSHPULL => 4,
            MODE14W::PUSHPULLDRIVE => 5,
            MODE14W::WIREDOR => 6,
            MODE14W::WIREDORPULLDOWN => 7,
            MODE14W::WIREDAND => 8,
            MODE14W::WIREDANDFILTER => 9,
            MODE14W::WIREDANDPULLUP => 10,
            MODE14W::WIREDANDPULLUPFILTER => 11,
            MODE14W::WIREDANDDRIVE => 12,
            MODE14W::WIREDANDDRIVEFILTER => 13,
            MODE14W::WIREDANDDRIVEPULLUP => 14,
            MODE14W::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE14W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE14W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE14W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE14W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE14W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE14W::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE14W::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE14W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE14W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE14W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE14W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE14W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE14W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE14W::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE14W::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE14W::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE14W::WIREDANDDRIVEPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE15`"]
pub enum MODE15W {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "Push-pull output"]
    PUSHPULL,
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    PUSHPULLDRIVE,
    #[doc = "Wired-or output"]
    WIREDOR,
    #[doc = "Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "Open-drain output"]
    WIREDAND,
    #[doc = "Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    WIREDANDDRIVE,
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEFILTER,
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUP,
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    WIREDANDDRIVEPULLUPFILTER,
}
impl MODE15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE15W::DISABLED => 0,
            MODE15W::INPUT => 1,
            MODE15W::INPUTPULL => 2,
            MODE15W::INPUTPULLFILTER => 3,
            MODE15W::PUSHPULL => 4,
            MODE15W::PUSHPULLDRIVE => 5,
            MODE15W::WIREDOR => 6,
            MODE15W::WIREDORPULLDOWN => 7,
            MODE15W::WIREDAND => 8,
            MODE15W::WIREDANDFILTER => 9,
            MODE15W::WIREDANDPULLUP => 10,
            MODE15W::WIREDANDPULLUPFILTER => 11,
            MODE15W::WIREDANDDRIVE => 12,
            MODE15W::WIREDANDDRIVEFILTER => 13,
            MODE15W::WIREDANDDRIVEPULLUP => 14,
            MODE15W::WIREDANDDRIVEPULLUPFILTER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE15W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE15W::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE15W::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE15W::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE15W::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE15W::PUSHPULL)
    }
    #[doc = "Push-pull output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn pushpulldrive(self) -> &'a mut W {
        self.variant(MODE15W::PUSHPULLDRIVE)
    }
    #[doc = "Wired-or output"]
    #[inline]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE15W::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE15W::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE15W::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE15W::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE15W::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE15W::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output with drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrive(self) -> &'a mut W {
        self.variant(MODE15W::WIREDANDDRIVE)
    }
    #[doc = "Open-drain output with filter and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivefilter(self) -> &'a mut W {
        self.variant(MODE15W::WIREDANDDRIVEFILTER)
    }
    #[doc = "Open-drain output with pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullup(self) -> &'a mut W {
        self.variant(MODE15W::WIREDANDDRIVEPULLUP)
    }
    #[doc = "Open-drain output with filter, pullup and drive-strength set by DRIVEMODE"]
    #[inline]
    pub fn wiredanddrivepullupfilter(self) -> &'a mut W {
        self.variant(MODE15W::WIREDANDDRIVEPULLUPFILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline]
    pub fn mode8(&self) -> MODE8R {
        MODE8R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline]
    pub fn mode9(&self) -> MODE9R {
        MODE9R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline]
    pub fn mode10(&self) -> MODE10R {
        MODE10R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline]
    pub fn mode11(&self) -> MODE11R {
        MODE11R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline]
    pub fn mode12(&self) -> MODE12R {
        MODE12R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline]
    pub fn mode13(&self) -> MODE13R {
        MODE13R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline]
    pub fn mode14(&self) -> MODE14R {
        MODE14R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline]
    pub fn mode15(&self) -> MODE15R {
        MODE15R::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline]
    pub fn mode8(&mut self) -> _MODE8W {
        _MODE8W { w: self }
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline]
    pub fn mode9(&mut self) -> _MODE9W {
        _MODE9W { w: self }
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline]
    pub fn mode10(&mut self) -> _MODE10W {
        _MODE10W { w: self }
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline]
    pub fn mode11(&mut self) -> _MODE11W {
        _MODE11W { w: self }
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline]
    pub fn mode12(&mut self) -> _MODE12W {
        _MODE12W { w: self }
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline]
    pub fn mode13(&mut self) -> _MODE13W {
        _MODE13W { w: self }
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline]
    pub fn mode14(&mut self) -> _MODE14W {
        _MODE14W { w: self }
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline]
    pub fn mode15(&mut self) -> _MODE15W {
        _MODE15W { w: self }
    }
}
