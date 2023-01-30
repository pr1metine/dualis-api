use serde::Serialize;

use super::DHBWCourse;

#[derive(Debug, Serialize)]
struct PerformanceOverview {
    courses: Vec<DHBWCourse>,
}
