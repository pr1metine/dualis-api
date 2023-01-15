use serde::Deserialize; 

#[derive(Deserialize)]
pub struct CoursesQuery {
    pub semester_id: String,
}