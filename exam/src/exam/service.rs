use crate::{
    api::{self, GetExamApi},
    auth::models::LoggedUser,
    class,
    errors::ServiceError,
};

use super::{
    dto::{GetStudentExamResultDto, GetStudentOpenExamDto, GetStudentQuestionDto},
    repository::{self, StudentQuestionResult},
};

pub async fn get_student_finished_exams(
    uid: i32,
) -> Result<Vec<GetStudentOpenExamDto>, ServiceError> {
    let exam_ids = repository::get_student_finished_exams(uid)?;

    let exams: Vec<_> = exam_ids
        .iter()
        .map(|exam_id| GetStudentOpenExamDto {
            id: exam_id.0,
            class_name: exam_id.1.clone(),
            end_time: exam_id.4,
            exam_name: exam_id.2.clone(),
            start_time: exam_id.3,
        })
        .collect();

    Ok(exams)
}

pub async fn get_student_exam_result(
    uid: i32,
    exam_id: i32,
) -> Result<Vec<GetStudentExamResultDto>, ServiceError> {
    let exam = class::repository::get_class_exam(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let exam = exam.unwrap();

    if chrono::Utc::now().naive_utc() < exam.end_time {
        return Err(ServiceError::BadRequest(
            "Exam not finished yet".to_string(),
        ));
    }

    let is_enrolled = class::service::is_student_enrolled(exam.class_id, uid)?;

    if !is_enrolled {
        return Err(ServiceError::Forbidden);
    }

    let result = repository::get_student_exam_result(exam_id, uid)?;

    Ok(result)
}

pub async fn get_student_open_exams(uid: i32) -> Result<Vec<GetStudentOpenExamDto>, ServiceError> {
    let exam_ids = repository::get_student_open_exams(uid)?;
    println!("{:?}", exam_ids);

    let exams: Vec<_> = exam_ids
        .iter()
        .map(|exam_id| GetStudentOpenExamDto {
            id: exam_id.0,
            class_name: exam_id.1.clone(),
            end_time: exam_id.4,
            exam_name: exam_id.2.clone(),
            start_time: exam_id.3,
        })
        .collect();

    Ok(exams)
}

pub fn submit_answer_to_question_in_exam(
    user_id: i32,
    exam_id: i32,
    question_id: i32,
    answer_id: i32,
) -> Result<(), ServiceError> {
    let exam = class::repository::get_class_exam(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let exam = exam.unwrap();

    let is_enrolled = class::service::is_student_enrolled(exam.class_id, user_id)?;

    if !is_enrolled {
        return Err(ServiceError::Forbidden);
    }

    if exam.start_time > chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest("Exam not started yet".to_string()));
    }

    if exam.end_time < chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest("Exam already ended".to_string()));
    }

    let question = repository::get_question_by_id(question_id)?;

    if question.is_none() {
        return Err(ServiceError::BadRequest("Question not found".to_string()));
    }
    let question = question.unwrap();

    let answer = question.answers.iter().find(|a| a.id == answer_id);

    if answer.is_none() {
        return Err(ServiceError::BadRequest("Answer not found".to_string()));
    }

    repository::submit_answer_to_question_in_exam(exam_id, question_id, user_id, answer_id)?;
    Ok(())
}

pub async fn get_questions_as_student(
    exam_id: i32,
    user_id: i32,
) -> Result<Vec<GetStudentQuestionDto>, ServiceError> {
    println!("Getting questions for exam {}", exam_id);
    let exam = class::repository::get_class_exam(exam_id)?;
    println!("{:?}", exam);

    if exam.is_none() {
        return Err(ServiceError::NotFound);
    }

    let exam = exam.unwrap();

    let current_time = chrono::Utc::now().naive_utc();

    if exam.start_time > current_time {
        return Err(ServiceError::BadRequest(
            "Exam has not started yet".to_string(),
        ));
    }

    println!("Getting questions for exam123123 {}", exam_id);

    let questions = repository::get_student_questions(exam_id, user_id)?;

    Ok(questions)
}
