//! # chatting
//!
//! A simple chat server written in Rust

pub mod config;
pub mod utils;

pub use config::{CliArgs, Config};

use std::{collections::HashMap, error::Error, net::SocketAddr, sync::Arc};

use futures::{SinkExt, StreamExt};
use log::{error, info, warn};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, RwLock},
};
use tokio_util::codec::{Framed, LinesCodec};

type Tx = mpsc::UnboundedSender<String>;

struct Server {
    clients: Arc<RwLock<HashMap<SocketAddr, Tx>>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn run(&mut self, config: &config::Config) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(config.addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;
            let clients = self.clients.clone();
            let speed_rate = config.speed_rate;

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(clients, stream, addr, speed_rate).await {
                    error!("client {} occurred error, error: {}", addr, e);
                }
            });
        }
    }

    async fn handle_connection(
        clients: Arc<RwLock<HashMap<SocketAddr, Tx>>>,
        stream: TcpStream,
        addr: SocketAddr,
        rate_limit: u16,
    ) -> Result<(), Box<dyn Error>> {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let mut frame = Framed::new(stream, LinesCodec::new());

        Self::server_send("Welcome to Mitsuha's chat room.", &mut frame).await?;

        let name = Self::get_username(&mut frame).await?;

        if Self::captcha(&mut frame).await? == false {
            return Err("Bot detected")?;
        }

        Self::broadcast(
            &clients,
            &addr,
            &name,
            &format!("{} joined chat room.", name),
            true,
        )
        .await?;

        clients.write().await.insert(addr, tx);

        // Initialize speed counter
        let counter = utils::Counter::new(rate_limit);

        loop {
            tokio::select! {
                Some(message) = rx.recv() => {
                    frame.send(message).await?;
                }

                result = frame.next() => match result {
                    Some(Ok(message)) => {
                        if counter.check().await {
                            Self::broadcast(&clients, &addr, &name, &message,false).await?;
                        } else {
                            Self::server_send("Oops, you have reached rate limit, please retry after 1 minute.",&mut frame).await?;
                        }

                        counter.add().await;
                    }

                    _ => break,
                }
            }
        }

        Self::broadcast(
            &clients,
            &addr,
            &name,
            &format!("{} left chat room.", name),
            true,
        )
        .await?;
        clients.write().await.remove(&addr);
        Ok(())
    }

    async fn get_username(
        frame: &mut Framed<TcpStream, LinesCodec>,
    ) -> Result<String, Box<dyn Error>> {
        Self::server_send("Please enter your name: ", frame).await?;

        let name = match frame.next().await {
            Some(Ok(n)) => n,
            _ => {
                return Err("Invalid name")?;
            }
        };

        frame.send("\n").await?;

        Ok(name)
    }

    async fn captcha(frame: &mut Framed<TcpStream, LinesCodec>) -> Result<bool, Box<dyn Error>> {
        let param1: u8 = rand::random();
        let param2: u8 = rand::random();
        let answer = param1 as u16 + param2 as u16;
        let captcha = format!("{} + {} = ?", param1, param2);

        Self::server_send(&format!("Please solve the captcha: {}", captcha), frame).await?;

        let input: u16 = match frame.next().await {
            Some(Ok(n)) => n.parse()?,
            _ => {
                return Err("Invalid name")?;
            }
        };

        let ret = input == answer;

        if ret {
            Self::server_send("Correct captcha, welcome!", frame).await?;
        } else {
            Self::server_send("WRONG CAPTCHA, DISCONNECTED!", frame).await?;
        }

        frame.send("\n").await?;

        Ok(ret)
    }

    async fn broadcast(
        clients: &Arc<RwLock<HashMap<SocketAddr, Tx>>>,
        sender: &SocketAddr,
        name: &str,
        message: &str,
        is_server: bool,
    ) -> Result<(), Box<dyn Error>> {
        let message = match is_server {
            true => format!("[SERVER] {}", message),
            false => format!("({}) {}", name, message),
        };

        info!("{}: {}", sender, message);

        for (addr, tx) in clients.read().await.iter() {
            if sender == addr {
                continue;
            }

            let message = message.clone();
            if let Err(e) = tx.send(message) {
                warn!("error sending to {}, error: {}", addr, e);
            }
        }

        Ok(())
    }

    async fn server_send(
        message: &str,
        frame: &mut Framed<TcpStream, LinesCodec>,
    ) -> Result<(), Box<dyn Error>> {
        frame.send(format!("[SERVER] {}", message)).await?;
        Ok(())
    }
}

pub async fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();

    server.run(&config).await
}

#[cfg(test)]
mod tests {}
