use std::sync::Mutex;

lazy_static! {
    static ref TRACE: Mutex<bool> = Mutex::new(false);
}

pub struct PortBase {
    pub trace: bool, pub override_trace: bool
}

impl PortBase {
    pub fn set_trace(trace: bool) {
        set_conn_trace(trace)
    }

    pub fn override_trace(&mut self, override_trace: bool, trace: bool) {
        self.trace = trace;
        self.override_trace = override_trace;
    }

    pub fn new() -> Self {
        PortBase { trace: false, override_trace: false }
    }
}

pub trait Port {
    fn is_connected(&self) -> bool;

    fn to_string(&self) -> &str;
}

pub fn set_conn_trace(trace: bool) {
    *TRACE.lock().unwrap() = trace;
}
