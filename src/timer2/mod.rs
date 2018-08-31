#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x10 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x14 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x18 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x1c - Counter Top Value Register"]
    pub top: TOP,
    #[doc = "0x20 - Counter Top Value Buffer Register"]
    pub topb: TOPB,
    #[doc = "0x24 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x28 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved0: [u8; 4usize],
    #[doc = "0x30 - CC Channel Control Register"]
    pub cc0_ctrl: CC0_CTRL,
    #[doc = "0x34 - CC Channel Value Register"]
    pub cc0_ccv: CC0_CCV,
    #[doc = "0x38 - CC Channel Value Peek Register"]
    pub cc0_ccvp: CC0_CCVP,
    #[doc = "0x3c - CC Channel Buffer Register"]
    pub cc0_ccvb: CC0_CCVB,
    #[doc = "0x40 - CC Channel Control Register"]
    pub cc1_ctrl: CC1_CTRL,
    #[doc = "0x44 - CC Channel Value Register"]
    pub cc1_ccv: CC1_CCV,
    #[doc = "0x48 - CC Channel Value Peek Register"]
    pub cc1_ccvp: CC1_CCVP,
    #[doc = "0x4c - CC Channel Buffer Register"]
    pub cc1_ccvb: CC1_CCVB,
    #[doc = "0x50 - CC Channel Control Register"]
    pub cc2_ctrl: CC2_CTRL,
    #[doc = "0x54 - CC Channel Value Register"]
    pub cc2_ccv: CC2_CCV,
    #[doc = "0x58 - CC Channel Value Peek Register"]
    pub cc2_ccvp: CC2_CCVP,
    #[doc = "0x5c - CC Channel Buffer Register"]
    pub cc2_ccvb: CC2_CCVB,
    _reserved1: [u8; 16usize],
    #[doc = "0x70 - DTI Control Register"]
    pub dtctrl: DTCTRL,
    #[doc = "0x74 - DTI Time Control Register"]
    pub dttime: DTTIME,
    #[doc = "0x78 - DTI Fault Configuration Register"]
    pub dtfc: DTFC,
    #[doc = "0x7c - DTI Output Generation Enable Register"]
    pub dtogen: DTOGEN,
    #[doc = "0x80 - DTI Fault Register"]
    pub dtfault: DTFAULT,
    #[doc = "0x84 - DTI Fault Clear Register"]
    pub dtfaultc: DTFAULTC,
    #[doc = "0x88 - DTI Configuration Lock Register"]
    pub dtlock: DTLOCK,
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
#[doc = "Counter Top Value Register"]
pub struct TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Top Value Register"]
pub mod top;
#[doc = "Counter Top Value Buffer Register"]
pub struct TOPB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Top Value Buffer Register"]
pub mod topb;
#[doc = "Counter Value Register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CC Channel Control Register"]
pub struct CC0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "CC Channel Value Register"]
pub struct CC0_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Register"]
pub mod cc0_ccv;
#[doc = "CC Channel Value Peek Register"]
pub struct CC0_CCVP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Peek Register"]
pub mod cc0_ccvp;
#[doc = "CC Channel Buffer Register"]
pub struct CC0_CCVB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Buffer Register"]
pub mod cc0_ccvb;
#[doc = "CC Channel Control Register"]
pub struct CC1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "CC Channel Value Register"]
pub struct CC1_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Register"]
pub mod cc1_ccv;
#[doc = "CC Channel Value Peek Register"]
pub struct CC1_CCVP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Peek Register"]
pub mod cc1_ccvp;
#[doc = "CC Channel Buffer Register"]
pub struct CC1_CCVB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Buffer Register"]
pub mod cc1_ccvb;
#[doc = "CC Channel Control Register"]
pub struct CC2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "CC Channel Value Register"]
pub struct CC2_CCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Register"]
pub mod cc2_ccv;
#[doc = "CC Channel Value Peek Register"]
pub struct CC2_CCVP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Value Peek Register"]
pub mod cc2_ccvp;
#[doc = "CC Channel Buffer Register"]
pub struct CC2_CCVB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CC Channel Buffer Register"]
pub mod cc2_ccvb;
#[doc = "DTI Control Register"]
pub struct DTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Control Register"]
pub mod dtctrl;
#[doc = "DTI Time Control Register"]
pub struct DTTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Time Control Register"]
pub mod dttime;
#[doc = "DTI Fault Configuration Register"]
pub struct DTFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Fault Configuration Register"]
pub mod dtfc;
#[doc = "DTI Output Generation Enable Register"]
pub struct DTOGEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Output Generation Enable Register"]
pub mod dtogen;
#[doc = "DTI Fault Register"]
pub struct DTFAULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Fault Register"]
pub mod dtfault;
#[doc = "DTI Fault Clear Register"]
pub struct DTFAULTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Fault Clear Register"]
pub mod dtfaultc;
#[doc = "DTI Configuration Lock Register"]
pub struct DTLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTI Configuration Lock Register"]
pub mod dtlock;
