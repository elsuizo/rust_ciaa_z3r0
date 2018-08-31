#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current Programming Register"]
    pub curprog: CURPROG,
    #[doc = "0x08 - Calibration Register"]
    pub cal: CAL,
    #[doc = "0x0c - Duty Cycle Configauration Register"]
    pub dutyconfig: DUTYCONFIG,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Current Programming Register"]
pub struct CURPROG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "Calibration Register"]
pub struct CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "Duty Cycle Configauration Register"]
pub struct DUTYCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Duty Cycle Configauration Register"]
pub mod dutyconfig;
