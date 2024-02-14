use std::error::Error;

use flexi_logger::Logger;

use chatting::{CliArgs, Config};
use clap::Parser;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Logger::try_with_str("debug")?.start()?;
    let args = CliArgs::parse();

    let config = Config::new(args);

    info!("running chat server with config: {}", config);

    chatting::run(config).await
}
