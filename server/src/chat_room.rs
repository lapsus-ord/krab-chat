use proto::chat::{chat_server::Chat, Ack, ConnectRequest, Message};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ChatRoomService {
    name: String,
    creator: String,
    senders: HashMap<String, Sender<Message>>,
}

impl ChatRoomService {
    pub fn new(name: &str, creator: &str) -> Self {
        Self {
            name: name.to_string(),
            creator: creator.to_string(),
            senders: HashMap::def,
        }
    }
}

impl Default for ChatRoomService {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            creator: "default".to_string(),
            senders: HashMap::def,
        }
    }
}

#[tonic::async_trait]
impl Chat for ChatRoomService {
    type ConnectStream = ReceiverStream<Result<Message, Status>>;

    async fn connect(
        &self,
        request: Request<ConnectRequest>,
    ) -> Result<Response<Self::ConnectStream>, Status> {
        todo!()
    }

    async fn send_message(&self, request: Request<Message>) -> Result<Response<Ack>, Status> {
        todo!()
    }
}
