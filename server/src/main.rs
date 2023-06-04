mod chat_room;

use chat_room::ChatRoom;
use proto::chat::{
    chat_server::{Chat, ChatServer},
    Message,
};
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use tonic::{transport::Server, Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:3000".parse()?;
    let chatroom_1 = ChatRoom::default();

    Server::builder()
        .add_service(ChatServer::new(chatroom_1))
        .serve(addr)
        .await?;

    Ok(())
}
