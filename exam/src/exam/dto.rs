use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStudentOpenExamDto {
    pub exam_name: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub class_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStudentAnswerDto {
    pub id: i32,
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStudentQuestionDto {
    pub id: i32,
    pub question: String,
    pub answers: Vec<GetStudentAnswerDto>,
}
