mod channel;
mod chat_service;
mod utils;

use crate::chat_service::ChatService;
use proto::chat::chat_server::ChatServer;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::INFO)
        .init();

    let addr = "[::1]:50051".parse()?;
    let chat_service = ChatService::default();

    info!("Server listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(ChatServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
}
