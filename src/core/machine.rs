use std::fmt;

pub struct M5150 {
    mstate:MachineState,
    astate:ActivityState,
}

impl M5150 {
    fn start(&mut self) {
    
    }

    fn stop(&mut self) {

    }
}

#[derive(Copy, Clone, Debug)]
pub enum MachineState {
    On,
    Off,
    Rebooting,
}

impl fmt::Display for MachineState {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            MachineState::On => write!(f, "On"),
            MachineState::Off => write!(f, "Off"),
            MachineState::Rebooting => write!(f, "Rebooting"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ActivityState {
    Paused,
    Running,
    Breakpoint,
    SingleStep,
}

impl ActivityState {
    /// Can we resume from a paused state?
    fn can_resume(&self) -> bool {
        matches!(self, ActivityState::Paused)
    }
    /// Can we pause the running machine?
    fn can_pause(&self) -> bool {
        matches!(self, ActivityState::Running | ActivityState::Breakpoint
            | ActivityState::SingleStep)
    }
    /// Can we resume execution of a running machine?
    fn can_run(&self) -> bool {
        matches!(&self, ActivityState::Breakpoint | ActivityState::SingleStep)
    }
    /// Can we single-step on a running machine?
    fn can_step(&self) -> bool {
        self.can_run()
    }
}

impl fmt::Display for ActivityState {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            ActivityState::Paused => write!(f, "Paused"),
            ActivityState::Running => write!(f, "Running"),
            ActivityState::Breakpoint => write!(f, "Breakpoint"),
            ActivityState::SingleStep => write!(f, "SingleStep"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MachineOperation {
    Pause,
    Resume,
    Run,
    SingleStep,
    StepOver,
    StepInto,
    Reset,
}

impl fmt::Display for MachineOperation {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            MachineOperation::Pause => write!(f, "Pause"),
            MachineOperation::Resume => write!(f, "Resume"),
            MachineOperation::Run => write!(f, "Run"),
            MachineOperation::SingleStep => write!(f, "SingleStep"),
            MachineOperation::StepOver => write!(f, "StepOver"),
            MachineOperation::StepInto => write!(f, "StepInto"),
            MachineOperation::Reset => write!(f, "Reset"),
        }
    }
}
