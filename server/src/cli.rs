#[derive(Debug, clap::Parser)]
pub struct Args {
    /// The port to serve the app on.
    #[clap(short, long)]
    pub port: u16,
}
