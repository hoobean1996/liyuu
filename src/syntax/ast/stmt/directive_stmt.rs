#[derive(PartialEq, Debug)]
pub enum Directive {
    Include(String),
    PragmaOnce,
}

#[cfg(test)]
mod tests {
    use super::Directive;

    #[test]
    pub fn test_include() {
        let include = Directive::Include(String::from("std"));
        if let Directive::Include(name) = include {
            assert_eq!(name, String::from("std"));
        }
    }
}
