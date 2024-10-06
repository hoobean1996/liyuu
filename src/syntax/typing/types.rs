#[derive(PartialEq, Debug)]
pub enum Type {
    Int,                                 // int
    Float,                               // float
    Char,                                // char
    Bool,                                // bool
    String,                              // string
    Vector(Box<Type>),                   // vector<T>,
    Function(Vec<Box<Type>>, Box<Type>), // (t1, t2, ..., ) -> t
    Pointer(Box<Type>),                  // int*
    Reference(Box<Type>),                // int&
}
