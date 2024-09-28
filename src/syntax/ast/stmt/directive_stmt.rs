pub enum DirectiveStmt {
    DirectiveIncludeStmt(String),
    DirectivePragmaOnce,
}

#[cfg(test)]
mod tests {
    use super::DirectiveStmt;

    #[test]
    pub fn test_include() {
        let include = DirectiveStmt::DirectiveIncludeStmt(String::from("std"));
        if let DirectiveStmt::DirectiveIncludeStmt(name) = include {
            assert_eq!(name, String::from("std"));
        }
    }
}
