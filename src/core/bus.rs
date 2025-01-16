use std::fmt::{self, Debug};
use crate::ext::prim::u20;

#[derive(Debug, Clone)]
pub enum BusMemoryError {
    OutOfBounds,
}

impl fmt::Display for BusMemoryError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            BusMemoryError::OutOfBounds => write!(f, "Out of bounds."),
        }
    }
}

pub struct BusInterface {
    ram:[u8; 1024 * 1024],
    address_latch:u20,
}

impl Default for BusInterface {
    fn default() -> Self {
        BusInterface::new()
    }
}

impl BusInterface {
    pub fn new() -> Self {
        Self {
            ram:[0x00; 1024 * 1024],
            address_latch:u20::new(0x00),
            ..BusInterface::default()
        }
    }

    pub fn fetch_8(&self) -> u8 {
        0x00
    }

    pub fn fetch_16(&self) -> u16 {
        0x00
    }

    pub fn read_8(&mut self, addr:usize) -> Result<u8, BusMemoryError> {
        //if addr > self.ram.len() { return Err(OutOfBounds) }
        Ok(1)
    }
}
