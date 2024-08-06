use futures::future::join_all;

use crate::{
    api, auth::models::LoggedUser, class, errors::ServiceError, exam::dto::GetTeacherExamDto,
};

use super::{
    dto::{
        GetStudentExamResultDto, GetStudentOpenExamDto, GetStudentQuestionDto, StudentExamResultDto,
    },
    repository::{self},
};

pub async fn get_teacher_exams(user: &LoggedUser) -> Result<Vec<GetTeacherExamDto>, ServiceError> {
    let classes = api::get_teacher_classes(user.jwt.clone()).await?;
    let class_ids: Vec<_> = classes.iter().map(|c| c.id).collect();
    let exams = repository::get_classes_exams(class_ids)?;

    let exams: Vec<_> = exams
        .iter()
        .map(|exam_id| GetTeacherExamDto {
            id: exam_id.0,
            class_name: classes
                .iter()
                .find(|c| c.id == exam_id.2)
                .unwrap()
                .name
                .clone(),
            end_time: exam_id.4,
            exam_name: exam_id.1.clone(),
            start_time: exam_id.3,
        })
        .collect();

    Ok(exams)
}

pub async fn get_exam_results_as_teacher(
    user: &LoggedUser,
    exam_id: i32,
) -> Result<Vec<StudentExamResultDto>, ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    

    let results = repository::get_exam_results_as_teacher(exam_id)?;
    
    let results: Vec<_> = results
        .iter()
        .map(|result| async {
            let user_id = result.id;
            let user = api::get_student_by_id(user_id, user.jwt.clone())
                .await
                .unwrap();

            StudentExamResultDto {
                name: user.name.clone(),
                id: user_id,
                answered_questions: result.answered_questions,
                score: result.score,
                total_questions: result.total_questions,
            }
        })
        .collect();

    let results = join_all(results).await;

    Ok(results)
}

pub async fn get_student_finished_exams(
    user: &LoggedUser,
) -> Result<Vec<GetStudentOpenExamDto>, ServiceError> {
    let enrolled_classes = api::get_enrolled_classes(user.jwt.clone()).await?;

    let class_ids: Vec<_> = enrolled_classes.iter().map(|c| c.id).collect();
    let exam_ids = repository::get_classes_finished_exams(class_ids)?;

    let exams: Vec<_> = exam_ids
        .iter()
        .map(|exam_id| GetStudentOpenExamDto {
            id: exam_id.0,
            class_name: enrolled_classes
                .iter()
                .find(|c| c.id == exam_id.2)
                .unwrap()
                .name
                .clone(),
            end_time: exam_id.4,
            exam_name: exam_id.1.clone(),
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

    let result = repository::get_student_exam_result(exam_id, uid)?;

    Ok(result)
}

pub async fn get_student_open_exams(
    user: &LoggedUser,
) -> Result<Vec<GetStudentOpenExamDto>, ServiceError> {
    let enrolled_classes = api::get_enrolled_classes(user.jwt.clone()).await?;
    

    let class_ids: Vec<_> = enrolled_classes.iter().map(|c| c.id).collect();
    let exam_ids = repository::get_classes_open_exams(class_ids)?;
    

    let exams: Vec<_> = exam_ids
        .iter()
        .map(|exam_id| GetStudentOpenExamDto {
            id: exam_id.0,
            class_name: enrolled_classes
                .iter()
                .find(|c| c.id == exam_id.2)
                .unwrap()
                .name
                .clone(),
            end_time: exam_id.4,
            exam_name: exam_id.1.clone(),
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
    
    let exam = class::repository::get_class_exam(exam_id)?;
    

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

    

    let questions = repository::get_student_questions(exam_id, user_id)?;

    Ok(questions)
}
