use super::*;
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
                    let mut byte = [0u8; 1];
                    stdin().read_exact(&mut byte)?;
                    *self.ptr = byte[0];
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

/// Parse the brainfuck src into enum
pub fn parse(src: &[u8], pos: &mut usize) -> Result<Vec<BrainFuck>, BrainFuckErr> {
    let mut bfs: Vec<BrainFuck> = Vec::new();
    while *pos < src.len() {
        let char = src[*pos];
        *pos += 1;
        match char {
            b'>' => bfs.push(BrainFuck::IncPtr),

            b'<' => bfs.push(BrainFuck::DecPtr),

            b'+' => bfs.push(BrainFuck::IncCell),

            b'-' => bfs.push(BrainFuck::DecCell),

            b'.' => bfs.push(BrainFuck::Output),

            b',' => bfs.push(BrainFuck::Input),

            b'[' => bfs.push(BrainFuck::LoopBegin),

            b']' => bfs.push(BrainFuck::LoopEnd),

            _ => {}
        }
    }

    let mut loops = 0;
    for bf in &bfs {
        if *bf == BrainFuck::LoopBegin {
            loops += 1;
        } else if *bf == BrainFuck::LoopEnd {
            if loops == 0 {
                return Err(BrainFuckErr::LoopMismatch);
            }
            loops -= 1;
        }
    }

    if loops != 0 {
        return Err(BrainFuckErr::LoopMismatch);
    }

    Ok(bfs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BF_SRC: &str = ">+[<]>-,[<+>-]<.";

    #[test]
    fn test_parse() {
        let chars = BF_SRC.as_bytes();
        let mut pos = 0;
        let bfs = parse(chars, &mut pos).unwrap();

        println!("{:#?}", bfs);
    }
}
