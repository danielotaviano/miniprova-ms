use crate::{errors::ServiceError, question};

use super::{
    dto::{CreateExamInputDto, GetExamWithQuestionCountDto},
    models::{Exam, NewExam, UpdateExam},
    repository,
};

pub fn create_exam(new_exam: CreateExamInputDto) -> Result<Exam, ServiceError> {
    let exam = repository::create_exam(
        NewExam {
            name: new_exam.name,
        },
        new_exam.questions,
    )?;
    Ok(exam)
}

pub fn get_exams() -> Result<Vec<GetExamWithQuestionCountDto>, ServiceError> {
    let exams = repository::get_exams()?;
    Ok(exams)
}

pub fn get_exam_by_id(exam_id: i32) -> Result<Option<GetExamWithQuestionCountDto>, ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;
    Ok(exam)
}

pub fn update_exam(
    exam_id: i32,
    new_exam: UpdateExam,
    questions: Vec<i32>,
) -> Result<Exam, ServiceError> {
    let existing = repository::get_exam_by_id(exam_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let exam = repository::update_exam(exam_id, new_exam)?;
    repository::update_questions_in_exam(exam_id, questions)?;
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
