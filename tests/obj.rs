use rprime::fw::obj::simple_obj_registry::ObjRegistry;
use rprime::fw::obj;

#[test]
fn create_simple_registry() {
    let registry = obj::registry();
    assert_eq!(registry.objects.len(), 0);
}

#[test]
fn reg_obj() {
    let mut registry = obj::registry();
    assert_eq!(registry.objects.len(), 0);

    struct MockObj;
    impl obj::obj_base::ObjBase for MockObj {
        fn get_obj_name(&self) -> &str {""}
        fn set_obj_name(&self, _name: Option<&str>) {}
        fn to_string(&self) -> &str {""}
        fn new(&self) {}
    }

    let mock = MockObj;
    registry.reg_object(Box::new(mock));
    assert_eq!(registry.objects.len(), 1);
}
