use super::range::Range;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Attr {
    range: Range,
}

#[cfg(test)]
mod tests {
    use super::Range;

    use super::Attr;

    #[test]
    pub fn test_default() {
        let attr = Attr::default();
        assert_eq!(attr.range, Range::default());
    }
}
