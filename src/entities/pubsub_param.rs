#[derive(Debug)]
pub enum PubSubParam {
    Channels,
    ChannelsWithChannel(String),
    Numsub,
    NumsubWithChannels(Vec<String>),
}