use std::{env, error::Error};

use flexi_logger::Logger;

use chatting::Config;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Logger::try_with_str("debug")?.start()?;

    let config = Config::new(env::args());

    info!("running chat server with config: {}", config);

    chatting::run(config).await
}
