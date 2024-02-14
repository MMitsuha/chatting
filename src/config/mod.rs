use std::{env::Args, fmt, net::SocketAddrV4};

use log::warn;

pub struct Config {
    pub address: SocketAddrV4,
}

impl Config {
    const DEFAULT_PORT: u16 = 1234;

    pub fn new(mut args: Args) -> Config {
        args.next();

        let port = args
            .next()
            .unwrap_or_else(|| {
                warn!("port not provided, defaulting to {}", Self::DEFAULT_PORT);
                Self::DEFAULT_PORT.to_string()
            })
            .parse()
            .unwrap_or_else(|e| {
                warn!(
                    "can not parse port, error: {}, defaulting to {}",
                    e,
                    Self::DEFAULT_PORT
                );
                Self::DEFAULT_PORT
            });

        return Config {
            address: SocketAddrV4::new("0.0.0.0".parse().unwrap(), port),
        };
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(listening addr: {})", self.address)
    }
}
