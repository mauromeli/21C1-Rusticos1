#[derive(Debug)]
/// PubSubParam: Enum usado para representar los parametros permitidos para el Command::PubSub.
pub enum PubSubParam {
    Channels,
    ChannelsWithChannel(String),
    Numsub,
    NumsubWithChannels(Vec<String>),
}
