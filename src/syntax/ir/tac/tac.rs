use std::fmt;

use super::{operand::Operand, operator::Operator};

pub enum Instruction {
    Assign1 {
        lhs: Operand,
        rhs: Option<Operand>,
    },
    Assign2 {
        lhs: Operand,
        rhs1: Operand,
        op: Operator,
        rhs2: Operand,
    },
    Ifz {
        condition: Operand,
        target: Operand,
    },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Assign1 {
                lhs: lhs1,
                rhs: lhs2,
            } => match (lhs1, lhs2) {
                (Operand::Variable(var), Some(Operand::Constant(v))) => {
                    write!(f, "{} = {}", &var, v)
                }
                (Operand::Variable(var), Some(Operand::Variable(var2))) => {
                    write!(f, "{} = {}", &var, var2)
                }
                _ => {
                    panic!("The TAC left-side must be Variable");
                }
            },
            Instruction::Assign2 {
                lhs,
                rhs1,
                op,
                rhs2,
            } => match (lhs, rhs1, op, rhs2) {
                (Operand::Variable(var), Operand::Variable(var2), op, Operand::Variable(var3)) => {
                    write!(f, "{} = {} {} {}", var, var2, op, var3)
                }
                (Operand::Variable(var), Operand::Variable(var2), op, Operand::Constant(i)) => {
                    write!(f, "{} = {} {} {}", var, var2, op, i)
                }
                (Operand::Variable(var), Operand::Constant(i), op, Operand::Variable(var2)) => {
                    write!(f, "{} = {} {} {}", var, i, op, var2)
                }
                (Operand::Variable(var), Operand::Constant(i), op, Operand::Constant(j)) => {
                    write!(f, "{} = {} {} {}", var, i, op, j)
                }
                _ => {
                    panic!("the TAC left-side must be Variable");
                }
            },
            Instruction::Ifz { condition, target } => match (condition, target) {
                (Operand::Variable(cond), Operand::Label(t)) => {
                    write!(f, "Ifz {} Goto {}", &cond, t)
                }
                _ => panic!("the TAC Ifz must be Ifz <variable> Goto <label>"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::ir::tac::{operand::Operand, operator::Operator};

    use super::Instruction;

    #[test]
    pub fn test_assign() {
        {
            let tac1 = Instruction::Assign1 {
                lhs: Operand::Variable(String::from("var")),
                rhs: Some(Operand::Constant(5)),
            };
            assert_eq!(tac1.to_string(), "var = 5");
        }
        {
            let tac2 = Instruction::Assign1 {
                lhs: Operand::Variable(String::from("var1")),
                rhs: Some(Operand::Variable(String::from("var2"))),
            };
            assert_eq!(tac2.to_string(), "var1 = var2");
        }

        {
            let tac2 = Instruction::Assign2 {
                lhs: Operand::Variable(String::from("var1")),
                rhs1: Operand::Variable(String::from("var2")),
                op: Operator::Add,
                rhs2: Operand::Variable(String::from("var3")),
            };
            assert_eq!(tac2.to_string(), "var1 = var2 + var3");
        }

        {
            let tac2 = Instruction::Assign2 {
                lhs: Operand::Variable(String::from("var1")),
                rhs1: Operand::Variable(String::from("var2")),
                op: Operator::Add,
                rhs2: Operand::Constant(3),
            };
            assert_eq!(tac2.to_string(), "var1 = var2 + 3");
        }
        {
            let tac2 = Instruction::Assign2 {
                lhs: Operand::Variable(String::from("var1")),
                rhs1: Operand::Constant(3),
                op: Operator::Add,
                rhs2: Operand::Variable(String::from("var2")),
            };
            assert_eq!(tac2.to_string(), "var1 = 3 + var2");
        }
        {
            let tac3 = Instruction::Ifz {
                condition: Operand::Variable(String::from("_t0")),
                target: Operand::Label(String::from("L1")),
            };
            assert_eq!(tac3.to_string(), "Ifz _t0 Goto L1");
        }
    }
}
