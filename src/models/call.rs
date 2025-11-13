use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    pub call_id: u32,
    pub weekday: u8,
    pub begins: DateTime<Utc>,
    pub ends: DateTime<Utc>,
    pub order: u32,
}
