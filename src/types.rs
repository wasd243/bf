#[derive(Debug)]
pub struct Context {
    pub ptr: usize,
    pub cells: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
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
