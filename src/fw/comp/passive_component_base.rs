use serde::{Serialize, Deserialize};

use crate::fw::obj::obj_base::ObjBase;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PassiveComponentBase {
    pub object: ObjBase,
    pub id_base: u32
}

impl PassiveComponentBase {

    pub fn get_id_base(&self) -> u32 {
        self.id_base
    }

    #[cfg(feature = "object_names")]
    pub fn to_string(&self) -> String {
        format!("Comp: {}", self.object.name)
    }

    #[cfg(not(feature = "object_names"))]
    pub fn to_string(&self) -> String {
        format!("PassiveComponentBase")
    }
    
    pub fn new(#[cfg(feature = "object_names")] name: String) -> Self{
        PassiveComponentBase {
            object: ObjBase::new(#[cfg(feature = "object_names")] name),
            id_base: 0
        }
    }

}
