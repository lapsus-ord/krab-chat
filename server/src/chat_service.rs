use crate::channel::Channel;
use proto::chat::chat_server::Chat;
use proto::chat::{ChannelCreateRequest, ChannelResponse, ChannelSelectRequest, Empty, Message};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

type Channels = Arc<Mutex<HashMap<Uuid, Channel>>>;

#[derive(Debug, Default)]
pub struct ChatService {
    channels: Channels,
}

#[tonic::async_trait]
impl Chat for ChatService {
    async fn create_channel(&self, request: Request<ChannelCreateRequest>) -> Result<Response<ChannelResponse>, Status> {
        let request = request.into_inner();
        let channel = Channel::new(request.channel_name, request.creator_username);
        let created_id = channel.id.to_string();

        // Add new channel to state
        let mut channels = self.channels.lock().unwrap();
        println!("Size: {} channels", &channels.len() + 1);
        channels.insert(channel.id, channel);

        Ok(Response::new(ChannelResponse { channel_id: created_id }))
    }

    type SubscribeChannelStream = ReceiverStream<Result<Message, Status>>;

    async fn subscribe_channel(&self, request: Request<ChannelSelectRequest>) -> Result<Response<Self::SubscribeChannelStream>, Status> {
        let request = request.into_inner();
        let channel_id = str_to_uuid(&request.channel_id)?;

        let mut channels = self.channels.lock().unwrap();
        let channel = channels
            .get_mut(&channel_id)
            .ok_or_else(|| Status::new(Code::NotFound, "Channel not found"))?;

        channel.add_client(request.username);

        todo!("subscribe to channel sender");
    }

    async fn unsubscribe_channel(&self, request: Request<ChannelSelectRequest>) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        let channel_id = str_to_uuid(&request.channel_id)?;

        let mut channels = self.channels.lock().unwrap();
        let channel = channels
            .get_mut(&channel_id)
            .ok_or_else(|| Status::new(Code::NotFound, "Channel not found"))?;

        channel.remove_client(request.username);

        todo!("unsubscribe from channel sender");

        Ok(Response::new(Empty::default()))
    }

    async fn send_message(&self, request: Request<Message>) -> Result<Response<Empty>, Status> {
        let message = request.into_inner();
        let channel_id = str_to_uuid(&message.channel_id)?;

        let channels = self.channels.lock().unwrap();
        let channel = channels
            .get(&channel_id)
            .ok_or_else(|| Status::new(Code::NotFound, "Channel not found"))?;

        channel.send(message)?;

        Ok(Response::new(Empty::default()))
    }
}

fn str_to_uuid(input: &str) -> Result<Uuid, Status> {
    Uuid::parse_str(input).map_err(|_| Status::new(Code::InvalidArgument, "Invalid UUID"))
}
