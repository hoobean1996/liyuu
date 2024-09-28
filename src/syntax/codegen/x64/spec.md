# Guide X86-64 
> Each register is 64 bits wide; the lower 32-, 16- and 8-bit portions are selectable by a pseudo-register name.

## Registers
`caller-owned` | `callee-owned`
For example, the registers for the first 6 arguments and return value are callee-owned.

If %rax holds a value, the caller wants to retain, the caller must copy the value to a 
safe location before making a call.

- %rdi
- %rsi
- %rdx
- %rcx
- %r8
- %r9

- %rsp: stack pointer
- %rax: for return value from a function

## Addressing Modes
movl $1, 0x604892   # direct (address is constant value)
movl $1, (%rax)     # indirect (address is in register %rax)
movl $1, -24(%rbp)  # indirect with displacement (address = %rbp - 24)
movl $1, 8(%rsp, %rdi, 4) (address = %rsp + 8 + %rdi * 4)

## Examples
```c
int main(int argc, char **argv) {
  return 0;
}

push rbp
mov rbp, rsp

mov DWORD PTR [rbp-4], edi
mov DWORD PTR [rbp-16], esi

mov rax, 0

pop rbp
ret
```

## Mov and lea
mov src, dst
mov $0, %eax
movb %al, 0x409892
mov 8(%rsp), %rax

lea 0x20(%rsp), %rdi
lea (%rdi, %rdx, 1), %rax

## Branches
cmpl op2, op1 # computes result = op1 - op2, discards result, set condition code

test op2, op1 # computes result = op1 & op2, discards result, set
condition code

jmp target  # unconditonal jump
je target   # jump equal, jz jump zero (ZF = 1)
jne target  # jump not equal, jnz jump non zero (ZF = 0)
js target   # jump signed (SF = 1)
jns target  # jump not signed (SF = 0)
jg target   # jump greater than, (ZF = 0 and SF=OF)
jge target  # jump greater than equal, (SF = OF)