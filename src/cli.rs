use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Path to the Mullvad VPN binary.
    #[arg(short, long, default_value_os_t = PathBuf::from("mullvad"))]
    pub binary: PathBuf,

    /// What text to display in the bar.
    #[arg(short, long, value_enum, default_value_t = Text::Location)]
    pub text: Text,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, ValueEnum)]
pub enum Text {
    /// Location in CITY,COUNTRY (e.g. SYD,AU)
    Location,
    /// City (e.g. SYD)
    City,
    /// Country (e.g. AU)
    Country,
    /// Hostname (e.g. au-syd-wg-001)
    Hostname,
    /// IPv4/IPv6 addresses
    Ips,
    /// IPv4 address
    Ipv4,
    /// IPv6 address
    Ipv6,
    /// State (offline/disconnected/connected)
    State,
}
