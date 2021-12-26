use serde::{Serialize, Deserialize};

use crate::fw::com::ComPacketType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComPacket {
    pub packet_type: ComPacketType,
}

impl ComPacket {
    pub fn new(packet_type: ComPacketType) -> Self {
        ComPacket {
            packet_type: packet_type,
        }
    }
}
