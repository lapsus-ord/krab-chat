use proto::chat::{chat_server::Chat, Ack, ConnectRequest, Message};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ChatRoom {
    name: String,
    creator: String,
    senders: HashMap<String, Sender<Message>>,
}

//#[tonic::async_trait]
impl ChatRoom {
    pub fn new(name: &str, creator: &str) -> Self {
        Self {
            name: name.to_string(),
            creator: creator.to_string(),
            senders: HashMap::def,
        }
    }
}

impl Default for ChatRoom {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            creator: "default".to_string(),
            senders: HashMap::def,
        }
    }
}
