```c
int main() {
  int a = 1;

  int b = a + 3;

  return 0;
}
```

```AST
FunctionDecl
  - Parameters: []
  - ReturnType: int
  - Statements:
    - Definition:
      - Name: ID("a")
      - Type: int
      - Value: 
        - Expr: Int(3)
    - Definition:
      - Name: ID("b)
      - Type: int
      - Value:
        - Expr: BinaryExpr(ID("a"), +, 3);
```

```TAC
main:
  BeginFunc 4;
  a = 1;
  b = a + 3
```

```BB
BB1:
  BeginFunc 4;
  a = 1;
  b = a + 3
```