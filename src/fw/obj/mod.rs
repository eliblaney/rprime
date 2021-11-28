pub mod obj_base;
pub mod simple_obj_registry;

use crate::fw::obj::simple_obj_registry::ObjRegistry;
use crate::fw::obj::simple_obj_registry::SimpleObjRegistry;

pub fn registry() -> Box<dyn ObjRegistry> {
    Box::new(SimpleObjRegistry { objects: Vec::new() })
}
