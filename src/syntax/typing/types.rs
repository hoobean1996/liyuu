#[derive(PartialEq, Debug)]
pub enum Types {
    Int,                                   // int
    Float,                                 // float
    Bool,                                  // bool
    String,                                // string
    Vector(Box<Types>),                    // vector<T>,
    Function(Vec<Box<Types>>, Box<Types>), // (t1, t2, ..., ) -> t
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_types() {}
}
