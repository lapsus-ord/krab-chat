use proto::chat::Message;
use std::collections::HashSet;
use tokio::sync::broadcast::{Receiver, Sender};
use tonic::{Code, Status};
use uuid::Uuid;

static CAPACITY: usize = 8;

#[derive(Debug)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub creator: String,
    pub clients: HashSet<String>,
    sender: Sender<Message>,
}

impl Channel {
    pub fn new(channel_name: String, channel_creator: String) -> Self {
        Channel {
            id: Uuid::new_v4(),
            name: channel_name,
            creator: channel_creator,
            clients: HashSet::new(),
            sender: tokio::sync::broadcast::channel(CAPACITY).0,
        }
    }

    pub fn add_client(&mut self, username: String) {
        todo!()
    }

    pub fn remove_client(&mut self, username: String) {
        todo!()
    }

    pub fn send(&self, message: Message) -> Result<(), Status> {
        let username = message.username.clone();

        self.sender.send(message).map_err(|_| {
            let error_message = format!("Failed to send message of user '{}'", username);
            Status::new(Code::Internal, error_message)
        })?;

        Ok(())
    }

    pub fn subscribe(&self) -> Receiver<Message> {
        self.sender.subscribe()
    }
}
