use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DHBWCourse {
    id: String,
    description: String,
    grade: Option<String>,
    ects_points: String,
}

impl DHBWCourse {
    pub fn new(
        id: String,
        description: String,
        grade: Option<String>,
        ects_points: String,
    ) -> Self {
        DHBWCourse {
            id,
            description,
            grade,
            ects_points,
        }
    }
}
