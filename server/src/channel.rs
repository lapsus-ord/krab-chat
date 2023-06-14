use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub creator: String,
    pub clients: HashSet<String>,
}

impl Channel {
    pub fn new(channel_id: String, channel_name: String, channel_creator: String) -> Self {
        Channel {
            id: channel_id,
            name: channel_name,
            creator: channel_creator,
            clients: HashSet::new(),
        }
    }
}
