use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub group_id: u32,
    pub name: String,
    pub campus_id: u32,
}
