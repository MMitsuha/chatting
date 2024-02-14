use std::{fmt, net::SocketAddrV4};

use clap::{arg, command, Parser};

/// The config of server
#[derive(Parser, Debug)]
#[command(version, about, long_about = "A simple chat server written in Rust")]
pub struct CliArgs {
    /// Listen port
    #[arg(default_value_t = 1234)]
    pub port: u16,

    /// Speak speed rate (in minutes)
    #[arg(short, long, default_value_t = 5)]
    pub speed_rate: u16,
}

pub struct Config {
    pub addr: SocketAddrV4,

    pub speed_rate: u16,
}

impl Config {
    /// Create new instance of `Config`
    pub fn new(args: CliArgs) -> Config {
        let port = args.port;
        let speed_rate = args.speed_rate;

        return Config {
            addr: SocketAddrV4::new("0.0.0.0".parse().unwrap(), port),
            speed_rate,
        };
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(listening addr: {}, speed rate: {})",
            self.addr, self.speed_rate
        )
    }
}
