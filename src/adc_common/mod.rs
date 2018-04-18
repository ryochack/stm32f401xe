use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Common status register"]
    pub csr: CSR,
    #[doc = "0x04 - ADC common control register"]
    pub ccr: CCR,
}
#[doc = "ADC Common status register"]
pub struct CSR {
    register: VolatileCell<u32>,
}
#[doc = "ADC Common status register"]
pub mod csr;
#[doc = "ADC common control register"]
pub struct CCR {
    register: VolatileCell<u32>,
}
#[doc = "ADC common control register"]
pub mod ccr;
