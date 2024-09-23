#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    String(String),
}

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    pub fn test_null() {
        let v1 = Value::Null;
        let v2 = Value::Null;
        assert_eq!(v1, v2);
    }

    #[test]
    pub fn test_int() {
        let expect_value = 3;
        let v = Value::Int(expect_value);
        if let Value::Int(actual_value) = v {
            assert_eq!(actual_value, expect_value);
        }
    }

    pub fn test_float() {
        let expect_value: f64 = 3.0;
        let v = Value::Float(expect_value);
        if let Value::Float(actual_value) = v {
            assert_eq!(actual_value, expect_value);
        }
    }
}
