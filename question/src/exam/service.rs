use crate::{errors::ServiceError, question};

use super::{
    dto::CreateExamInputDto,
    models::{Exam, NewExam, UpdateExam},
    repository,
};

pub fn create_exam(new_exam: CreateExamInputDto) -> Result<Exam, ServiceError> {
    if new_exam.start_date > new_exam.end_date {
        return Err(ServiceError::BadRequest(
            "End date must be after start date".to_string(),
        ));
    }

    if new_exam.start_date < chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest(
            "Start date must be in the future".to_string(),
        ));
    }

    let exam = repository::create_exam(NewExam {
        name: new_exam.name,
    })?;
    Ok(exam)
}

pub fn get_exam_by_id(exam_id: i32) -> Result<Option<Exam>, ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;
    Ok(exam)
}

pub fn update_exam(exam_id: i32, new_exam: UpdateExam) -> Result<Exam, ServiceError> {
    let existing = repository::get_exam_by_id(exam_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let exam = repository::update_exam(exam_id, new_exam)?;
    Ok(exam)
}

pub fn delete_exam(exam_id: i32) -> Result<(), ServiceError> {
    let existing = repository::get_exam_by_id(exam_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    repository::delete_exam(exam_id)?;
    Ok(())
}

pub fn update_questions_in_exam(exam_id: i32, question_ids: Vec<i32>) -> Result<(), ServiceError> {
    let existing = repository::get_exam_by_id(exam_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let maybe_errors = question_ids
        .iter()
        .map(|question_id| {
            let question = question::service::get_question_by_id(*question_id);
            if question.is_err() {
                return Some(ServiceError::InternalServerError);
            }

            if question.unwrap().is_none() {
                return Some(ServiceError::BadRequest(format!(
                    "Question {} not found",
                    question_id
                )));
            }

            None
        })
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    if !maybe_errors.is_empty() {
        let error = maybe_errors.get(0).unwrap();
        return Err(error.clone());
    }

    repository::update_questions_in_exam(exam_id, question_ids)?;
    Ok(())
}

pub fn get_exam_questions(
    exam_id: i32,
) -> Result<Vec<question::dto::QuestionWithAnswersDto>, ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let questions = repository::get_exam_questions(exam_id)?;
    Ok(questions)
}
