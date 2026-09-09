use std::io::{stdin, stdout, Read, Write};
use std::slice;
use crate::{BrainFuck, Context};

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
    pub fn run(&mut self, bfs: Vec<BrainFuck>) -> anyhow::Result<()> {
        self.ptr = self.cells.as_mut_ptr();
        self.build_jump_table(&bfs)?;

        let mut ip = 0;
        while ip < bfs.len() {
            match bfs[ip] {
                BrainFuck::IncPtr => unsafe {
                    self.ptr = self.ptr.add(1);
                },

                BrainFuck::DecPtr => unsafe {
                    self.ptr = self.ptr.sub(1);
                },

                BrainFuck::IncCell => unsafe {
                    *self.ptr = (*self.ptr).wrapping_add(1);
                },

                BrainFuck::DecCell => unsafe {
                    *self.ptr = (*self.ptr).wrapping_sub(1);
                },

                BrainFuck::Output => unsafe {
                    stdout().write_all(slice::from_ref(&*self.ptr))?;
                },

                BrainFuck::Input => unsafe {
                    // we could not use `std::io::stdin().read_exact()` here
                    let mut byte = [0u8; 1];
                    match stdin().read(&mut byte) {
                        Ok(1) => *self.ptr = byte[0],
                        Ok(0) => *self.ptr = 0,
                        Err(_) => *self.ptr = 0,
                        _ => {}
                    }
                },

                BrainFuck::LoopBegin => unsafe {
                    if *self.ptr == 0 {
                        ip = self.jump_table[ip];
                    }
                },

                BrainFuck::LoopEnd => unsafe {
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
    fn build_jump_table(&mut self, bfs: &[BrainFuck]) -> anyhow::Result<()> {
        let mut stack = Vec::new();
        let mut jump_points = vec![0; bfs.len()];
        for (i, bf) in bfs.iter().enumerate() {
            match bf {
                BrainFuck::LoopBegin => stack.push(i),
                BrainFuck::LoopEnd => {
                    let begin = stack.pop().unwrap();
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
