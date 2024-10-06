use super::env::local::Local;
use super::value::value::Value;
use crate::syntax::ast::ast_module::CompilationUnit;
use crate::syntax::ast::expr::binary_expr::Binary;
use crate::syntax::ast::expr::expr::Expr;
use crate::syntax::ast::expr::group_expr::GroupExpr;
use crate::syntax::ast::expr::literal_expr::Literal;
use crate::syntax::ast::expr::trinary_expr::Trinary;
use crate::syntax::ast::expr::unary_expr::Unary;
use crate::syntax::ast::stmt::stmt::Stmt;
use crate::syntax::intepreter::env::env::Env;

pub struct Intepreter {
    env: Box<dyn Env>,
}

impl Intepreter {
    pub fn new() -> Intepreter {
        Intepreter {
            env: Box::new(Local::new()),
        }
    }

    pub fn inteprete(&mut self, astModule: CompilationUnit) -> Option<Value> {
        Some(Value::Null)
    }

    fn intepreter_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Assign(name, e) => {
                if let Expr::LiteralExpr(Literal::ID(name)) = *name {
                    let v = self.inteprete_expr(*e);
                    self.env.set(&name, v)
                } else {
                    panic!("the assign left side must be a Identifier")
                }
            }
            Stmt::Expr(e) => {}
            Stmt::Return(e) => {}
            Stmt::Directive(directive) => {}
            Stmt::Block(stmts) => {}
            Stmt::Declare(declare) => {}
        }
    }

    fn inteprete_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::LiteralExpr(Literal::Bool(b)) => Value::Bool(b),
            Expr::LiteralExpr(Literal::Char(c)) => Value::Int(c.to_digit(10).unwrap() as i64),
            Expr::LiteralExpr(Literal::Int(i)) => Value::Int(i),
            Expr::LiteralExpr(Literal::String(s)) => Value::String(s),
            Expr::LiteralExpr(Literal::ID(name)) => self.env.get(&name),
            Expr::UnaryExpr(ue) => match ue {
                Unary::Minus(e) => {
                    let v = self.inteprete_expr(*e);
                    if let Value::Int(pv) = v {
                        Value::Int(-pv)
                    } else {
                        panic!("the value must be numberic");
                    }
                }
                Unary::Plus(e) => {
                    let v = self.inteprete_expr(*e);
                    if let Value::Int(pv) = v {
                        Value::Int(pv)
                    } else {
                        panic!("the value must be numberic");
                    }
                }
                _ => panic!("not supported yet"),
            },
            Expr::BinaryExpr(be) => match be {
                Binary::Plus(e1, e2) => {
                    let left = self.inteprete_expr(*e1);
                    let right = self.inteprete_expr(*e2);
                    if let (Value::Int(vleft), Value::Int(vright)) = (left, right) {
                        Value::Int(vleft + vright)
                    } else {
                        panic!("the left value and right value must be numberic");
                    }
                }
                Binary::Div(e1, e2) => {
                    let left = self.inteprete_expr(*e1);
                    let right = self.inteprete_expr(*e2);
                    if let (Value::Int(vleft), Value::Int(vright)) = (left, right) {
                        Value::Int(vleft / vright)
                    } else {
                        panic!("the left value and right value must be numberic");
                    }
                }
                Binary::Minus(e1, e2) => {
                    let left = self.inteprete_expr(*e1);
                    let right = self.inteprete_expr(*e2);
                    if let (Value::Int(vleft), Value::Int(vright)) = (left, right) {
                        Value::Int(vleft - vright)
                    } else {
                        panic!("the left value and right value must be numberic");
                    }
                }
                Binary::Mul(e1, e2) => {
                    let left = self.inteprete_expr(*e1);
                    let right = self.inteprete_expr(*e2);
                    if let (Value::Int(vleft), Value::Int(vright)) = (left, right) {
                        Value::Int(vleft * vright)
                    } else {
                        panic!("the left value and right value must be numberic");
                    }
                }
                _ => panic!("not supported yet"),
            },
            Expr::GroupExpr(ge) => match ge {
                GroupExpr::GroupExpr(e) => {
                    let v = self.inteprete_expr(*e);
                    if let Value::Int(pv) = v {
                        Value::Int(pv)
                    } else {
                        panic!("the value must be numberic")
                    }
                }
            },
            Expr::TrinaryExpr(te) => match te {
                Trinary::TrinaryExpr(cond, left, right) => {
                    let cond = self.inteprete_expr(*cond);
                    let left = self.inteprete_expr(*left);
                    let right = self.inteprete_expr(*right);
                    if let (Value::Bool(b), Value::Int(vleft), Value::Int(vright)) =
                        (cond, left, right)
                    {
                        if b {
                            Value::Int(vleft)
                        } else {
                            Value::Int(vright)
                        }
                    } else {
                        panic!("the left value and right value must be numberic")
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{
        ast::{
            ast_module::CompilationUnit,
            expr::{
                binary_expr::Binary, expr::Expr, group_expr::GroupExpr, literal_expr::Literal,
                trinary_expr::Trinary, unary_expr::Unary,
            },
            stmt::stmt::Stmt,
        },
        intepreter::value::value::Value,
    };

    use super::Intepreter;

    #[test]
    pub fn test_intepreter() {
        let mut intepreter = Intepreter::new();
        if let Some(value) = intepreter.inteprete(CompilationUnit { stmts: Vec::new() }) {
            assert_eq!(value, Value::Null);
        }
    }

    #[test]
    pub fn test_intepreter_expr() {
        let mut intepreter = Intepreter::new();

        {
            let expect_value = 3;
            let e = Expr::LiteralExpr(Literal::Int(expect_value));
            let v = intepreter.inteprete_expr(e);
            if let Value::Int(actual_value) = v {
                assert_eq!(actual_value, expect_value);
            }
        }

        {
            let expect_value: i64 = -3;
            let left = Expr::LiteralExpr(Literal::Int(3));
            let e = Expr::UnaryExpr(Unary::Minus(Box::new(left)));
            let v = intepreter.inteprete_expr(e);
            if let Value::Int(actual_value) = v {
                assert_eq!(actual_value, expect_value);
            }
        }

        {
            let expect_value: i64 = 8;
            let left = Expr::LiteralExpr(Literal::Int(2));
            let right = Expr::LiteralExpr(Literal::Int(4));
            let e = Expr::BinaryExpr(Binary::Mul(Box::new(left), Box::new(right)));
            let v = intepreter.inteprete_expr(e);
            if let Value::Int(actual_value) = v {
                assert_eq!(actual_value, expect_value);
            }
        }

        {
            let expect_value = 3;
            let e1 = Expr::LiteralExpr(Literal::Int(expect_value));
            let e = Expr::GroupExpr(GroupExpr::GroupExpr(Box::new(e1)));
            let v = intepreter.inteprete_expr(e);
            if let Value::Int(actual_value) = v {
                assert_eq!(actual_value, expect_value);
            }
        }

        {
            let cond = Expr::LiteralExpr(Literal::Bool(true));
            let e1 = Expr::LiteralExpr(Literal::Int(5));
            let e2 = Expr::LiteralExpr(Literal::Int(3));
            let e = Expr::TrinaryExpr(Trinary::TrinaryExpr(
                Box::new(cond),
                Box::new(e1),
                Box::new(e2),
            ));
            let v = intepreter.inteprete_expr(e);
            if let Value::Int(actual_value) = v {
                assert_eq!(actual_value, 5);
            }
        }

        {
            let cond = Expr::LiteralExpr(Literal::Bool(false));
            let e1 = Expr::LiteralExpr(Literal::Int(5));
            let e2 = Expr::LiteralExpr(Literal::Int(3));
            let e = Expr::TrinaryExpr(Trinary::TrinaryExpr(
                Box::new(cond),
                Box::new(e1),
                Box::new(e2),
            ));
            let v = intepreter.inteprete_expr(e);
            if let Value::Int(actual_value) = v {
                assert_eq!(actual_value, 3);
            }
        }
    }

    #[test]
    pub fn test_inteprete_stmt() {
        let mut intepreter = Intepreter::new();
        {
            let expect_value = 10;
            let s = Stmt::Assign(
                Box::new(Expr::LiteralExpr(Literal::ID(String::from("age")))),
                Box::new(Expr::LiteralExpr(Literal::Int(expect_value))),
            );
            intepreter.intepreter_stmt(s);
            if let Value::Int(actual_value) = intepreter.env.get("age") {
                assert_eq!(actual_value, 10);
            }
        }
    }
}
