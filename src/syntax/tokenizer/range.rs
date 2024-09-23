use super::pos::Pos;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Range {
    start: Pos,
    end: Pos,
}

#[cfg(test)]
mod tests {
    use crate::syntax::tokenizer::pos::Pos;

    use super::Range;

    #[test]
    pub fn test_default() {
        let r = Range::default();
        assert_eq!(r.start, Pos::default());
        assert_eq!(r.end, Pos::default());
    }
}
