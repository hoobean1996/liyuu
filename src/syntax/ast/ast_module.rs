use super::stmt::stmt::Stmt;

pub struct CompilationUnit {
    pub stmts: Vec<Stmt>,
}

/*
int main(int argc, char **argv)
{
    return 0;
}

CompilationUnit
\_  Stmt::Declare(Declare::DeclareFunction(
        String::from("main"),
        [
            (String::from("argc"), Type::Int), 
            (String::from("argv"), Type::Pointer<Type::Pointer<Type::Char>>)
        ],
        Type::Int,
        [
            Stmt::Return(Box::new(Expr::LiteralExpr(Literal::Int(0)))),
        ]
    ))
*/
