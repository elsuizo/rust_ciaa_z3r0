#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Configuration Lock Register"]
    pub lock: LOCK,
    _reserved1: [u8; 24usize],
    #[doc = "0x24 - Auxiliary Control Register"]
    pub auxctrl: AUXCTRL,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "Auxiliary Control Register"]
pub struct AUXCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Control Register"]
pub mod auxctrl;
