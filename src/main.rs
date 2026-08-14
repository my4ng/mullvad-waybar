use std::{
    io::{self, BufRead, BufReader, Write},
    path::Path,
    process::{Command, Stdio},
};

use crate::{cli::Cli, json::Status};
use anyhow::Result;
use clap::Parser;
use log::{debug, error, warn};

mod cli;
mod json;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let cli = Cli::parse();

    let binary_path = cli.binary.as_deref().unwrap_or(Path::new("mullvad"));
    let mut child = Command::new(binary_path)
        .args(["status", "--json", "listen"])
        .stdout(Stdio::piped())
        .spawn()
        .inspect_err(|e| error!("Failed to created mullvad-status child process: {e}"))?;

    let mut reader = BufReader::new(child.stdout.take().expect("handle present"));
    let mut line = String::new();

    loop {
        line.clear();
        reader
            .read_line(&mut line)
            .inspect_err(|e| error!("Failed to read mullvad-status response: {e}"))?;
        match serde_json::from_str::<serde_json::Value>(&line) {
            Ok(value) => {
                debug!(
                    "Received mullvad-status json:\n{}",
                    serde_json::to_string_pretty(&value).expect("json value")
                );

                if let Some(status) = Status::from_status_json(&value) {
                    debug!("Parsed status: {status:#?}");
                    let response = status.into_response_json();
                    let mut stdout = io::stdout();
                    stdout.write_all(response.as_bytes())?;
                    stdout.write_all(b"\n")?;
                    stdout.flush()?;
                }
            }
            Err(e) => {
                warn!("Failed to parse mullvad-status response: {e}");
            }
        };
    }
}
