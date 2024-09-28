use super::operand::Operand;

#[derive(Debug)]
pub enum Instruction {
    Mov { dest: Operand, source: Operand },
    Push { operand: Operand },
    Pop { operand: Operand },

    Add { dest: Operand, source: Operand },
    Sub { dest: Operand, source: Operand },
    Imul { dest: Operand, source: Operand },
    Idiv { divisor: Operand },
    Inc { operand: Operand },
    Dec { operand: Operand },

    And { dest: Operand, source: Operand },
    Or { dest: Operand, source: Operand },
    Xor { dest: Operand, source: Operand },
    Not { operand: Operand },

    Shl { dest: Operand, count: Operand },
    Shr { dest: Operand, count: Operand },
    Sar { dest: Operand, count: Operand },

    Jmp { target: Operand },
    Je { target: Operand },
    Jne { target: Operand },
    Jl { target: Operand },
    Jle { target: Operand },
    Jg { target: Operand },
    Jge { target: Operand },
    Call { target: Operand },
    Ret,

    Cmp { left: Operand, right: Operand },
    Test { left: Operand, right: Operand },

    Enter { stack_space: u16, nesting_level: u8 },
    Leave,

    Syscall,
}
