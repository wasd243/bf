//! ### Brainfuck Preprocessor
//! This module sanitizes empty loops, convert `BrainFuck` to `PreprocBrainFuck`.

mod convert_ir;
mod empty_loop;

pub use convert_ir::bf_to_pbf;

#[derive(Default, Debug)]
pub struct Preproc;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PreprocBrainFuck {
    IncPtr(usize),
    DecPtr(usize),
    IncCell(usize),
    DecCell(usize),
    Output,
    Input,
    LoopBegin,
    LoopEnd,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    macro_rules! test_preproc {
        ($bf_src: expr) => {
            let bfs = parse($bf_src.as_bytes(), &mut 0).unwrap();
            let bfs = Preproc::new().preproc(bfs).unwrap();
            println!("{:?}", bfs);
            assert_eq!(bfs.len(), 0);
        };
    }

    const BF_SRC: &str = "[++++++------]";
    #[test]
    fn empty_loop() {
        test_preproc!(BF_SRC);
    }

    const BF_SRC2: &str = "[This is a comment---the bf interpreter won't parse normal en +++ comment or other unicode like 中文 の emoji like 😀 or other thing like \n]";
    #[test]
    fn unused_loop3() {
        test_preproc!(BF_SRC2);
    }

    const BF_SRC3: &str = "[------++>+]";
    #[test]
    fn output_converted_or() {
        let bfs = parse(BF_SRC3.as_bytes(), &mut 0).unwrap();
        let bfs = bf_to_pbf(Preproc::new().preproc(bfs).unwrap());
        println!("{:?}", bfs);
        assert_eq!(bfs.len(), 6);
    }
}
