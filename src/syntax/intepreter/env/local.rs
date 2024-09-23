use std::collections::HashMap;

use crate::syntax::intepreter::value::value::Value;

use super::env::Env;

pub struct Local {
    values: HashMap<String, Value>,
}

impl Local {
    pub fn new() -> Local {
        Local {
            values: HashMap::new(),
        }
    }
}

impl Env for Local {
    fn get(&self, key: &str) -> Value {
        if let Some(r) = self.values.get(key) {
            r.clone()
        } else {
            Value::Null
        }
    }

    fn set(&mut self, key: &str, val: Value) {
        self.values.insert(key.to_owned(), val);
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::intepreter::value::value::Value;

    use super::{Env, Local};

    #[test]
    pub fn test_local() {
        let mut env = Local::new();
        env.set("age", Value::Int(10));
        let r = env.get("age");
        match r {
            Value::Int(v) => assert_eq!(v, 10),
            _ => panic!("it should be a Value::Int(3)"),
        }
    }
}
