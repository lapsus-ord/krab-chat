use crate::channel::Channel;
use proto::chat::chat_server::Chat;
use proto::chat::{ChannelCreateRequest, ChannelResponse, ChannelSelectRequest, Empty, Message};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ChatService {
    _rooms: Arc<Mutex<HashMap<String, Channel>>>,
}

#[tonic::async_trait]
impl Chat for ChatService {
    async fn create_channel(&self, _request: Request<ChannelCreateRequest>) -> Result<Response<ChannelResponse>, Status> {
        unimplemented!()
    }

    async fn join_channel(&self, _request: Request<ChannelSelectRequest>) -> Result<Response<Empty>, Status> {
        unimplemented!()
    }

    async fn leave_channel(&self, _request: Request<ChannelSelectRequest>) -> Result<Response<Empty>, Status> {
        unimplemented!()
    }

    async fn send_message(&self, _request: Request<Message>) -> Result<Response<Empty>, Status> {
        unimplemented!()
    }

    type ReceiveMessagesStream = ReceiverStream<Result<Message, Status>>;

    async fn receive_messages(&self, _request: Request<Message>) -> Result<Response<Self::ReceiveMessagesStream>, Status> {
        unimplemented!()
    }
}
