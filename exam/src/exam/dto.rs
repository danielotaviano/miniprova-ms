use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStudentOpenExamDto {
    pub exam_name: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub class_name: String,
}
