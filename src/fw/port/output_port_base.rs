use serde::{Serialize, Deserialize};

use crate::fw::port::port_base::PortBase;
use crate::fw::port::port_base::Port;
use crate::fw::port::input_port_base::InputPortBase;
use crate::fw::comp::passive_component_base::PassiveComponentBase;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OutputPortBase {
    pub base: PortBase,
    pub port_number: u16,
    pub connected: bool,
    pub comp: Option<PassiveComponentBase>,
    pub serial_port: Option<InputPortBase>
}

impl OutputPortBase {
    pub fn set_port_num(&mut self, port_number: u16) {
        self.port_number = port_number;
    }

    pub fn new(#[cfg(feature = "object_names")] name: String, port_number: u16) -> Self {
        OutputPortBase {
            base: PortBase::new(#[cfg(feature = "object_names")] name),
            port_number: port_number,
            connected: false,
            comp: None,
            serial_port: None
        }
    }

    #[cfg(feature = "object_names")]
    pub fn to_string(&self) -> String {
        match &self.base.conn {
            Some(conn) => format!("OutputPort: {} C->({})", self.base.object.name, conn.name),
            None => format!("OutputPort: {} NC->(None)", self.base.object.name),
        }
    }

    #[cfg(not(feature = "object_names"))]
    pub fn to_string(&self) -> String {
        "OutputPort".to_string()
    }
}

impl Port for OutputPortBase {
    fn is_connected(&self) -> bool {
        self.connected
    }
}
