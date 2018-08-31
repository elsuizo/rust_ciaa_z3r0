#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Single Sample Control Register"]
    pub singlectrl: SINGLECTRL,
    #[doc = "0x10 - Scan Control Register"]
    pub scanctrl: SCANCTRL,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x18 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x1c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x24 - Single Conversion Result Data"]
    pub singledata: SINGLEDATA,
    #[doc = "0x28 - Scan Conversion Result Data"]
    pub scandata: SCANDATA,
    #[doc = "0x2c - Single Conversion Result Data Peek Register"]
    pub singledatap: SINGLEDATAP,
    #[doc = "0x30 - Scan Sequence Result Data Peek Register"]
    pub scandatap: SCANDATAP,
    #[doc = "0x34 - Calibration Register"]
    pub cal: CAL,
    _reserved0: [u8; 4usize],
    #[doc = "0x3c - Bias Programming Register"]
    pub biasprog: BIASPROG,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Single Sample Control Register"]
pub struct SINGLECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Sample Control Register"]
pub mod singlectrl;
#[doc = "Scan Control Register"]
pub struct SCANCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Control Register"]
pub mod scanctrl;
#[doc = "Interrupt Enable Register"]
pub struct IEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "Interrupt Flag Register"]
pub struct IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register"]
pub struct IFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register"]
pub struct IFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Single Conversion Result Data"]
pub struct SINGLEDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "Scan Conversion Result Data"]
pub struct SCANDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "Single Conversion Result Data Peek Register"]
pub struct SINGLEDATAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "Scan Sequence Result Data Peek Register"]
pub struct SCANDATAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "Calibration Register"]
pub struct CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "Bias Programming Register"]
pub struct BIASPROG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bias Programming Register"]
pub mod biasprog;
