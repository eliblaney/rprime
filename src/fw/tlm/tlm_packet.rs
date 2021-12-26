use std::time::SystemTime;
use serde::{Serialize, Deserialize};

use crate::fw::tlm::TlmBuffer;
use crate::fw::com::ComPacketType;
use crate::fw::com::com_packet::ComPacket;
use crate::config::fw_config::FwChanIdType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TlmPacket {
    pub base: ComPacket,
    pub id: FwChanIdType,
    pub time: SystemTime,
    pub buffer: TlmBuffer
}

impl TlmPacket {
    pub fn new(id: FwChanIdType) -> Self {
        TlmPacket {
            base: ComPacket::new(ComPacketType::FwPacketTelem),
            id: id,
            time: SystemTime::now(),
            buffer: TlmBuffer::new()
        }
    }
}
