use super::Lesson;

pub struct Schedule {
    pub group_id: u32,
    pub date: String,
    pub lessons: Vec<Lesson>,
}
