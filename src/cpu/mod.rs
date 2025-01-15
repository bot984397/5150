pub mod addr;

use std::fmt::{self, Debug};
use crate::ext::queue::StaticQueue;
use crate::core::biu::BusInterface;

#[derive(Debug, Clone)]
pub enum CpuStatus {
    Breakpoint,
}

impl fmt::Display for CpuStatus {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuStatus::Breakpoint => write!(f, "Breakpoint hit."),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CpuError {

}

// impl fmt::Display for CpuError {}

pub struct I8088 {
    /* due to the 8088 utilizing a prefetch queue, the PC will
     * point to the next byte to be fetched, not executed. */
    prefetch_queue:StaticQueue<u8, 0x04>,
    pc:u16, /* program counter / instruction pointer */
    le:u32, /* last calculated effective address */

    bus:BusInterface,

    ax:u16,
    bx:u16,
    cx:u16,
    dx:u16,
    si:u16,
    di:u16,
    bp:u16,
    sp:u16,

    ds:u16,
    cs:u16,
    ss:u16,
    es:u16,

    flags:u16,
}

impl I8088 {
    pub fn new() -> Self {

        Self {
            prefetch_queue:StaticQueue::<u8, 0x04>::new(),
            pc:0x00, /* program counter / instruction pointer */
            le:0x00,

            bus:BusInterface::new(),

            ax:0x00,
            bx:0x00,
            cx:0x00,
            dx:0x00,
            si:0x00,
            di:0x00,
            bp:0x00,
            sp:0x00,

            ds:0x00,
            cs:0x00,
            ss:0x00,
            es:0x00,

            flags:0x00,
        }
    }

    pub fn cycle(&mut self) -> Result<(), CpuError> {
        Ok(())
    }
}
