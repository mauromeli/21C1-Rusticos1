use std::time::{Duration, SystemTime};

/// Decoder usado para convertir SystemTime a Strings.
/// Esto es usado en los logs para marcar el horario del evento y en los momentos donde se consulta
/// el horario del servidor.
pub fn timestamp_to_string(time: SystemTime) -> String {
    let time = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs();

    let secs_offset = time % (24 * 3600);
    let hs = secs_offset / 3600;
    let mins = secs_offset % 3600 / 60;
    let secs = secs_offset % 60;
    format!("{:02}:{:02}:{:02}", hs, mins, secs)
}
