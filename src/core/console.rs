use crate::core::machine::M5150;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ConCommand {
    name:String,
    desc:String,
    args:u8,
    func:fn(&mut M5150, Vec<String>) -> Result<String, String>,
}

/// Emulator console user interface
pub struct Console {
    cmds:HashMap<ConCommand>,
}
