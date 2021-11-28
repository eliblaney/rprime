use rprime::fw::obj;
use rprime::fw::obj::obj_base::ObjBase;

#[test]
fn create_simple_registry() {
    let registry = obj::registry();
    assert_eq!(registry.get_objects().len(), 0);
}

#[test]
fn reg_obj() {
    let mut registry = obj::registry();
    assert_eq!(registry.get_objects().len(), 0);

    struct MockObj { base: ObjBase };

    let mock = MockObj {
        base: ObjBase::new(
                  #[cfg(feature = "object_names")]
                  "MockObj".to_string()
                  )
    };

    #[cfg(feature = "object_names")]
    assert_eq!(mock.base.name, "MockObj".to_string());

    registry.reg_object(mock.base);
    assert_eq!(registry.get_objects().len(), 1);
}
