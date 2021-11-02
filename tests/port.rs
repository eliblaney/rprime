use rprime::fw::port::port_base;

#[test]
fn create_port() {
    struct MockObj { base: port_base::PortBase, connected: bool };

    impl MockObj {
        fn new() -> Self {
            MockObj { 
                base: port_base::PortBase::new(), connected: false
            }
        }
    }

    impl port_base::Port for MockObj {
        fn is_connected(&self) -> bool { self.connected }
        fn to_string(&self) -> &str { "Mock Port object" }
    }

    let mock = MockObj::new();
    assert_eq!(mock.base.trace, false);
}
