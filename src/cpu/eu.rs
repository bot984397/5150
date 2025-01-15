use crate::cpu::{I8088, CpuStatus, CpuError};

impl I8088 {
    // Fetch, decode and execute one single instruction.
    pub fn advance(&mut self) -> Result<CpuStatus, CpuError> {
        // PC points to the next instruction to be fetched, not the next one
        // to be executed. [adjust_pc] calculates the real IP.
        let ip_real:u16 = self.adjust_pc();
        if (self.is_breakpoint()) {
            return Ok(CpuStatus::Breakpoint); 
        }

        // TODO: check RET / IRET


    }
}
