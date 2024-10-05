use crate::syntax::ir::tac::tac::Instruction;

pub struct BasicBlock {
    id: usize,
    instructions: Vec<Instruction>,
    predecessors: Vec<usize>,
    successors: Vec<usize>,
}

impl BasicBlock {
    pub fn new(id: usize) -> BasicBlock {
        BasicBlock {
            id,
            instructions: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }
}
