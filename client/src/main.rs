use proto::chat::{chat_client::ChatClient, ChannelCreateRequest, ChannelSelectRequest, Message};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "anon1234".to_owned();
    let mut client = ChatClient::connect("http://[::1]:3000").await?;

    // Create a channel
    let create_channel = Request::new(ChannelCreateRequest {
        channel_name: "room1".to_owned(),
        creator_username: username.clone(),
    });

    let response = client.create_channel(create_channel).await?;
    println!("create_channel: {:?}", response);

    // Join a channel
    let select_channel = Request::new(ChannelSelectRequest {
        channel_id: "1".to_owned(),
    });

    let response = client.join_channel(select_channel).await?;
    println!("join_channel: {:?}", response);

    // Send a message
    let message = Request::new(Message {
        username: username.to_owned(),
        content: "hello world!".to_owned(),
        channel_id: "1".to_owned(),
    });

    let response = client.send_message(message).await?;
    println!("send_message: {:?}", response);

    // Leave a channel
    let select_channel = Request::new(ChannelSelectRequest {
        channel_id: "1".to_owned(),
    });

    let response = client.leave_channel(select_channel).await?;
    println!("leave_channel: {:?}", response);

    Ok(())
}
