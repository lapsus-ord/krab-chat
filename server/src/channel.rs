use crate::utils::CHANNEL_SIZE;
use proto::chat::Message;
use std::collections::HashMap;
use tokio::sync::broadcast;
use tonic::Status;
use tracing::warn;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub creator: String,
    pub users: HashMap<Uuid, String>,
    pub sender: broadcast::Sender<Result<Message, Status>>,
}

impl Channel {
    /// Create a new channel, but doesn't add the channel's creator in the user list
    pub fn new(channel_name: &str, channel_creator: &str) -> Self {
        let (tx, _) = broadcast::channel::<Result<Message, Status>>(CHANNEL_SIZE);
        Self {
            id: Uuid::new_v4(),
            name: channel_name.to_string(),
            creator: channel_creator.to_string(),
            users: HashMap::new(),
            sender: tx,
        }
    }

    /// Add user to the channel and send back its associated uuid (= user id in the channel)
    pub fn add_user(&mut self, username: &str) -> Uuid {
        let user_uuid = Uuid::new_v4();
        self.users.insert(user_uuid, username.to_string());

        user_uuid
    }

    pub fn remove_user(&mut self, uuid: &Uuid) -> Option<String> {
        self.users.remove(uuid)
    }

    /// Broadcast new message to all subscribed users of the channel
    pub fn broadcast(&self, new_message: Message) {
        if let Err(e) = self.sender.send(Ok(new_message)) {
            warn!("could not broadcast the message {:?}", e);
        }
    }
}

impl From<Channel> for proto::chat::Channel {
    fn from(value: Channel) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            creator: value.creator,
            user_count: value.users.len() as i32,
        }
    }
}
