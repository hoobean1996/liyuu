use super::reg::Register;

#[derive(Debug)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
    Memory(Box<MemoryOperand>),
}

#[derive(Debug)]
pub struct MemoryOperand {
    base: Option<Register>,
    index: Option<Register>,
    scale: Option<u8>,
    displacement: i32,
}
