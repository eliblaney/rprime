use rprime::fw::port::port_base;
use rprime::fw::port::input_port_base::InputPortBase;
use rprime::fw::port::output_port_base::OutputPortBase;

#[test]
fn create_port_base() {
    struct MockPort { base: port_base::PortBase, connected: bool };

    impl MockPort {
        fn new() -> Self {
            MockPort { 
                base: port_base::PortBase::new(#[cfg(feature = "object_names")] "MockPort".to_string()), connected: false
            }
        }
    }

    impl port_base::Port for MockPort {
        fn is_connected(&self) -> bool { self.connected }
    }

    let mock = MockPort::new();
    #[cfg(feature = "port_tracing")]
    assert_eq!(mock.base.trace, false);
}

#[test]
fn create_input_port_base() {
    struct MockPort {
        base: InputPortBase,
    };

    impl MockPort {
        fn new() -> Self {
            MockPort { 
                base: InputPortBase::new(#[cfg(feature = "object_names")] "MockInputPort".to_string(), 3333)
            }
        }
    }

    let mock = MockPort::new();
    assert_eq!(mock.base.base.conn.is_none(), true);
}

#[test]
fn create_output_port_base() {
    struct MockPort {
        base: OutputPortBase,
    };

    impl MockPort {
        fn new() -> Self {
            MockPort { 
                base: OutputPortBase::new(#[cfg(feature = "object_names")] "MockOutputPort".to_string(), 3333)
            }
        }
    }

    let mock = MockPort::new();
    assert_eq!(mock.base.base.conn.is_none(), true);
}
