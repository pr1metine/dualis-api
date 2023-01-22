use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DHBWSemester {
    id: String,
    description: String,
}

impl DHBWSemester {
    pub fn new(id: String, description: String) -> Self {
        Self { id, description }
    }
}
