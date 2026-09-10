mod app;
use app::build_app;

fn main() -> anyhow::Result<()> {
    build_app()?;
    Ok(())
}
