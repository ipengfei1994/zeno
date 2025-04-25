use chrono::{DateTime, Utc};

pub fn generate_timestamp_id() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y%m%d%H%M%S").to_string()
}
