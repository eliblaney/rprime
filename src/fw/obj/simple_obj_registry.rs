use crate::fw::obj::obj_base::ObjBase;

pub trait ObjRegistry {
    fn reg_object(&mut self, obj: Box<dyn ObjBase>);
}

pub struct SimpleObjRegistry {
    pub objects: Vec<Box<dyn ObjBase>>
}

impl ObjRegistry for SimpleObjRegistry {
    fn reg_object(&mut self, obj: Box<dyn ObjBase>) {
        self.objects.push(obj);
    }
}
