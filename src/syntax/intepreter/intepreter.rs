use super::env::local::Local;
use super::value::value::Value;
use crate::syntax::ast::ast_module::AstModule;
use crate::syntax::ast::expr::expr::Expr;
use crate::syntax::ast::expr::unary_expr::UnaryExpr;
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

    pub fn inteprete(&mut self, astModule: AstModule) -> Option<Value> {
        Some(Value::Null)
    }

    fn inteprete_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::IntLiteralExpr(i) => Value::Int(i),
            Expr::UnaryExpr(ue) => match ue {
                UnaryExpr::Minus(e) => {
                    let v = self.inteprete_expr(*e);
                    if let Value::Int(pv) = v {
                        Value::Int(pv)
                    } else {
                        panic!("the value must be numberic");
                    }
                }
                UnaryExpr::Plus(e) => {
                    let v = self.inteprete_expr(*e);
                    if let Value::Int(pv) = v {
                        Value::Int(-pv)
                    } else {
                        panic!("the value must be numberic");
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{
        ast::{ast_module::AstModule, expr::expr::Expr},
        intepreter::value::value::Value,
    };

    use super::Intepreter;

    #[test]
    pub fn test_intepreter() {
        let mut intepreter = Intepreter::new();
        if let Some(value) = intepreter.inteprete(AstModule {}) {
            assert_eq!(value, Value::Null);
        }
    }

    #[test]
    pub fn test_intepreter_expr() {
        let mut intepreter = Intepreter::new();
        let expect_value = 3;
        let e = Expr::IntLiteralExpr(expect_value);
        let v = intepreter.inteprete_expr(e);
        if let Value::Int(actual_value) = v {
            assert_eq!(actual_value, expect_value);
        }
    }
}
