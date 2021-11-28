use crate::fw::obj::obj_base::ObjBase;
use serde::{Serialize, Deserialize};

pub trait ObjRegistry {
    fn reg_object(&mut self, obj: ObjBase);
    fn get_objects(&self) -> &Vec<ObjBase>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleObjRegistry {
    pub objects: Vec<ObjBase>
}

impl ObjRegistry for SimpleObjRegistry {
    fn reg_object(&mut self, obj: ObjBase) {
        self.objects.push(obj);
    }

    fn get_objects(&self) -> &Vec<ObjBase> {
        &self.objects
    }
}
