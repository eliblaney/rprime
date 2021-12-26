use serde::{Serialize, Deserialize};

use crate::fw::cmd::CmdArgBuffer;
use crate::fw::com::ComPacketType;
use crate::fw::com::com_packet::ComPacket;
use crate::config::fw_config::FwOpcodeType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CmdPacket {
    pub base: ComPacket,
    pub opcode_type: FwOpcodeType,
    pub arg_buffer: CmdArgBuffer
}

impl CmdPacket {
    pub fn new(opcode_type: FwOpcodeType) -> Self {
        CmdPacket {
            base: ComPacket::new(ComPacketType::FwPacketCommand),
            opcode_type: opcode_type,
            arg_buffer: CmdArgBuffer::new()
        }
    }
}
