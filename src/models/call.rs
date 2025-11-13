use chrono::{DateTime, Utc};

pub struct Call {
    pub call_id: u32,
    pub weekday: u8,
    pub begins: DateTime<Utc>,
    pub ends: DateTime<Utc>,
    pub order: u32,
}
