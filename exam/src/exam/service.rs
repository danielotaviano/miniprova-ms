use crate::{
    api::{self, GetExamApi},
    auth::models::LoggedUser,
    errors::ServiceError,
};

use super::{dto::GetStudentOpenExamDto, repository};

pub async fn get_student_finished_exams(
    uid: i32,
    user_jwt: String,
) -> Result<Vec<GetStudentOpenExamDto>, ServiceError> {
    let exam_ids = repository::get_student_finished_exams(uid)?;

    let exams: Vec<_> = exam_ids
        .iter()
        .map(|exam_id| async {
            let exam = api::get_exam(exam_id.0, user_jwt.clone()).await;
            match exam {
                Ok(exam) => Ok(GetStudentOpenExamDto {
                    class_name: exam_id.1.clone(),
                    end_time: exam_id.3,
                    exam_name: exam.name,
                    start_time: exam_id.2,
                }),
                Err(e) => Err(ServiceError::from(e)),
            }
        })
        .collect();

    let exams = futures::future::join_all(exams).await;

    let mut result = Vec::new();
    for exam in exams {
        match exam {
            Ok(exam) => result.push(exam),
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

pub async fn get_student_open_exams(
    uid: i32,
    user_jwt: String,
) -> Result<Vec<GetStudentOpenExamDto>, ServiceError> {
    let exam_ids = repository::get_student_open_exams(uid)?;
    println!("{:?}", exam_ids);

    let exams: Vec<_> = exam_ids
        .iter()
        .map(|exam_id| async {
            let exam = api::get_exam(exam_id.0, user_jwt.clone()).await;
            println!("{:?}", exam);
            match exam {
                Ok(exam) => Ok(GetStudentOpenExamDto {
                    class_name: exam_id.1.clone(),
                    end_time: exam_id.3,
                    exam_name: exam.name,
                    start_time: exam_id.2,
                }),
                Err(e) => Err(ServiceError::from(e)),
            }
        })
        .collect();

    let exams = futures::future::join_all(exams).await;

    let mut result = Vec::new();
    for exam in exams {
        match exam {
            Ok(exam) => result.push(exam),
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}
