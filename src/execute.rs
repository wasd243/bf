use crate::{Context, PreprocBrainFuck};
use std::io::{Read, Write, stdin, stdout};
use std::slice;

impl Context {
    pub fn new() -> Self {
        let mut cells = vec![0_u8; 30000];
        let ptr = cells.as_mut_ptr();

        Self {
            ptr,
            cells,
            jump_table: Vec::new(),
        }
    }

    /// run the brainfuck code
    pub fn run(&mut self, bfs: Vec<PreprocBrainFuck>) -> anyhow::Result<()> {
        self.ptr = self.cells.as_mut_ptr();
        self.build_jump_table(&bfs)?;

        let mut ip = 0;
        while ip < bfs.len() {
            match bfs[ip] {
                PreprocBrainFuck::IncPtr(x) => unsafe {
                    self.ptr = self.ptr.add(x);
                },

                PreprocBrainFuck::DecPtr(x) => unsafe {
                    self.ptr = self.ptr.sub(x);
                },

                PreprocBrainFuck::IncCell(x) => unsafe {
                    *self.ptr = (*self.ptr).wrapping_add(x as u8);
                },

                PreprocBrainFuck::DecCell(x) => unsafe {
                    *self.ptr = (*self.ptr).wrapping_sub(x as u8);
                },

                PreprocBrainFuck::Output => unsafe {
                    stdout().write_all(slice::from_ref(&*self.ptr))?;
                },

                PreprocBrainFuck::Input => unsafe {
                    // we could not use `std::io::stdin().read_exact()` here
                    let mut byte = [0u8; 1];
                    match stdin().read(&mut byte) {
                        Ok(1) => *self.ptr = byte[0],
                        Ok(0) => *self.ptr = 0,
                        Err(_) => *self.ptr = 0,
                        _ => {}
                    }
                },

                PreprocBrainFuck::LoopBegin => unsafe {
                    if *self.ptr == 0 {
                        ip = self.jump_table[ip];
                    }
                },

                PreprocBrainFuck::LoopEnd => unsafe {
                    if *self.ptr != 0 {
                        ip = self.jump_table[ip];
                    }
                },
            }
            ip += 1;
        }

        Ok(())
    }

    /// build jump table for loop detection
    fn build_jump_table(&mut self, bfs: &[PreprocBrainFuck]) -> anyhow::Result<()> {
        let mut stack = Vec::new();
        let mut jump_points = vec![0; bfs.len()];
        for (i, bf) in bfs.iter().enumerate() {
            match bf {
                PreprocBrainFuck::LoopBegin => stack.push(i),
                PreprocBrainFuck::LoopEnd => {
                    let begin = stack.pop().unwrap_or_default();
                    jump_points[begin] = i;
                    jump_points[i] = begin;
                }
                _ => {}
            }
        }
        self.jump_table = jump_points;
        Ok(())
    }
}
