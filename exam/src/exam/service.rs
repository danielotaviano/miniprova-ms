use crate::{
    api::{self, GetExamApi},
    auth::models::LoggedUser,
    class,
    errors::ServiceError,
};

use super::{
    dto::{GetStudentOpenExamDto, GetStudentQuestionDto},
    repository,
};

pub async fn get_student_finished_exams(
    uid: i32,
) -> Result<Vec<GetStudentOpenExamDto>, ServiceError> {
    let exam_ids = repository::get_student_finished_exams(uid)?;

    let exams: Vec<_> = exam_ids
        .iter()
        .map(|exam_id| GetStudentOpenExamDto {
            class_name: exam_id.1.clone(),
            end_time: exam_id.4,
            exam_name: exam_id.2.clone(),
            start_time: exam_id.3,
        })
        .collect();

    Ok(exams)
}

pub async fn get_student_open_exams(uid: i32) -> Result<Vec<GetStudentOpenExamDto>, ServiceError> {
    let exam_ids = repository::get_student_open_exams(uid)?;
    println!("{:?}", exam_ids);

    let exams: Vec<_> = exam_ids
        .iter()
        .map(|exam_id| GetStudentOpenExamDto {
            class_name: exam_id.1.clone(),
            end_time: exam_id.4,
            exam_name: exam_id.2.clone(),
            start_time: exam_id.3,
        })
        .collect();

    Ok(exams)
}

pub async fn get_questions_as_student(
    exam_id: i32,
) -> Result<Vec<GetStudentQuestionDto>, ServiceError> {
    let exam = class::repository::get_class_exam(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::NotFound);
    }

    let exam = exam.unwrap();

    let current_time = chrono::Utc::now();

    if exam.start_time > current_time {
        return Err(ServiceError::BadRequest(
            "Exam has not started yet".to_string(),
        ));
    }

    let questions = repository::get_student_questions(exam_id)?;

    Ok(questions)
}
