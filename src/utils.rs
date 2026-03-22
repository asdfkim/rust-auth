use chrono::Utc;

pub fn now_ms() -> u64 {
    Utc::now().timestamp_millis() as u64
}
