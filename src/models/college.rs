use super::{Call, Campus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct College {
    pub college_id: u32,
    pub name: String,
    pub calls: Vec<Call>,
    pub campuses: Vec<Campus>,
}
