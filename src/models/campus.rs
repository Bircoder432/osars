use super::Group;

pub struct Campus {
    pub campus_id: u32,
    pub name: String,
    pub college_id: u32,
    pub groups: Vec<Group>,
}
