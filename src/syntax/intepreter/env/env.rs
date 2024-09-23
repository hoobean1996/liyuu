use std::collections::HashMap;

use crate::syntax::intepreter::value::value::Value;

pub trait Env {
    fn get(&self, key: &str) -> Value;
    fn set(&mut self, key: &str, val: Value);
}
