#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
/// LogLevel: Enum utilizado para representar el nivel de un log.
pub enum LogLevel {
    /// Nivel de logeo tipo Debug
    Debug,
    /// Nivel de logeo tipo Info
    Info,
    /// Nivel de logeo tipo Error
    Error,
}
