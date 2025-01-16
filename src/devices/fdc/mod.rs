use crate::devices::PortMappedDevice;

/// Port numbers
pub const PORT_FDC_DATA_REG                 = 0x3F5;
pub const PORT_FDC_MAIN_STATUS_REG          = 0x3F4;
pub const PORT_DIGITAL_OUTPUT_REG           = 0x3F2;

/// Command mask - only the low 5 bits constitute the command ID.
pub const FDC_COMMAND_MASK:u8               = 0b0001_1111;

/// Four 5-1/4" diskette drives (Two internal, two external)
pub const FDC_MAX_DRIVES:u8                 = 0x04;

/// Digital Output Register
/// ------------------------------------------------------
/// Decoded by the hardware to select one drive if its motor is on.
pub const DOR_DRIVE_SELECT_A:u8             = 0b0000_0000;
pub const DOR_DRIVE_SELECT_B:u8             = 0b0000_0001;
pub const DOR_DRIVE_SELECT_C:u8             = 0b0000_0010;
pub const DOR_DRIVE_SELECT_D:u8             = 0b0000_0011;
/// FDC is held reset when this bit is clear. Must be set by the program to
/// enable the FDC.
pub const DOR_FDC_HELD_RESET:u8             = 0b0000_0100;
/// Allows the FDC interrupt and DMA requests to be gated onto the I/O
/// interface.
pub const DOR_FDC_DMA_ACTIVE:u8             = 0b0000_1000;
/// Control the motors of drives [0,1,2,3] / [A,B,C,D], respectively.
/// If a bit is clear, the associated motor is off, and the drive cannot
/// be selected.
pub const DOR_DRIVE_MOTOR_A:u8              = 0b0001_0000;
pub const DOR_DRIVE_MOTOR_B:u8              = 0b0010_0000;
pub const DOR_DRIVE_MOTOR_C:u8              = 0b0100_0000;
pub const DOR_DRIVE_MOTOR_D:u8              = 0b1000_0000;

/// Main Status Register
/// ------------------------------------------------------
/// Indicates which drives are in 'Seek' mode.
pub const FDC_STATUS_FDD_BUSY_A:u8          = 0b0000_0001;
pub const FDC_STATUS_FDD_BUSY_B:u8          = 0b0000_0010;
pub const FDC_STATUS_FDD_BUSY_C:u8          = 0b0000_0100;
pub const FDC_STATUS_FDD_BUSY_D:u8          = 0b0000_1000;
/// A read or write command is in process.
pub const FDC_STATUS_FDC_BUSY:u8            = 0b0001_0000;
/// FDC is in the non-DMA mode.
pub const FDC_STATUS_NON_DMA_MODE:u8        = 0b0010_0000;
/// Indicates direction of data transfer between FDC and processor.
/// If '1' - transfer from FDC data register to processor. If '0' - transfer
/// from processor to FDC data register.
pub const FDC_STATUS_DATA_INPUT:u8          = 0b0100_0000;
pub const FDC_STATUS_REQUEST_FOR_MASTER:u8  = 0b1000_0000;

#[derive(Copy, Clone, Debug)]
pub enum IoDirection {
    FdcToCpu,
    CpuToFdc,
}

#[derive(Copy, Clone, Debug)]
pub enum OperationPhase {
    None,
    /// FDC receives all information required to perform an operation.
    Command,
    /// FDC performs the operation it was instructed to do.
    Execution,
    /// Status and other information are made available to the processor.
    Result,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Command {
    ReadData,
    ReadDeletedData,
    WriteData,
    WriteDeletedData,
    ReadTrack,
    ReadId,
    FormatTrack,
    ScanEqual,
    ScanLowOrEqual,
    ScanHighOrEqual,
    Recalibrate,
    SenseInterruptStatus,
    Specify,
    SenseDriveStatus,
    Seek,
    Invalid,
}

pub const COMMAND_READ_DATA:u8              = 0b0000_0110;
pub const COMMAND_READ_DELETED_DATA:u8      = 0b0000_1100;
pub const OCMMAND_WRITE_DATA:u8             = 0b0000_0101;
pub const COMMAND_WRITE_DELETED_DATA:u8     = 0b0000_1001;
pub const COMMAND_READ_TRACK:u8             = 0b0000_0010;
pub const COMMAND_READ_ID:u8                = 0b0000_1010;
pub const COMMAND_FORMAT_TRACK:u8           = 0b0000_1101;
pub const COMMAND_SCAN_EQUAL:u8             = 0b0001_0001;
pub const COMMAND_SCAN_LOW_OR_EQUAL:u8      = 0b0001_1001;
pub const COMMAND_SCAN_HIGH_OR_EQUAL:u8     = 0b0001_1101;
pub const COMMAND_RECALIBRATE:u8            = 0b0000_0111;
pub const COMMAND_SENSE_INTERRUPT_STATUS:u8 = 0b0000_1000;
pub const COMMAND_SPECIFY:u8                = 0b0000_0011;
pub const COMMAND_SENSE_DRIVE_STATUS:u8     = 0b0000_0100;
pub const COMMAND_SEEK:u8                   = 0b0000_1111;

pub struct FdcCommand {
    type:Command,
    fn:Option<FdcDispatch>,
    param:u8,
}

impl FdcCommand {
    pub fn new() -> Self {
        Self {
            type:Command::Invalid,
            fn:None,
            param:0,
        }
    }
}

pub struct UPD765 {
    status_register:u8,
    data_register:u8,

    phase:OperationPhase,
    command:FdcCommand,
}

impl PortMappedDevice for UPD765 {
    pub fn write_8(&mut self, port:u16, val:u8) {
        match port {
            PORT_FDC_DATA_REG => {
                self.dispatch_data_register_write(val);
            }
        }
    }

    pub fn read_8(&mut self, port:u16) -> u8 {
        0x00
    }
}

impl Default for UPD765 {
    fn default() -> Self {
        UPD768::new()
    }
}

type FdcDispatch = fn(&mut UPD765);

impl UPD765 {
    pub fn new() -> Self {
        Self {
            status_register:0,
            data_register:0,

            phase:OperationPhase::None,
            command:FdcCommand::new(),
        }
    }

    pub fn calculate_sector_size(val:u8) -> u16 {
        match val {
            0 => 128,
            1 => 256,
            2 => 512,
            3 => 1024,
        }
    }

    fn set_active_command(&mut self, cmd:Command, param:u8, fn:FdcDispatch) {
        self.command.cmd = Command;
        self.command.param = param;
        self.command.fn = Some(fn);
        self.phase = OperationPhase::Command;
    }

    pub fn dispatch_data_register_write(&mut self, val:u8) {
        if self.phase == OperationPhase::Command {
            // fetch single parameter
            self.command.param -= 1;
            if self.command.param == 0 { 
                self.phase = OperationPhase::Execution;
            }
            // take in additional parameters
            return;
        }
        if self.phase == OperationPhase::Execution {
            self.command.fn();
            self.phase = OperationPhase::Result;
            return;
        }
        match val & FDC_COMMAND_MASK {
            COMMAND_READ_DATA => {
                self.set_active_command(Command::ReadData, 
                                        8,
                                        UDP765::fdc_read_data);
            },
            COMMAND_READ_DELETED_DATA => {
                self.set_active_command(Command::ReadDeletedData, 
                                        8, 
                                        UDP765::fdc_read_deleted_data);
            },
            COMMAND_WRITE_DATA => {
                self.set_active_command(Command::WriteData,
                                        8,
                                        UDP765::fdc_write_data);
            },
            COMMAND_WRITE_DELETED_DATA => {
                self.set_active_command(Command::WriteDeletedData,
                                        8,
                                        UDP765::fdc_write_deleted_data);
            },
            COMMAND_READ_TRACK => {
                self.set_active_command(Command::ReadTrack,
                                        8,
                                        UDP765::fdc_read_track);
            },
            COMMAND_READ_ID => {
                self.set_active_command(Command::ReadId,
                                        1,
                                        UDP765::fdc_read_id);
            },
            COMMAND_FORMAT_TRACK => {
                self.set_active_command(Command::FormatTrack,
                                        5,
                                        UDP765::fdc_format_track);
            },
            COMMAND_SCAN_EQUAL => {
                self.set_active_command(Command::ScanEqual,
                                        8,
                                        UDP765::fdc_scan_equal);
            },
            COMMAND_SCAN_LOW_OR_EQUAL => {
                self.set_active_command(Command::ScanLowOrEqual,
                                        8,
                                        UDP765::fdc_scan_low_or_equal);
            },
            COMMAND_SCAN_HIGH_OR_EQUAL => {
                self.set_active_command(Command::ScanHighOrEqual,
                                        8,
                                        UDP765::fdc_scan_high_or_equal);
            },
            COMMAND_RECALIBRATE => {
                self.set_active_command(Command::Recalibrate,
                                        1,
                                        UDP765::fdc_recalibrate);
            },
            COMMAND_SENSE_INTERRUPT_STATUS => {
                self.set_active_command(Command::SenseInterruptStatus,
                                        0,
                                        UDP765::fdc_sense_interrupt_status);
            },
            COMMAND_SPECIFY => {
                self.set_active_command(Command::Specify,
                                        2,
                                        UDP765::fdc_specify);
            },
            COMMAND_SENSE_DRIVE_STATUS => {
                self.set_active_command(Command::SenseDriveStatus,
                                        1,
                                        UDP765::fdc_sense_drive_status);
            },
            COMMAND_SEEK => {
                self.set_active_command(Command::Seek,
                                        1,
                                        UDP765::fdc_seek);
            },
            _ => {
                println!("[FDC] INVALID COMMAND");
            }
        }
    }
}

/// Dispatch functions - called by central command dispatcher
impl UPD765 {
    pub fn fdc_read_data(&mut self) {

    }

    pub fn fdc_
}
