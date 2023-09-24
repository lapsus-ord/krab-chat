use crate::channel::Channel;
use crate::channel_manager::ChannelManager;
use crate::utils::{str_to_uuid, ResponseStream};
use proto::chat::chat_server::Chat;
use proto::chat::{
    ChannelCreateRequest, ChannelCreateResponse, ChannelListResponse, Empty, JoinChannelRequest,
    JoinChannelResponse, LeaveChannelRequest, Message, SubscribeChannelRequest,
};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ChatService {
    channel_manager: ChannelManager,
}

#[tonic::async_trait]
impl Chat for ChatService {
    async fn create_channel(
        &self,
        request: Request<ChannelCreateRequest>,
    ) -> Result<Response<ChannelCreateResponse>, Status> {
        let ChannelCreateRequest {
            channel_name,
            creator_username,
        } = request.into_inner();
        let channel = Channel::new(&channel_name, &creator_username);

        self.channel_manager.create_channel(channel.clone())?;

        Ok(Response::new(ChannelCreateResponse {
            channel: Some(channel.into()),
        }))
    }

    async fn get_channel_list(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<ChannelListResponse>, Status> {
        Ok(Response::new(self.channel_manager.get_channel_list()?))
    }

    async fn join_channel(
        &self,
        request: Request<JoinChannelRequest>,
    ) -> Result<Response<JoinChannelResponse>, Status> {
        let JoinChannelRequest {
            channel_id,
            username,
        } = request.into_inner();
        let channel_id = str_to_uuid(&channel_id)?;
        let added_user_uuid = self
            .channel_manager
            .add_user_to_channel(&username, channel_id)?;

        Ok(Response::new(JoinChannelResponse {
            internal_user_uuid: added_user_uuid.to_string(),
        }))
    }

    async fn leave_channel(
        &self,
        request: Request<LeaveChannelRequest>,
    ) -> Result<Response<Empty>, Status> {
        let LeaveChannelRequest {
            channel_id,
            user_uuid,
        } = request.into_inner();

        self.channel_manager
            .remove_user_from_channel(str_to_uuid(&user_uuid)?, str_to_uuid(&channel_id)?)?;

        Ok(Response::new(Empty {}))
    }

    type SubscribeChannelStream = ResponseStream<Message>;
    async fn subscribe_channel(
        &self,
        request: Request<SubscribeChannelRequest>,
    ) -> Result<Response<Self::SubscribeChannelStream>, Status> {
        let SubscribeChannelRequest { channel_id } = request.into_inner();
        let rx = self
            .channel_manager
            .subscribe_to_channel(str_to_uuid(&channel_id)?)?;

        let stream = BroadcastStream::new(rx).filter_map(|item| item.ok());

        Ok(Response::new(Box::pin(stream)))
    }

    async fn send_message(&self, request: Request<Message>) -> Result<Response<Empty>, Status> {
        self.channel_manager
            .broadcast_message(request.into_inner())?;

        Ok(Response::new(Empty {}))
    }
}
