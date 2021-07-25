#[derive(Debug)]
/// PubSubParam: Enum usado para representar los parametros permitidos para el Command::PubSub.
pub enum PubSubParam {
    /// Representa el Parametro Channels de PubSub con canales vacíos.
    Channels,
    /// Representa el Parametro Channels de PubSub con canal especifico.
    ChannelsWithChannel(String),
    /// Representa el Parametro NumSub de PubSub con canales vacíos.
    Numsub,
    /// Representa el Parametro Numsub de PubSub con canales específicos.
    NumsubWithChannels(Vec<String>),
}
