#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - System Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x18 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved0: [u8; 245740usize],
    #[doc = "0x3c008 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x3c00c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x3c010 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0x3c014 - Interrupt Register"]
    pub gintsts: GINTSTS,
    #[doc = "0x3c018 - Interrupt Mask Register"]
    pub gintmsk: GINTMSK,
    #[doc = "0x3c01c - Receive Status Debug Read Register"]
    pub grxstsr: GRXSTSR,
    #[doc = "0x3c020 - Receive Status Read and Pop Register"]
    pub grxstsp: GRXSTSP,
    #[doc = "0x3c024 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0x3c028 - Non-periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: GNPTXFSIZ,
    _reserved1: [u8; 48usize],
    #[doc = "0x3c05c - Global DFIFO Configuration Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved2: [u8; 164usize],
    #[doc = "0x3c104 - Device IN Endpoint Transmit FIFO 1 Size Register"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x3c108 - Device IN Endpoint Transmit FIFO 2 Size Register"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x3c10c - Device IN Endpoint Transmit FIFO 3 Size Register"]
    pub dieptxf3: DIEPTXF3,
    _reserved3: [u8; 1776usize],
    #[doc = "0x3c800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0x3c804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0x3c808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved4: [u8; 4usize],
    #[doc = "0x3c810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x3c814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x3c818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0x3c81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved5: [u8; 20usize],
    #[doc = "0x3c834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved6: [u8; 200usize],
    #[doc = "0x3c900 - Device IN Endpoint 0 Control Register"]
    pub diep0ctl: DIEP0CTL,
    _reserved7: [u8; 4usize],
    #[doc = "0x3c908 - Device IN Endpoint 0 Interrupt Register"]
    pub diep0int: DIEP0INT,
    _reserved8: [u8; 4usize],
    #[doc = "0x3c910 - Device IN Endpoint 0 Transfer Size Register"]
    pub diep0tsiz: DIEP0TSIZ,
    #[doc = "0x3c914 - Device IN Endpoint 0 DMA Address Register"]
    pub diep0dmaaddr: DIEP0DMAADDR,
    #[doc = "0x3c918 - Device IN Endpoint 0 Transmit FIFO Status Register"]
    pub diep0txfsts: DIEP0TXFSTS,
    _reserved9: [u8; 4usize],
    #[doc = "0x3c920 - Device IN Endpoint x+1 Control Register"]
    pub diep0_ctl: DIEP0_CTL,
    _reserved10: [u8; 4usize],
    #[doc = "0x3c928 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep0_int: DIEP0_INT,
    _reserved11: [u8; 4usize],
    #[doc = "0x3c930 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep0_tsiz: DIEP0_TSIZ,
    #[doc = "0x3c934 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep0_dmaaddr: DIEP0_DMAADDR,
    #[doc = "0x3c938 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep0_txfsts: DIEP0_TXFSTS,
    _reserved12: [u8; 4usize],
    #[doc = "0x3c940 - Device IN Endpoint x+1 Control Register"]
    pub diep1_ctl: DIEP1_CTL,
    _reserved13: [u8; 4usize],
    #[doc = "0x3c948 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep1_int: DIEP1_INT,
    _reserved14: [u8; 4usize],
    #[doc = "0x3c950 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep1_tsiz: DIEP1_TSIZ,
    #[doc = "0x3c954 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep1_dmaaddr: DIEP1_DMAADDR,
    #[doc = "0x3c958 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep1_txfsts: DIEP1_TXFSTS,
    _reserved15: [u8; 4usize],
    #[doc = "0x3c960 - Device IN Endpoint x+1 Control Register"]
    pub diep2_ctl: DIEP2_CTL,
    _reserved16: [u8; 4usize],
    #[doc = "0x3c968 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep2_int: DIEP2_INT,
    _reserved17: [u8; 4usize],
    #[doc = "0x3c970 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep2_tsiz: DIEP2_TSIZ,
    #[doc = "0x3c974 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep2_dmaaddr: DIEP2_DMAADDR,
    #[doc = "0x3c978 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep2_txfsts: DIEP2_TXFSTS,
    _reserved18: [u8; 388usize],
    #[doc = "0x3cb00 - Device OUT Endpoint 0 Control Register"]
    pub doep0ctl: DOEP0CTL,
    _reserved19: [u8; 4usize],
    #[doc = "0x3cb08 - Device OUT Endpoint 0 Interrupt Register"]
    pub doep0int: DOEP0INT,
    _reserved20: [u8; 4usize],
    #[doc = "0x3cb10 - Device OUT Endpoint 0 Transfer Size Register"]
    pub doep0tsiz: DOEP0TSIZ,
    #[doc = "0x3cb14 - Device OUT Endpoint 0 DMA Address Register"]
    pub doep0dmaaddr: DOEP0DMAADDR,
    _reserved21: [u8; 8usize],
    #[doc = "0x3cb20 - Device OUT Endpoint x+1 Control Register"]
    pub doep0_ctl: DOEP0_CTL,
    _reserved22: [u8; 4usize],
    #[doc = "0x3cb28 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep0_int: DOEP0_INT,
    _reserved23: [u8; 4usize],
    #[doc = "0x3cb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep0_tsiz: DOEP0_TSIZ,
    #[doc = "0x3cb34 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved24: [u8; 8usize],
    #[doc = "0x3cb40 - Device OUT Endpoint x+1 Control Register"]
    pub doep1_ctl: DOEP1_CTL,
    _reserved25: [u8; 4usize],
    #[doc = "0x3cb48 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep1_int: DOEP1_INT,
    _reserved26: [u8; 4usize],
    #[doc = "0x3cb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep1_tsiz: DOEP1_TSIZ,
    #[doc = "0x3cb54 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved27: [u8; 8usize],
    #[doc = "0x3cb60 - Device OUT Endpoint x+1 Control Register"]
    pub doep2_ctl: DOEP2_CTL,
    _reserved28: [u8; 4usize],
    #[doc = "0x3cb68 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep2_int: DOEP2_INT,
    _reserved29: [u8; 4usize],
    #[doc = "0x3cb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep2_tsiz: DOEP2_TSIZ,
    #[doc = "0x3cb74 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved30: [u8; 648usize],
    #[doc = "0x3ce00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "System Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "System Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Status Register"]
pub mod status;
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
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "AHB Configuration Register"]
pub struct GAHBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "USB Configuration Register"]
pub struct GUSBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "Reset Register"]
pub struct GRSTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "Interrupt Register"]
pub struct GINTSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "Interrupt Mask Register"]
pub struct GINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "Receive Status Debug Read Register"]
pub struct GRXSTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "Receive Status Read and Pop Register"]
pub struct GRXSTSP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "Receive FIFO Size Register"]
pub struct GRXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub struct GNPTXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "Global DFIFO Configuration Register"]
pub struct GDFIFOCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register"]
pub struct DIEPTXF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register"]
pub mod dieptxf1;
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register"]
pub struct DIEPTXF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register"]
pub mod dieptxf2;
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register"]
pub struct DIEPTXF3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register"]
pub mod dieptxf3;
#[doc = "Device Configuration Register"]
pub struct DCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "Device Control Register"]
pub struct DCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "Device Status Register"]
pub struct DSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub struct DIEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub struct DOEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "Device All Endpoints Interrupt Register"]
pub struct DAINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub struct DAINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub struct DIEPEMPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "Device IN Endpoint 0 Control Register"]
pub struct DIEP0CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub struct DIEP0INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub struct DIEP0TSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub struct DIEP0DMAADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register"]
pub struct DIEP0TXFSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register"]
pub mod diep0txfsts;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub struct DIEP0_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub struct DIEP0_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub struct DIEP0_TSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub struct DIEP0_DMAADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub struct DIEP0_TXFSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep0_txfsts;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub struct DIEP1_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub struct DIEP1_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub struct DIEP1_TSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub struct DIEP1_DMAADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub struct DIEP1_TXFSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep1_txfsts;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub struct DIEP2_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub struct DIEP2_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub struct DIEP2_TSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub struct DIEP2_DMAADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub struct DIEP2_TXFSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep2_txfsts;
#[doc = "Device OUT Endpoint 0 Control Register"]
pub struct DOEP0CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub struct DOEP0INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub struct DOEP0TSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub struct DOEP0DMAADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub struct DOEP0_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub struct DOEP0_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub struct DOEP0_TSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub struct DOEP0_DMAADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub struct DOEP1_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub struct DOEP1_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub struct DOEP1_TSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub struct DOEP1_DMAADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub struct DOEP2_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub struct DOEP2_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub struct DOEP2_TSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub struct DOEP2_DMAADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "Power and Clock Gating Control Register"]
pub struct PCGCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
