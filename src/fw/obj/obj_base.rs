use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjBase {
    #[cfg(feature = "object_names")]
    pub name: String
}

impl ObjBase {
    //  \brief Returns the object's name
    //
    //  This function returns a pointer to the name of the object
    //
    //  \return object name
    #[cfg(feature = "object_names")]
    pub fn get_obj_name(&self) -> &String {
        &self.name
    }

    //  \brief Sets the object name
    //
    //  This function takes the provided string and copies it
    //  to the private buffer containing the name of the object.
    //
    //  \param name the name of the object
    #[cfg(feature = "object_names")]
    pub fn set_obj_name(&mut self, name: String) {
        self.name = name
    }


    //  \brief Returns a string representation of the object
    //
    //  A virtual function defined for all ObjBase types. It is
    //  meant to be overridden by subclasses to return a description
    //  of the object. The default implementation in this class
    //  returns the name of the object.
    //
    //  \param str destination buffer where string description is placed
    //  \param size destination buffer size (including terminator). String should be terminated
    #[cfg(feature = "object_names")]
    pub fn to_string(&self) -> String {
        format!("Obj: {}", &self.name)
    }

    //  \brief Returns a string representation of the object
    //
    //  A virtual function defined for all ObjBase types. It is
    //  meant to be overridden by subclasses to return a description
    //  of the object. The default implementation in this class
    //  returns the name of the object.
    //
    //  \param str destination buffer where string description is placed
    //  \param size destination buffer size (including terminator). String should be terminated
    #[cfg(not(feature = "object_names"))]
    pub fn to_string(&self) -> String {
        "ObjBase".to_string()
    }
    
    //  \brief Object initializer
    //
    //  Initializes the object. For the base class, it calls
    //  the object registry if registered by setObjRegistry()
    //
    #[cfg(feature = "object_names")]
    pub fn new(name: String) -> Self {
        ObjBase {
            name: name
        }
    }

    //  \brief Object initializer
    //
    //  Initializes the object. For the base class, it calls
    //  the object registry if registered by setObjRegistry()
    //
    #[cfg(not(feature = "object_names"))]
    pub fn new() -> Self {
        ObjBase { }
    }
}
