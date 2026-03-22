use chrono::Utc;

pub fn now_unix() -> i64 {
    Utc::now().timestamp()
}
