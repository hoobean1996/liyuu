use super::range::Range;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Attr {
    range: Range,
}

impl Attr {
    pub fn new(row: u32, col: u8) -> Attr {
        Attr {
            range: Range::new(row, col),
        }
    }

    pub fn range(start_row: u32, start_col: u8, end_row: u32, end_col: u8) -> Attr {
        let mut attr = Attr::new(start_row, start_col);
        attr.set_end(end_row, end_col);
        attr
    }

    pub fn set_start(&mut self, row: u32, col: u8) {
        self.range.set_start(row, col);
    }

    pub fn set_end(&mut self, row: u32, col: u8) {
        self.range.set_end(row, col);
    }
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
