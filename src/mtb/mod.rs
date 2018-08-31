#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB Trace Position Register."]
    pub position: POSITION,
    #[doc = "0x04 - MTB Trace Control Register"]
    pub master: MASTER,
    #[doc = "0x08 - MTB Trace Flow Register"]
    pub flow: FLOW,
    #[doc = "0x0c - MTB Trace Base Register"]
    pub base: BASE,
}
#[doc = "MTB Trace Position Register."]
pub struct POSITION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Trace Position Register."]
pub mod position;
#[doc = "MTB Trace Control Register"]
pub struct MASTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Trace Control Register"]
pub mod master;
#[doc = "MTB Trace Flow Register"]
pub struct FLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Trace Flow Register"]
pub mod flow;
#[doc = "MTB Trace Base Register"]
pub struct BASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MTB Trace Base Register"]
pub mod base;
