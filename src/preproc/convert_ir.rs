use super::PreprocBrainFuck;
use crate::BrainFuck;

macro_rules! preproc_bf_to_pbf {
    ($bfs:expr, $i:expr, $bf_name:expr, $pbf_name:expr) => {{
        let mut count = 1;
        while $i + count < $bfs.len() && $bfs[$i + count] == $bf_name {
            count += 1;
        }
        ($pbf_name(count), count)
    }};
}

/// This function should be used after preprocessing.
pub fn bf_to_pbf(bfs: Vec<BrainFuck>) -> Vec<PreprocBrainFuck> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < bfs.len() {
        match bfs[i] {
            BrainFuck::IncPtr => {
                let (item, count) = preproc_bf_to_pbf!(bfs, i, BrainFuck::IncPtr, PreprocBrainFuck::IncPtr);
                result.push(item);
                i += count;
            }

            BrainFuck::DecPtr => {
                let (item, count) = preproc_bf_to_pbf!(bfs, i, BrainFuck::DecPtr, PreprocBrainFuck::DecPtr);
                result.push(item);
                i += count;
            }

            BrainFuck::IncCell => {
                let (item, count) = preproc_bf_to_pbf!(bfs, i, BrainFuck::IncCell, PreprocBrainFuck::IncCell);
                result.push(item);
                i += count;
            }

            BrainFuck::DecCell => {
                let (item, count) = preproc_bf_to_pbf!(bfs, i, BrainFuck::DecCell, PreprocBrainFuck::DecCell);
                result.push(item);
                i += count;
            }

            BrainFuck::Output => {
                result.push(PreprocBrainFuck::Output);
                i += 1;
            }

            BrainFuck::Input => {
                result.push(PreprocBrainFuck::Input);
                i += 1;
            }

            BrainFuck::LoopBegin => {
                result.push(PreprocBrainFuck::LoopBegin);
                i += 1;
            }

            BrainFuck::LoopEnd => {
                result.push(PreprocBrainFuck::LoopEnd);
                i += 1;
            }
        }
    }

    result
}
