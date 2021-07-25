#[derive(Debug)]
/// Infoparam: Enum usado para representar los parametros permitidos para el Command::Info.
pub enum InfoParam {
    /// Utilizado para el parametro ProcessID del Comando Info
    ProcessId,
    /// Utilizado para el parametro Port del Comando Info
    Port,
    /// Utilizado para el parametro ServerTime del Comando Info
    ServerTime,
    /// Utilizado para el parametro Uptime del Comando Info
    Uptime,
    /// Utilizado para el parametro ConfigFile del Comando Info
    ConfigFile,
    /// Utilizado para el parametro ConnectedClients del Comando Info
    ConnectedClients,
}
