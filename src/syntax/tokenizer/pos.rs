#[derive(Debug, Default, PartialEq, Eq)]
pub struct Pos {
    row: u32,
    col: u8,
}

impl Pos {}

#[cfg(test)]
mod tests {
    use super::Pos;

    #[test]
    pub fn test_default() {
        let p = Pos::default();
        assert_eq!(p.row, 0);
        assert_eq!(p.col, 0);
    }
    #[test]
    pub fn test_eq() {
        let p1 = Pos { row: 1, col: 1 };
        let p2 = Pos { row: 1, col: 1 };
        assert_eq!(p1, p2);
    }
}
