use super::Lesson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub group_id: u32,
    pub date: String,
    pub lessons: Vec<Lesson>,
}
