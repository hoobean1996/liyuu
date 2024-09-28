# aarch64


```
.section    __TEXT,__text,regular,pure_instructions
.globl      _main
.p2align    2
_main:
        sub     sp, sp, #16
        mov     w8, w0
        mov     w0, wzr
        str     wzr, [sp, #12]
        str     w8, [sp, #8]
        str     x1, [sp]
        add     sp, sp, #16
        ret
```