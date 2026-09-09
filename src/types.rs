use std::error::Error;
use std::fmt;

#[derive(Debug, Default)]
pub struct Context {
    pub ptr: *mut u8,
    pub cells: Vec<u8>,
    pub jump_table: Vec<usize>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

/// Display the error, although we only have one error lol
impl fmt::Display for BrainFuckErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loop Mismatch")
    }
}

impl Error for BrainFuckErr {
    /* empty */
}
