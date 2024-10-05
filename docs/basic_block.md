# BB
basic block is a straight-line code sequence with no branches in except to the entry 
and no branches out except at the exit

```python
three_address_code = [
  "t1 = a + b",
  "if t1 > 0 goto label1",
  "t2 = a - b",
  "goto label2",
  "label1: t3 = a * b",
  "label2: result = t3",
]
```