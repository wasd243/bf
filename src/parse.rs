use super::*;

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
