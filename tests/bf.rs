use bf::*;

const BF_SRC1: &str = r#"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."#;

#[test]
fn hello_bf() {
    let bfs = parse(BF_SRC1.as_bytes(), &mut 0).unwrap();
    let mut ctx = Context::new();
    ctx.run(bfs).unwrap();
}
