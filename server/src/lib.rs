mod channel;
mod channel_manager;
mod chat_service;
mod utils;

use crate::chat_service::ChatService;
use proto::chat::chat_server::ChatServer;
use std::net::SocketAddr;
use tracing::info;

pub async fn run(host: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    info!("server listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(ChatServer::new(ChatService::default()))
        .serve(addr)
        .await
        .map_err(|e| e.into())
}
