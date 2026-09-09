//! ### Brainfuck Preprocessor
//! This module sanitizes empty loops.

mod empty_loop;

#[derive(Default, Debug)]
pub struct Preproc;

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

    const BF_SRC2: &str = "[++++++]";
    #[test]
    fn unused_loop() {
        test_preproc!(BF_SRC2);
    }

    const BF_SRC3: &str = "[------------=========]";
    #[test]
    fn unused_loop2() {
        test_preproc!(BF_SRC3);
    }

    const BF_SRC4: &str = "[This is a comment---the bf interpreter won't parse normal en comment or other unicode like 中文]";
    #[test]
    fn unused_loop3() {
        test_preproc!(BF_SRC4);
    }
}
