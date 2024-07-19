use std::error::Error;

use diesel::RunQueryDsl;

use crate::{
    db::DB_MANAGER,
    errors::ServiceError,
    question::{
        dto::QuestionWithAnswersDto,
        models::{Answer, Question},
    },
    schema::{answers, exam_questions, exams, questions},
};

use super::models::{Exam, NewExam, UpdateExam};
use crate::diesel::*;

pub fn create_exam(new_exam: NewExam) -> Result<Exam, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Exam = diesel::insert_into(exams::table)
        .values(new_exam)
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(result)
}

pub fn get_exam_by_id(exam_id: i32) -> Result<Option<Exam>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let exam = exams::table
        .find(exam_id)
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(exam)
}

pub fn update_exam(exam_id: i32, new_exam: UpdateExam) -> Result<Exam, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Exam = diesel::update(exams::table.find(exam_id))
        .set(new_exam)
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(result)
}

pub fn delete_exam(exam_id: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    diesel::delete(exams::table.find(exam_id))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

pub fn update_questions_in_exam(exam_id: i32, question_ids: Vec<i32>) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result = conn.transaction::<_, Box<dyn Error>, _>(|tx| {
        diesel::delete(exam_questions::table.filter(exam_questions::exam_id.eq(exam_id)))
            .execute(tx)?;

        for question_id in question_ids {
            diesel::insert_into(exam_questions::table)
                .values((
                    exam_questions::exam_id.eq(exam_id),
                    exam_questions::question_id.eq(question_id),
                ))
                .execute(tx)?;
        }

        Ok(())
    });

    if result.is_err() {
        println!("{:?}", result.err().unwrap());
        return Err(ServiceError::InternalServerError);
    }

    Ok(())
}

pub fn get_exam_questions(exam_id: i32) -> Result<Vec<QuestionWithAnswersDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let questions: Vec<Question> = exam_questions::table
        .inner_join(questions::table)
        .filter(exam_questions::exam_id.eq(exam_id))
        .select(questions::all_columns)
        .load(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    let mut result = Vec::new();

    for question in questions {
        let answers: Vec<Answer> = answers::table
            .filter(answers::question_id.eq(question.id))
            .load(&mut conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        let answers_dto = answers
            .iter()
            .map(|answer| crate::question::dto::AnswerDto {
                id: answer.id,
                answer: answer.answer.clone(),
                is_correct: Some(answer.is_correct),
            })
            .collect();

        result.push(QuestionWithAnswersDto {
            id: question.id,
            question: question.question,
            answers: answers_dto,
        });
    }

    Ok(result)
}
