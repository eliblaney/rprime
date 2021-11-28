#[cfg(feature = "port_tracing")] 
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

use crate::fw::obj::obj_base::ObjBase;

#[cfg(feature = "port_tracing")] 
lazy_static! {
    static ref TRACE: Mutex<bool> = Mutex::new(false);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortBase {
    pub object: ObjBase,
    #[cfg(feature = "port_tracing")] 
    pub trace: bool,
    #[cfg(feature = "port_tracing")] 
    pub override_trace: bool,
    pub conn: Option<ObjBase>
}

impl PortBase {
    #[cfg(feature = "port_tracing")] 
    pub fn set_trace(trace: bool) {
        set_conn_trace(trace)
    }

    #[cfg(feature = "port_tracing")] 
    pub fn override_trace(&mut self, override_trace: bool, trace: bool) {
        self.trace = trace;
        self.override_trace = override_trace;
    }

    pub fn new(#[cfg(feature = "object_names")] name: String) -> Self {
        PortBase { 
            object: ObjBase::new(#[cfg(feature = "object_names")] name),
            #[cfg(feature = "port_tracing")] 
            trace: false,
            #[cfg(feature = "port_tracing")] 
            override_trace: false,
            conn: None
        }
    }

    #[cfg(feature = "object_names")]
    pub fn to_string(&self) -> String {
        format!("Port: {}", &self.object.name)
    }

    #[cfg(not(feature = "object_names"))]
    pub fn to_string(&self) -> String {
        "Port".to_string()
    }
}

pub trait Port {
    fn is_connected(&self) -> bool;
}

#[cfg(feature = "port_tracing")] 
pub fn set_conn_trace(trace: bool) {
    *TRACE.lock().unwrap() = trace;
}
