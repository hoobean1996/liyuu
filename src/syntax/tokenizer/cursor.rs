use super::pos::Pos;

pub struct Cursor {
    pub index: i32,
    row: u32,
    col: u8,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            index: -1,
            row: 1,
            col: 0,
        }
    }

    pub fn incr_col(&mut self) {
        self.index += 1;
        self.col += 1;
    }

    pub fn incr_row(&mut self) {
        self.index += 1;
        self.row += 1;
        self.col = 0
    }

    pub fn cur_pos(&mut self) -> Pos {
        Pos::new(self.row, self.col)
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::tokenizer::pos::Pos;

    use super::Cursor;

    #[test]
    pub fn test_cursor() {
        let mut cursor = Cursor::new();
        assert_eq!(cursor.cur_pos(), Pos::new(1, 0));
        cursor.incr_col();
        assert_eq!(cursor.cur_pos(), Pos::new(1, 1));
        cursor.incr_row();
        assert_eq!(cursor.cur_pos(), Pos::new(2, 0));
        cursor.incr_col();
        assert_eq!(cursor.cur_pos(), Pos::new(2, 1));
    }
}
