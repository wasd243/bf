use bf::*;
use std::fs;
use std::path::Path;

const BF_SRC1: &str = r#"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."#;

#[test]
fn hello_bf() {
    let bfs = parse(BF_SRC1.as_bytes(), &mut 0).unwrap();
    let mut ctx = Context::new();
    ctx.run(bfs).unwrap();
}

#[derive(Debug, Default)]
struct BrainFuckTest {
    name: String,
    src: String,
}

impl BrainFuckTest {
    fn run(&self) -> anyhow::Result<()> {
        println!("Running {}...", self.name);

        let bfs = parse(self.src.as_bytes(), &mut 0).unwrap();
        let mut ctx = Context::new();

        ctx.run(bfs)?;

        Ok(())
    }
}


#[derive(Debug, Default)]
struct BrainFuckTests {
    tests: Vec<BrainFuckTest>,
}

impl BrainFuckTests {
    fn new() -> Self {
        let test_dir = Path::new("tests/bfs");

        let tests = test_dir
            .read_dir()
            .unwrap()
            .filter_map(|file| {
                let file = file.ok()?;

                if !file.file_type().ok()?.is_file() {
                    return None;
                }

                let name = file.file_name().to_string_lossy().into_owned();
                let src = fs::read_to_string(file.path()).ok()?;

                Some(BrainFuckTest { name, src })
            })
            .collect();

        Self { tests }
    }

    fn run(&self) -> anyhow::Result<()> {
        for test in &self.tests {
            println!("Running {}...", test.name);

            let bfs = parse(test.src.as_bytes(), &mut 0).unwrap();
            let mut ctx = Context::new();

            ctx.run(bfs)?;
        }

        Ok(())
    }
}

#[test]
fn test_life() {
    let bfs = BrainFuckTest {
        name: "life".to_string(),
        src: include_str!("bfs/life.bf").to_string(),
    };
    bfs.run().unwrap();
}

#[test]
fn test_bf() {
    BrainFuckTests::new().run().unwrap();
}
