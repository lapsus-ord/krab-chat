use crate::channel::Channel;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tonic::Status;
use uuid::Uuid;

/// To simplify response stream of tokio/tonic
pub type ResponseStream<T> = Pin<Box<dyn tokio_stream::Stream<Item = Result<T, Status>> + Send>>;

/// Broadcast channel size
pub const CHANNEL_SIZE: usize = 10;

/// Shared type for the channels (chatting channels)
pub type SharedChannels = Arc<Mutex<Channels>>;
pub type Channels = HashMap<Uuid, Channel>;

pub fn str_to_uuid(input: &str) -> Result<Uuid, Status> {
    Uuid::parse_str(input).map_err(|_| Status::invalid_argument("invalid UUID"))
}
