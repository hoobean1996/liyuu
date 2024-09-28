use super::pos::Pos;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Range {
    start: Pos,
    end: Pos,
}

impl Range {
    pub fn new(row: u32, col: u8) -> Range {
        Range {
            start: Pos::new(row, col),
            end: Pos::new(row, col),
        }
    }

    pub fn set_start(&mut self, row: u32, col: u8) {
        self.start = Pos::new(row, col);
    }

    pub fn set_end(&mut self, row: u32, col: u8) {
        self.end = Pos::new(row, col);
    }
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
