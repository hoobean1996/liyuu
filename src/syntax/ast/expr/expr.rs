use super::unary_expr::UnaryExpr;

pub enum Expr {
    IntLiteralExpr(i64),
    UnaryExpr(UnaryExpr),
}

#[cfg(test)]
mod tests {
    use crate::syntax::ast::expr::unary_expr::UnaryExpr;

    use super::Expr;

    #[test]
    pub fn test_int_literal_expr() {
        let expect_value = 3;
        let e = Expr::IntLiteralExpr(expect_value);
        if let Expr::IntLiteralExpr(actual_value) = e {
            assert_eq!(actual_value, 3);
        }
    }

    #[test]
    pub fn test_unary_expr() {
        {
            let expect_value = 3;
            let e1 = Expr::IntLiteralExpr(expect_value);
            let e2 = Expr::UnaryExpr(UnaryExpr::Plus(Box::new(e1)));
            if let Expr::UnaryExpr(ue) = e2 {
                if let UnaryExpr::Plus(e) = ue {
                    if let Expr::IntLiteralExpr(actual_value) = *e {
                        assert_eq!(actual_value, expect_value);
                    }
                }
            }
        }
        {
            let expect_value = 3;
            let e1 = Expr::IntLiteralExpr(expect_value);
            let e2 = Expr::UnaryExpr(UnaryExpr::Minus(Box::new(e1)));
            if let Expr::UnaryExpr(ue) = e2 {
                if let UnaryExpr::Minus(e) = ue {
                    if let Expr::IntLiteralExpr(actual_value) = *e {
                        assert_eq!(actual_value, expect_value);
                    }
                }
            }
        }
    }
}
