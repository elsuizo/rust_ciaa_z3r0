#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    pub pa_ctrl: PA_CTRL,
    #[doc = "0x04 - Port Pin Mode Low Register"]
    pub pa_model: PA_MODEL,
    #[doc = "0x08 - Port Pin Mode High Register"]
    pub pa_modeh: PA_MODEH,
    #[doc = "0x0c - Port Data Out Register"]
    pub pa_dout: PA_DOUT,
    #[doc = "0x10 - Port Data Out Set Register"]
    pub pa_doutset: PA_DOUTSET,
    #[doc = "0x14 - Port Data Out Clear Register"]
    pub pa_doutclr: PA_DOUTCLR,
    #[doc = "0x18 - Port Data Out Toggle Register"]
    pub pa_douttgl: PA_DOUTTGL,
    #[doc = "0x1c - Port Data In Register"]
    pub pa_din: PA_DIN,
    #[doc = "0x20 - Port Unlocked Pins Register"]
    pub pa_pinlockn: PA_PINLOCKN,
    #[doc = "0x24 - Port Control Register"]
    pub pb_ctrl: PB_CTRL,
    #[doc = "0x28 - Port Pin Mode Low Register"]
    pub pb_model: PB_MODEL,
    #[doc = "0x2c - Port Pin Mode High Register"]
    pub pb_modeh: PB_MODEH,
    #[doc = "0x30 - Port Data Out Register"]
    pub pb_dout: PB_DOUT,
    #[doc = "0x34 - Port Data Out Set Register"]
    pub pb_doutset: PB_DOUTSET,
    #[doc = "0x38 - Port Data Out Clear Register"]
    pub pb_doutclr: PB_DOUTCLR,
    #[doc = "0x3c - Port Data Out Toggle Register"]
    pub pb_douttgl: PB_DOUTTGL,
    #[doc = "0x40 - Port Data In Register"]
    pub pb_din: PB_DIN,
    #[doc = "0x44 - Port Unlocked Pins Register"]
    pub pb_pinlockn: PB_PINLOCKN,
    #[doc = "0x48 - Port Control Register"]
    pub pc_ctrl: PC_CTRL,
    #[doc = "0x4c - Port Pin Mode Low Register"]
    pub pc_model: PC_MODEL,
    #[doc = "0x50 - Port Pin Mode High Register"]
    pub pc_modeh: PC_MODEH,
    #[doc = "0x54 - Port Data Out Register"]
    pub pc_dout: PC_DOUT,
    #[doc = "0x58 - Port Data Out Set Register"]
    pub pc_doutset: PC_DOUTSET,
    #[doc = "0x5c - Port Data Out Clear Register"]
    pub pc_doutclr: PC_DOUTCLR,
    #[doc = "0x60 - Port Data Out Toggle Register"]
    pub pc_douttgl: PC_DOUTTGL,
    #[doc = "0x64 - Port Data In Register"]
    pub pc_din: PC_DIN,
    #[doc = "0x68 - Port Unlocked Pins Register"]
    pub pc_pinlockn: PC_PINLOCKN,
    #[doc = "0x6c - Port Control Register"]
    pub pd_ctrl: PD_CTRL,
    #[doc = "0x70 - Port Pin Mode Low Register"]
    pub pd_model: PD_MODEL,
    #[doc = "0x74 - Port Pin Mode High Register"]
    pub pd_modeh: PD_MODEH,
    #[doc = "0x78 - Port Data Out Register"]
    pub pd_dout: PD_DOUT,
    #[doc = "0x7c - Port Data Out Set Register"]
    pub pd_doutset: PD_DOUTSET,
    #[doc = "0x80 - Port Data Out Clear Register"]
    pub pd_doutclr: PD_DOUTCLR,
    #[doc = "0x84 - Port Data Out Toggle Register"]
    pub pd_douttgl: PD_DOUTTGL,
    #[doc = "0x88 - Port Data In Register"]
    pub pd_din: PD_DIN,
    #[doc = "0x8c - Port Unlocked Pins Register"]
    pub pd_pinlockn: PD_PINLOCKN,
    #[doc = "0x90 - Port Control Register"]
    pub pe_ctrl: PE_CTRL,
    #[doc = "0x94 - Port Pin Mode Low Register"]
    pub pe_model: PE_MODEL,
    #[doc = "0x98 - Port Pin Mode High Register"]
    pub pe_modeh: PE_MODEH,
    #[doc = "0x9c - Port Data Out Register"]
    pub pe_dout: PE_DOUT,
    #[doc = "0xa0 - Port Data Out Set Register"]
    pub pe_doutset: PE_DOUTSET,
    #[doc = "0xa4 - Port Data Out Clear Register"]
    pub pe_doutclr: PE_DOUTCLR,
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    pub pe_douttgl: PE_DOUTTGL,
    #[doc = "0xac - Port Data In Register"]
    pub pe_din: PE_DIN,
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    pub pe_pinlockn: PE_PINLOCKN,
    #[doc = "0xb4 - Port Control Register"]
    pub pf_ctrl: PF_CTRL,
    #[doc = "0xb8 - Port Pin Mode Low Register"]
    pub pf_model: PF_MODEL,
    #[doc = "0xbc - Port Pin Mode High Register"]
    pub pf_modeh: PF_MODEH,
    #[doc = "0xc0 - Port Data Out Register"]
    pub pf_dout: PF_DOUT,
    #[doc = "0xc4 - Port Data Out Set Register"]
    pub pf_doutset: PF_DOUTSET,
    #[doc = "0xc8 - Port Data Out Clear Register"]
    pub pf_doutclr: PF_DOUTCLR,
    #[doc = "0xcc - Port Data Out Toggle Register"]
    pub pf_douttgl: PF_DOUTTGL,
    #[doc = "0xd0 - Port Data In Register"]
    pub pf_din: PF_DIN,
    #[doc = "0xd4 - Port Unlocked Pins Register"]
    pub pf_pinlockn: PF_PINLOCKN,
    _reserved0: [u8; 40usize],
    #[doc = "0x100 - External Interrupt Port Select Low Register"]
    pub extipsell: EXTIPSELL,
    #[doc = "0x104 - External Interrupt Port Select High Register"]
    pub extipselh: EXTIPSELH,
    #[doc = "0x108 - External Interrupt Rising Edge Trigger Register"]
    pub extirise: EXTIRISE,
    #[doc = "0x10c - External Interrupt Falling Edge Trigger Register"]
    pub extifall: EXTIFALL,
    #[doc = "0x110 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x114 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x118 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x11c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x120 - I/O Routing Register"]
    pub route: ROUTE,
    #[doc = "0x124 - Input Sense Register"]
    pub insense: INSENSE,
    #[doc = "0x128 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x12c - GPIO Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x130 - GPIO Command Register"]
    pub cmd: CMD,
    #[doc = "0x134 - EM4 Wake-up Enable Register"]
    pub em4wuen: EM4WUEN,
    #[doc = "0x138 - EM4 Wake-up Polarity Register"]
    pub em4wupol: EM4WUPOL,
    #[doc = "0x13c - EM4 Wake-up Cause Register"]
    pub em4wucause: EM4WUCAUSE,
}
#[doc = "Port Control Register"]
pub struct PA_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PA_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "Port Pin Mode High Register"]
pub struct PA_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "Port Data Out Register"]
pub struct PA_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "Port Data Out Set Register"]
pub struct PA_DOUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Set Register"]
pub mod pa_doutset;
#[doc = "Port Data Out Clear Register"]
pub struct PA_DOUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Clear Register"]
pub mod pa_doutclr;
#[doc = "Port Data Out Toggle Register"]
pub struct PA_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "Port Data In Register"]
pub struct PA_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data In Register"]
pub mod pa_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PA_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "Port Control Register"]
pub struct PB_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PB_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "Port Pin Mode High Register"]
pub struct PB_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "Port Data Out Register"]
pub struct PB_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "Port Data Out Set Register"]
pub struct PB_DOUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Set Register"]
pub mod pb_doutset;
#[doc = "Port Data Out Clear Register"]
pub struct PB_DOUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Clear Register"]
pub mod pb_doutclr;
#[doc = "Port Data Out Toggle Register"]
pub struct PB_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "Port Data In Register"]
pub struct PB_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data In Register"]
pub mod pb_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PB_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "Port Control Register"]
pub struct PC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PC_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "Port Pin Mode High Register"]
pub struct PC_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "Port Data Out Register"]
pub struct PC_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "Port Data Out Set Register"]
pub struct PC_DOUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Set Register"]
pub mod pc_doutset;
#[doc = "Port Data Out Clear Register"]
pub struct PC_DOUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Clear Register"]
pub mod pc_doutclr;
#[doc = "Port Data Out Toggle Register"]
pub struct PC_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "Port Data In Register"]
pub struct PC_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data In Register"]
pub mod pc_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PC_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "Port Control Register"]
pub struct PD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PD_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "Port Pin Mode High Register"]
pub struct PD_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "Port Data Out Register"]
pub struct PD_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "Port Data Out Set Register"]
pub struct PD_DOUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Set Register"]
pub mod pd_doutset;
#[doc = "Port Data Out Clear Register"]
pub struct PD_DOUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Clear Register"]
pub mod pd_doutclr;
#[doc = "Port Data Out Toggle Register"]
pub struct PD_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "Port Data In Register"]
pub struct PD_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data In Register"]
pub mod pd_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PD_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "Port Control Register"]
pub struct PE_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PE_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "Port Pin Mode High Register"]
pub struct PE_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "Port Data Out Register"]
pub struct PE_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "Port Data Out Set Register"]
pub struct PE_DOUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Set Register"]
pub mod pe_doutset;
#[doc = "Port Data Out Clear Register"]
pub struct PE_DOUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Clear Register"]
pub mod pe_doutclr;
#[doc = "Port Data Out Toggle Register"]
pub struct PE_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "Port Data In Register"]
pub struct PE_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data In Register"]
pub mod pe_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PE_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "Port Control Register"]
pub struct PF_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "Port Pin Mode Low Register"]
pub struct PF_MODEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "Port Pin Mode High Register"]
pub struct PF_MODEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "Port Data Out Register"]
pub struct PF_DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "Port Data Out Set Register"]
pub struct PF_DOUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Set Register"]
pub mod pf_doutset;
#[doc = "Port Data Out Clear Register"]
pub struct PF_DOUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Clear Register"]
pub mod pf_doutclr;
#[doc = "Port Data Out Toggle Register"]
pub struct PF_DOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "Port Data In Register"]
pub struct PF_DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Data In Register"]
pub mod pf_din;
#[doc = "Port Unlocked Pins Register"]
pub struct PF_PINLOCKN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "External Interrupt Port Select Low Register"]
pub struct EXTIPSELL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "External Interrupt Port Select High Register"]
pub struct EXTIPSELH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub struct EXTIRISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub struct EXTIFALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
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
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "Input Sense Register"]
pub struct INSENSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "GPIO Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Control Register"]
pub mod ctrl;
#[doc = "GPIO Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Command Register"]
pub mod cmd;
#[doc = "EM4 Wake-up Enable Register"]
pub struct EM4WUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EM4 Wake-up Enable Register"]
pub mod em4wuen;
#[doc = "EM4 Wake-up Polarity Register"]
pub struct EM4WUPOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EM4 Wake-up Polarity Register"]
pub mod em4wupol;
#[doc = "EM4 Wake-up Cause Register"]
pub struct EM4WUCAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EM4 Wake-up Cause Register"]
pub mod em4wucause;
