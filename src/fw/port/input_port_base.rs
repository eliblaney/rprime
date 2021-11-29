use serde::{Serialize, Deserialize};

use crate::fw::port::port_base::PortBase;
use crate::fw::port::port_base::Port;
use crate::fw::comp::passive_component_base::PassiveComponentBase;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputPortBase {
    pub base: PortBase,
    pub port_number: u16,
    pub connected: bool,
    pub comp: Option<PassiveComponentBase>
}

impl InputPortBase {
    pub fn set_port_num(&mut self, port_number: u16) {
        self.port_number = port_number;
    }

    pub fn new(#[cfg(feature = "object_names")] name: String, port_number: u16) -> Self {
        InputPortBase {
            base: PortBase::new(#[cfg(feature = "object_names")] name),
            port_number: port_number,
            connected: false,
            comp: None
        }
    }

    #[cfg(feature = "object_names")]
    pub fn to_string(&self) -> String {
        match &self.base.conn {
            Some(conn) => format!("InputPort: {}->{}", self.base.object.name, conn.name),
            None => format!("InputPort: {}->None", self.base.object.name),
        }
    }

    #[cfg(not(feature = "object_names"))]
    pub fn to_string(&self) -> String {
        "InputPort".to_string()
    }
}

impl Port for InputPortBase {
    fn is_connected(&self) -> bool {
        self.connected
    }
}
