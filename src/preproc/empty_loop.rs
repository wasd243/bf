use super::Preproc;
use crate::types::BrainFuck;

use anyhow::Result;

impl Preproc {
    pub fn new() -> Self {
        Self
    }

    /// preproc the brainfuck code
    pub fn preproc(&mut self, bfs: Vec<BrainFuck>) -> Result<Vec<BrainFuck>> {
        let mut result = Vec::new();
        let mut current_loop = Vec::new();
        let mut in_loop = false;

        for bf in bfs {
            if bf == BrainFuck::LoopBegin {
                in_loop = true;
                current_loop.clear();
                current_loop.push(bf);
                continue;
            }

            if in_loop {
                current_loop.push(bf);

                if bf == BrainFuck::LoopEnd {
                    // excluding the loop beginning and ending
                    let loop_content = current_loop.clone();
                    if !check_loop(loop_content[1..loop_content.len() - 1].to_vec())
                        && !clean_empty_loop(&loop_content)
                        && !check_loop_has_same_inc_or_sub(&loop_content)
                    {
                        result.extend(current_loop.clone());
                    }

                    in_loop = false;
                    current_loop.clear();
                }

                continue;
            }

            result.push(bf);
        }

        Ok(result)
    }
}

fn check_loop(bf_loop: Vec<BrainFuck>) -> bool {
    // check the unused loops like
    // ```brainfuck
    // [++--]
    // [+++----+]
    // ```
    let mut increment_count = 0;
    let mut decrement_count = 0;
    for bf in bf_loop {
        match bf {
            BrainFuck::IncCell => increment_count += 1,
            BrainFuck::DecCell => decrement_count += 1,
            _ => return false,
        }
    }

    increment_count == decrement_count
}

fn check_loop_has_same_inc_or_sub(bf_loop: &Vec<BrainFuck>) -> bool {
    // check the:
    // ```brainfuck
    // [++++++]
    // [------]
    // ```
    matches!(bf_loop.as_slice(),
        [BrainFuck::LoopBegin, middle @ .., BrainFuck::LoopEnd]
        if middle.iter().all(|bf| matches!(bf, BrainFuck::IncCell) || matches!(bf, BrainFuck::DecCell)))
}

fn clean_empty_loop(bf_loop: &Vec<BrainFuck>) -> bool {
    matches!(bf_loop.as_slice(), [BrainFuck::LoopBegin, BrainFuck::LoopEnd])
}
