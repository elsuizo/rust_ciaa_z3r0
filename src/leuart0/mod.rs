#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Clock Control Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x10 - Start Frame Register"]
    pub startframe: STARTFRAME,
    #[doc = "0x14 - Signal Frame Register"]
    pub sigframe: SIGFRAME,
    #[doc = "0x18 - Receive Buffer Data Extended Register"]
    pub rxdatax: RXDATAX,
    #[doc = "0x1c - Receive Buffer Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x20 - Receive Buffer Data Extended Peek Register"]
    pub rxdataxp: RXDATAXP,
    #[doc = "0x24 - Transmit Buffer Data Extended Register"]
    pub txdatax: TXDATAX,
    #[doc = "0x28 - Transmit Buffer Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x2c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x30 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x34 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x38 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x3c - Pulse Control Register"]
    pub pulsectrl: PULSECTRL,
    #[doc = "0x40 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x44 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved0: [u8; 12usize],
    #[doc = "0x54 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved1: [u8; 84usize],
    #[doc = "0xac - LEUART Input Register"]
    pub input: INPUT,
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
#[doc = "Clock Control Register"]
pub struct CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "Start Frame Register"]
pub struct STARTFRAME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Frame Register"]
pub mod startframe;
#[doc = "Signal Frame Register"]
pub struct SIGFRAME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signal Frame Register"]
pub mod sigframe;
#[doc = "Receive Buffer Data Extended Register"]
pub struct RXDATAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "Receive Buffer Data Register"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Data Register"]
pub mod rxdata;
#[doc = "Receive Buffer Data Extended Peek Register"]
pub struct RXDATAXP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "Transmit Buffer Data Extended Register"]
pub struct TXDATAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "Transmit Buffer Data Register"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Buffer Data Register"]
pub mod txdata;
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
#[doc = "Interrupt Enable Register"]
pub struct IEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "Pulse Control Register"]
pub struct PULSECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Control Register"]
pub mod pulsectrl;
#[doc = "Freeze Register"]
pub struct FREEZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "LEUART Input Register"]
pub struct INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LEUART Input Register"]
pub mod input;
