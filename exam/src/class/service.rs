use crate::{
    api::{self},
    auth::models::LoggedUser,
    class::repository,
    errors::ServiceError,
    exam::{self, model::NewExam, repository::ImportQuestion},
};

use super::dto::AddExamToClassDto;

pub async fn add_exam_to_class(
    class_id: i32,
    exam: AddExamToClassDto,
    user: &LoggedUser,
) -> Result<(), ServiceError> {
    let class = api::get_class_by_id(user.jwt.clone(), class_id).await?;

    if class.is_none() {
        return Err(ServiceError::BadRequest("Class not found".to_string()));
    }

    

    let api_exam = api::get_exam(exam.exam_id, user.jwt.clone()).await?;
    
    let questions = api::get_exam_questions(api_exam.id, user.jwt.clone()).await?;
    

    let formated_questions: Vec<ImportQuestion> = questions
        .iter()
        .map(|q| ImportQuestion {
            question: q.question.clone(),
            answers: q
                .answers
                .iter()
                .map(|a| (a.answer.clone(), a.is_correct))
                .collect(),
        })
        .collect();
    

    let db_exam_id = exam::repository::import_exam(
        NewExam {
            name: api_exam.name,
        },
        formated_questions,
    )?;
    

    

    repository::add_exam_to_class(class_id, db_exam_id, exam.start_date, exam.end_date)?;

    Ok(())
}
