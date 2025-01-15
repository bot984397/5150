use std::fmt::{self, Debug, Display, Formatter};
use crate::cpu::mnemonic::Mnemonic;

#[derive(Debug, Clone)]
pub enum DecodeError {
    UnknownOpcode(u8),
    UnimplementedOpcode(u8),

}

pub struct Instruction {
    m:Mnemonic,
}

impl Error for DecodeError {}

impl Display for DecodeError {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        match self {
            DecodeError::UnknownOpcode(op) => {
                write!(f, "Unknown opcode {:#2X}", op)
            },
            DecodeError::UnimplementedOpcode(op) => {
                write!(f, "Unimplemented opcode {:#2X}", op)
            },
        }
    }
}
