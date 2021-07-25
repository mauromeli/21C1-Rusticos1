#[derive(Debug)]
/// Infoparam: Enum usado para representar los parametros permitidos para el Command::Info.
pub enum InfoParam {
    ProcessId,
    Port,
    ServerTime,
    Uptime,
    ConfigFile,
    ConnectedClients,
}
