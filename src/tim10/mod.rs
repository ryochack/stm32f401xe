use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    _reserved0: [u8; 8usize],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dier: DIER,
    #[doc = "0x10 - status register"]
    pub sr: SR,
    #[doc = "0x14 - event generation register"]
    pub egr: EGR,
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    pub ccmr1_output: CCMR1_OUTPUT,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - capture/compare enable register"]
    pub ccer: CCER,
    #[doc = "0x24 - counter"]
    pub cnt: CNT,
    #[doc = "0x28 - prescaler"]
    pub psc: PSC,
    #[doc = "0x2c - auto-reload register"]
    pub arr: ARR,
    _reserved2: [u8; 4usize],
    #[doc = "0x34 - capture/compare register 1"]
    pub ccr1: CCR1,
}
#[doc = "control register 1"]
pub struct CR1 {
    register: VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "DMA/Interrupt enable register"]
pub struct DIER {
    register: VolatileCell<u32>,
}
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "status register"]
pub struct SR {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "event generation register"]
pub struct EGR {
    register: VolatileCell<u32>,
}
#[doc = "event generation register"]
pub mod egr;
#[doc = "capture/compare mode register 1 (output mode)"]
pub struct CCMR1_OUTPUT {
    register: VolatileCell<u32>,
}
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "capture/compare mode register 1 (input mode)"]
pub struct CCMR1_INPUT {
    register: VolatileCell<u32>,
}
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "capture/compare enable register"]
pub struct CCER {
    register: VolatileCell<u32>,
}
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "counter"]
pub struct CNT {
    register: VolatileCell<u32>,
}
#[doc = "counter"]
pub mod cnt;
#[doc = "prescaler"]
pub struct PSC {
    register: VolatileCell<u32>,
}
#[doc = "prescaler"]
pub mod psc;
#[doc = "auto-reload register"]
pub struct ARR {
    register: VolatileCell<u32>,
}
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "capture/compare register 1"]
pub struct CCR1 {
    register: VolatileCell<u32>,
}
#[doc = "capture/compare register 1"]
pub mod ccr1;
