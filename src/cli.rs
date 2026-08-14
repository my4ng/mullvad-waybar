use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Path to the Mullvad VPN binary
    #[arg(short, long)]
    pub binary: Option<PathBuf>,
}
