use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub lesson_id: u32,
    pub group_id: u32,
    pub order: u32,
    pub title: String,
    pub teacher: String,
    pub cabinet: String,
}
