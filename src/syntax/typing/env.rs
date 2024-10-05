use std::collections::HashMap;

use super::types::Types;

pub struct TypingEnv {
    symbols: HashMap<String, Types>,
}

impl TypingEnv {
    pub fn new() -> TypingEnv {
        TypingEnv {
            symbols: HashMap::new(),
        }
    }
}
