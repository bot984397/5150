#[derive(Debug, Clone])
pub enum ExecutionStatus {
    UnknownOpcode(u8),
    Okay,
}

impl fmt::Display for ExecutionStatus {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionStatus::UnknownOpcode => write!(f, "Unknown opcode."),
            ExecutionStatus::Okay => write!(f, "Execution OK"),
        }
    }
}
