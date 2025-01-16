use crate::cpu::I8088;

#[derive(Copy, Clone, Debug)]
pub enum TState {
    None,
    TS,
    T0,
    T1,
    T2,
    T3,
    T4,
}

impl I8088 {
    /// Executes one CPU cycle, accurately emulating the transfer between and
    /// execution of T-states.
    pub fn cycle(&mut self) {

    }
}
