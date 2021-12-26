pub mod com_packet;

use serde::{Serialize, Deserialize};

pub type ComBuffer = Vec<u8>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ComPacketType {
    FwPacketCommand, // Command packet type - incoming
    FwPacketTelem, // Telemetry packet type - outgoing
    FwPacketLog, // Log type - outgoing
    FwPacketFile, // File type - incoming and outgoing
    FwPacketPacketizedTlm, // Packetized telemetry packet type
    FwPacketIdle, // Idle packet
    FwPacketUnknown // Unknown packet
}