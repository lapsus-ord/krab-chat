use crate::channel::Channel;
use crate::utils::{str_to_uuid, Channels, SharedChannels};
use proto::chat::{ChannelListResponse, Message};
use std::sync::MutexGuard;
use tokio::sync::broadcast;
use tonic::Status;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct ChannelManager {
    channels: SharedChannels,
}

impl ChannelManager {
    pub fn create_channel(&self, channel: Channel) -> Result<(), Status> {
        let mut channels = self.get_lock()?;

        channels.insert(channel.id, channel);
        info!("new channel size: {}", channels.len());

        Ok(())
    }

    pub fn get_channel_list(&self) -> Result<ChannelListResponse, Status> {
        let lock = self.get_lock()?;
        let channels = lock.values();
        info!("get channels: {:?}", &channels);
        let channels = channels.cloned().map(Into::into).collect();

        Ok(ChannelListResponse { channels })
    }

    pub fn add_user_to_channel(&self, username: &str, channel_id: Uuid) -> Result<Uuid, Status> {
        let mut lock = self.get_lock()?;
        let channel = lock
            .get_mut(&channel_id)
            .ok_or(Status::not_found("channel not found"))?;

        let new_user_uuid = channel.add_user(username);
        info!(
            "added user {} with uuid {} to channel {:?}",
            username, new_user_uuid, channel
        );

        Ok(new_user_uuid)
    }

    pub fn remove_user_from_channel(
        &self,
        user_uuid: Uuid,
        channel_id: Uuid,
    ) -> Result<(), Status> {
        let mut lock = self.get_lock()?;
        let channel = lock
            .get_mut(&channel_id)
            .ok_or(Status::not_found("channel not found"))?;

        let username = channel
            .remove_user(&user_uuid)
            .ok_or(Status::not_found("user not found"))?;
        info!(
            "removed user {} with uuid {} from channel {:?}",
            username, user_uuid, channel
        );

        Ok(())
    }

    pub fn subscribe_to_channel(
        &self,
        channel_id: Uuid,
    ) -> Result<broadcast::Receiver<Result<Message, Status>>, Status> {
        let lock = self.get_lock()?;
        let channel = lock
            .get(&channel_id)
            .ok_or(Status::not_found("channel not found"))?;
        info!("new subscriber to channel {:?}", channel);

        Ok(channel.sender.subscribe())
    }

    pub fn broadcast_message(&self, new_message: Message) -> Result<(), Status> {
        let channel_id = str_to_uuid(&new_message.channel_id)?;
        let lock = self.get_lock()?;
        let channel = lock
            .get(&channel_id)
            .ok_or(Status::not_found("channel not found"))?;

        info!("broadcast new message: {:?}", &new_message);
        channel.broadcast(new_message);

        Ok(())
    }

    fn get_lock(&self) -> Result<MutexGuard<Channels>, Status> {
        self.channels.lock().map_err(|e| {
            error!("mutex poison error: {}", e);
            Status::internal("corrupted mutex")
        })
    }
}
