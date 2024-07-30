use crate::{
    api::{self, GetExamApi},
    auth::models::LoggedUser,
    errors::ServiceError,
};

use super::{dto::GetStudentOpenExamDto, repository};

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
