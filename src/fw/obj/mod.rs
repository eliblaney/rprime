pub mod obj_base;
pub mod simple_obj_registry;

use crate::fw::obj::simple_obj_registry::SimpleObjRegistry;

pub fn registry() -> SimpleObjRegistry {
    SimpleObjRegistry { objects: Vec::new() }
}
