use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StudentAnswerInputDto {
    pub answer_id: i32,
}

impl StudentAnswerInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.answer_id <= 0 {
            return Err("Answer id is required".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStudentOpenExamDto {
    pub id: i32,
    pub exam_name: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub class_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStudentAnswerDto {
    pub id: i32,
    pub answer: String,
    pub marked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStudentQuestionDto {
    pub id: i32,
    pub question: String,
    pub answers: Vec<GetStudentAnswerDto>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct GetStudentExamResultAnswersDto {
    pub id: i32,
    pub answer: String,
    pub correct: bool,
    pub marked: bool,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct GetStudentExamResultDto {
    pub id: i32,
    pub question: String,
    pub answers: Vec<GetStudentExamResultAnswersDto>,
}
