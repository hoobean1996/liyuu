use std::collections::HashMap;

use super::types::Type;

pub struct TypingEnv {
    symbols: HashMap<String, Type>,
}

impl TypingEnv {
    pub fn new() -> TypingEnv {
        TypingEnv {
            symbols: HashMap::new(),
        }
    }
}
