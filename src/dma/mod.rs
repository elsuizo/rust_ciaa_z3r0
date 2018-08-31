#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status Registers"]
    pub status: STATUS,
    #[doc = "0x04 - DMA Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x08 - Channel Control Data Base Pointer Register"]
    pub ctrlbase: CTRLBASE,
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer Register"]
    pub altctrlbase: ALTCTRLBASE,
    #[doc = "0x10 - Channel Wait on Request Status Register"]
    pub chwaitstatus: CHWAITSTATUS,
    #[doc = "0x14 - Channel Software Request Register"]
    pub chswreq: CHSWREQ,
    #[doc = "0x18 - Channel Useburst Set Register"]
    pub chusebursts: CHUSEBURSTS,
    #[doc = "0x1c - Channel Useburst Clear Register"]
    pub chuseburstc: CHUSEBURSTC,
    #[doc = "0x20 - Channel Request Mask Set Register"]
    pub chreqmasks: CHREQMASKS,
    #[doc = "0x24 - Channel Request Mask Clear Register"]
    pub chreqmaskc: CHREQMASKC,
    #[doc = "0x28 - Channel Enable Set Register"]
    pub chens: CHENS,
    #[doc = "0x2c - Channel Enable Clear Register"]
    pub chenc: CHENC,
    #[doc = "0x30 - Channel Alternate Set Register"]
    pub chalts: CHALTS,
    #[doc = "0x34 - Channel Alternate Clear Register"]
    pub chaltc: CHALTC,
    #[doc = "0x38 - Channel Priority Set Register"]
    pub chpris: CHPRIS,
    #[doc = "0x3c - Channel Priority Clear Register"]
    pub chpric: CHPRIC,
    _reserved0: [u8; 12usize],
    #[doc = "0x4c - Bus Error Clear Register"]
    pub errorc: ERRORC,
    _reserved1: [u8; 3520usize],
    #[doc = "0xe10 - Channel Request Status"]
    pub chreqstatus: CHREQSTATUS,
    _reserved2: [u8; 4usize],
    #[doc = "0xe18 - Channel Single Request Status"]
    pub chsreqstatus: CHSREQSTATUS,
    _reserved3: [u8; 484usize],
    #[doc = "0x1000 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x1004 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x1008 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x100c - Interrupt Enable register"]
    pub ien: IEN,
    _reserved4: [u8; 240usize],
    #[doc = "0x1100 - Channel Control Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x1104 - Channel Control Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0x1108 - Channel Control Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0x110c - Channel Control Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x1110 - Channel Control Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x1114 - Channel Control Register"]
    pub ch5_ctrl: CH5_CTRL,
}
#[doc = "DMA Status Registers"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Status Registers"]
pub mod status;
#[doc = "DMA Configuration Register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "Channel Control Data Base Pointer Register"]
pub struct CTRLBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Data Base Pointer Register"]
pub mod ctrlbase;
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub struct ALTCTRLBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub mod altctrlbase;
#[doc = "Channel Wait on Request Status Register"]
pub struct CHWAITSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Wait on Request Status Register"]
pub mod chwaitstatus;
#[doc = "Channel Software Request Register"]
pub struct CHSWREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Software Request Register"]
pub mod chswreq;
#[doc = "Channel Useburst Set Register"]
pub struct CHUSEBURSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Useburst Set Register"]
pub mod chusebursts;
#[doc = "Channel Useburst Clear Register"]
pub struct CHUSEBURSTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Useburst Clear Register"]
pub mod chuseburstc;
#[doc = "Channel Request Mask Set Register"]
pub struct CHREQMASKS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Request Mask Set Register"]
pub mod chreqmasks;
#[doc = "Channel Request Mask Clear Register"]
pub struct CHREQMASKC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Request Mask Clear Register"]
pub mod chreqmaskc;
#[doc = "Channel Enable Set Register"]
pub struct CHENS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable Set Register"]
pub mod chens;
#[doc = "Channel Enable Clear Register"]
pub struct CHENC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable Clear Register"]
pub mod chenc;
#[doc = "Channel Alternate Set Register"]
pub struct CHALTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Alternate Set Register"]
pub mod chalts;
#[doc = "Channel Alternate Clear Register"]
pub struct CHALTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Alternate Clear Register"]
pub mod chaltc;
#[doc = "Channel Priority Set Register"]
pub struct CHPRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Priority Set Register"]
pub mod chpris;
#[doc = "Channel Priority Clear Register"]
pub struct CHPRIC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Priority Clear Register"]
pub mod chpric;
#[doc = "Bus Error Clear Register"]
pub struct ERRORC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Error Clear Register"]
pub mod errorc;
#[doc = "Channel Request Status"]
pub struct CHREQSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Request Status"]
pub mod chreqstatus;
#[doc = "Channel Single Request Status"]
pub struct CHSREQSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Single Request Status"]
pub mod chsreqstatus;
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
#[doc = "Interrupt Enable register"]
pub struct IEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable register"]
pub mod ien;
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
