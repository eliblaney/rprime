pub trait ObjBase {

    //  \brief Returns the object's name
    //
    //  This function returns a pointer to the name of the object
    //
    //  \return object name
    fn get_obj_name(&self) -> &str;

    //  \brief Sets the object name
    //
    //  This function takes the provided string and copies it
    //  to the private buffer containing the name of the object.
    //
    //  \param name the name of the object
    fn set_obj_name(&self, name: Option<&str>);


    //  \brief Returns a string representation of the object
    //
    //  A virtual function defined for all ObjBase types. It is
    //  meant to be overridden by subclasses to return a description
    //  of the object. The default implementation in this class
    //  returns the name of the object.
    //
    //  \param str destination buffer where string description is placed
    //  \param size destination buffer size (including terminator). String should be terminated
    fn to_string(&self) -> &str;


    //  \brief Object initializer
    //
    //  Initializes the object. For the base class, it calls
    //  the object registry if registered by setObjRegistry()
    //
    fn new(&self);
}
