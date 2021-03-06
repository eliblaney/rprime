pub mod cmd_packet;

use crate::config::fw_config::FwOpcodeType;

pub type CmdArgBuffer = Vec<u8>;

/* IMPORTANT NOTE:
 * The items below were XML files in F' that were since removed.
 * They are left in here for now, but will likely be removed at a later date.
 */
 
pub enum CommandResponse {
    CommandOk, // Command successfully executed
    CommandInvalidOpcode, // Invalid opcode dispatched
    CommandValidationError, // Command failed validation
    CommandFormatError, // Command failed to deserialize
    CommandExecutionError, // Command had execution error
    CommandBusy // Component busy
}

// Command registration port
pub struct CmdRegPort {
    // Command Op Code
    pub opcode_type: FwOpcodeType
}

// Command port
pub struct CmdResponsePort {
    // Command Op Code
    pub opcode_type: FwOpcodeType,
    // Command Sequence
    pub cmd_seq: u32,
    // The command response argument
    pub response: CommandResponse
}

// Command port
pub struct CmdPort {
    // Command Op Code
    pub opcode_type: FwOpcodeType,
    // Command Sequence
    pub cmd_seq: u32,
    // Buffer containing arguments
    pub args: CmdArgBuffer
}
