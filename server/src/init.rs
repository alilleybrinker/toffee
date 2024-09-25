use anyhow::Result;

pub fn init() -> Result<()> {
    init_tracing();
    init_dotenv()?;
    Ok(())
}

fn init_tracing() {
    tracing_subscriber::fmt().init();
}

fn init_dotenv() -> Result<()> {
    dotenvy::dotenv()?;
    Ok(())
}
