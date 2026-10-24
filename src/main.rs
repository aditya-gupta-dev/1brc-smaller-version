use row::generate::{self, Config};

fn main() -> std::io::Result<()> {
    generate::generate_dataset(&Config::default())?;
    Ok(())
}
