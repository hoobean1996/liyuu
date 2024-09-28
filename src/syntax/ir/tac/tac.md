# Three-Address Code
- high-level assembly where each operation has at most 3 operands
- use explicit runtime stack for function calls
- use vtables for dynamic dispatch

## Variable Assignment
var = constant
var1 = var2
var1 = var2 op var3
var1 = constant op var2
var1 = var2 op constant
var = constant1 op constant2

```c
/// y = 0 - x;
y = -x;

/// y = -1 * x;
y = -x;

/// _t0 = x + x;
/// _t1 = y;
/// b1 = _t0 < _t1;
b1 = x + x < y;

/// _t0 = x + x;
/// _t1 = y;
/// b2 = _t0 == _t1;
b2 = x + x == y;

/// _t0 = x + x;
/// _t1 = y;
/// b2 = _t0 > _t1;
b2 = x + x > y;

/// _t0 = x < y;
/// _t1 = x == y;
/// b = _t0 || _t1;
b = (x <= y);

/// Compiler assumes condition is more likely to be true,

/// _t0 = x < y;
/// Ifz _t0 Goto _L0;
/// z = x;
/// Goto _L1;
/// _L0:
///   z = y;
/// _L1:
///   z = z * z;
if (x < y) {
  z = x;    
} else {
  z = y;    
}
z = z * z; 

/// _L0:
///   _t0 = x < y;
///   Ifz _t0 Goto _L1;
///   x = x * 2;
///   Goto _L0;
/// _L1:
///   y = x;
while (x < y) {
  x =  x * 2;
}
y = x;
```

## Analogy
```c

/// main:
///   BeginFunc 24;
///   _t0 = x * x;
///   _t1 = y * y;
///   m2  = _t0 + _t1;
/// _L0:
///   _t2 = 5 < m2;
///   Ifz _t2 Goto _L1;
///   m2 = m2 - x;
///   Goto _L0;
/// _L1:
///   EndFunc;
int main(int argc, char **argv) {
  int x, y;
  int m2 = x * x + y * y;

  while (m2 > 5) {
    m2 = m2 - x;
  }
}


/// fn:
///   BeginFunc 16;
///   _t0 = x * y;
///   _t1 = _t0 * z;
///   x = _t1;
///   EndFunc
/// main:
///   BeginFunc 4;
///   _t0 = 137;
///   PushParam _t0;
///   LCall _fn;
///   PopParams 4;
///   EndFunc;
void fn(int z) {
  int x, y;
  x = x * y * z;
}

void main() {
  fn(137);
}
```

## Stack Frame
In x86-64, 1-6 parameters will be passed by registers, 
more parameters will be passed by rbp + ...
```
Param N
Param N-1
...
Param 1
[Locals and Temporaries]
```

