use chrono::{DateTime, Duration, Utc};

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn add_days(days: i64) -> DateTime<Utc> {
    now() + Duration::days(days)
}

pub fn is_expired(ts: DateTime<Utc>) -> bool {
    now() >= ts
}

pub fn hours_left(ts: DateTime<Utc>) -> i64 {
    if is_expired(ts) {
        0
    } else {
        (ts - now()).num_hours()
    }
}
