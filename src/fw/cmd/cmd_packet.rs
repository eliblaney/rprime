use serde::{Serialize, Deserialize};

use crate::config::fw_config::FwOpcodeType;
// use crate::fw::com::ComPacket;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CmdPacket {
    // pub base: ComPacket,
    pub opcode_type: FwOpcodeType,
    pub arg_buffer: Vec<u8>
}

impl CmdPacket {

    pub fn deserialize(&self) {
        todo!();
    }

    pub fn new(opcode_type: FwOpcodeType) -> Self {
        CmdPacket {
            opcode_type: opcode_type,
            arg_buffer: Vec::new()
        }
    }
}
