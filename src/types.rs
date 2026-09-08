#[derive(Debug, Default)]
pub struct Context {
    pub ptr: *mut u8,
    pub cells: Vec<u8>,
    pub jump_table: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub enum BrainFuck {
    IncPtr,
    DecPtr,
    IncCell,
    DecCell,
    Output,
    Input,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug)]
pub enum BrainFuckErr {
    LoopMismatch, // the only error could be existed in brainfuck haha
}
