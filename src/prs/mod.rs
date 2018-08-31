#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    pub swpulse: SWPULSE,
    #[doc = "0x04 - Software Level Register"]
    pub swlevel: SWLEVEL,
    #[doc = "0x08 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Channel Control Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x14 - Channel Control Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0x18 - Channel Control Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0x1c - Channel Control Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x20 - Channel Control Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x24 - Channel Control Register"]
    pub ch5_ctrl: CH5_CTRL,
    _reserved1: [u8; 24usize],
    #[doc = "0x40 - MTB Trace Control Register"]
    pub tracectrl: TRACECTRL,
}
#[doc = "Software Pulse Register"]
pub struct SWPULSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "Software Level Register"]
pub struct SWLEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "Channel Control Register"]
pub struct CH0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "Channel Control Register"]
pub struct CH1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "Channel Control Register"]
pub struct CH2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "Channel Control Register"]
pub struct CH3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "Channel Control Register"]
pub struct CH4_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "Channel Control Register"]
pub struct CH5_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "MTB Trace Control Register"]
pub struct TRACECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Trace Control Register"]
pub mod tracectrl;
