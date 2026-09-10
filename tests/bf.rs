use bf::*;
use std::io::Write;
use std::process::{Command, Stdio};

const BF_HELLO_WORLD: &str = r#"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."#;

#[test]
fn hello_bf() {
    let bfs = parse(BF_HELLO_WORLD.as_bytes(), &mut 0).unwrap();
    let bfs = bf_to_pbf(Preproc::new().preproc(bfs).unwrap());
    assert_eq!(bfs.len(), 43);
    let mut ctx = Context::new();
    ctx.run(bfs).unwrap();
}

#[test]
fn dbfi_hello_world() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_bf"))
        .arg("tests/bfs/interpreter/dbfi.bf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(format!("{BF_HELLO_WORLD}!").as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    println!("[DBFI]Output: {}", String::from_utf8_lossy(&output.stdout));
    // we need to add a `\n` because `dbfi` outputs has a `\n` at the end
    assert_eq!(output.stdout, b"Hello World!\n");
}
