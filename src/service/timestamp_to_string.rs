use std::time::{Duration, SystemTime};

pub fn timestamp_to_string(time: SystemTime) -> String {
    let time = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs();

    let secs_offset = time % (24 * 3600);
    let hs = secs_offset / 3600;
    let mins = secs_offset % 3600 / 60;
    let secs = secs_offset % 60;
    format!(
        "{}:{}:{}",
        format_ceros(hs),
        format_ceros(mins),
        format_ceros(secs)
    )
}

fn format_ceros(num: u64) -> String {
    if num < 10 {
        return format!("0{}", num);
    }
    format!("{}", num)
}
