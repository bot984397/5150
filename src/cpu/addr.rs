use crate::cpu::I8088;

#[derive(Debug, Copy, Clone)]
pub enum Segment {
    CS,
    DS,
    SS,
    ES,
}

impl I8088 {
    // Calculates a 20-bit physical address from a 16-bit segment value and a
    // 16-bit offset. Last calculated address is stored for future use.
    pub fn calculate_physical_address(&mut self, s:Segment, o:u16) -> u32 {
        self.le = (((match s {
            Segment::CS => self.cs,
            Segment::DS => self.ds,
            Segment::SS => self.ss,
            Segment::ES => self.es,
        } as u32) << 4) + o as u32) & 0xFFFFFu32;
        self.le
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder_core() {
        let mut cpu = I8088::new();
        assert_eq!(cpu.calculate_physical_address(Segment::CS, 0xFF), 0xFF);
        assert_ne!(cpu.calculate_physical_address(Segment::CS, 0xFE), 0xFF);
    }
}
