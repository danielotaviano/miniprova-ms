use crate::diesel::OptionalExtension;
use std::error::Error;

use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    db::DB_MANAGER,
    errors::ServiceError,
    question::models::Answer,
    schema::{answers, questions},
};

use super::{
    dto::{AnswerDto, CreateQuestionInputDto, QuestionWithAnswersDto},
    models::{NewAnswer, NewQuestion, Question},
};

pub fn create_question(new_question: CreateQuestionInputDto) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Result<(), Box<dyn Error>> = conn.transaction(|tx| {
        let question = diesel::insert_into(questions::table)
            .values(&NewQuestion {
                question: &new_question.question,
            })
            .returning(questions::all_columns)
            .get_result::<Question>(tx)?;

        for answer in new_question.answers {
            let _: Answer = diesel::insert_into(answers::table)
                .values(&NewAnswer {
                    answer: answer.answer,
                    is_correct: answer.is_correct,
                    question_id: question.id,
                })
                .returning(answers::all_columns)
                .get_result(tx)?;
        }

        Ok(())
    });

    if result.is_err() {
        
        return Err(ServiceError::InternalServerError);
    }

    Ok(())
}

pub fn get_question_by_id(
    question_id: i32,
) -> Result<Option<QuestionWithAnswersDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let question = questions::table
        .filter(questions::id.eq(question_id))
        .first::<Question>(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    let question = match question {
        Some(q) => {
            let answers: Vec<Answer> = answers::table
                .filter(answers::question_id.eq(q.id))
                .load::<Answer>(&mut conn)
                .map_err(|_| ServiceError::InternalServerError)?;

            Some(QuestionWithAnswersDto {
                id: q.id,
                question: q.question,
                answers: answers
                    .into_iter()
                    .map(|a| AnswerDto {
                        answer: a.answer,
                        id: a.id,
                        is_correct: Some(a.is_correct),
                    })
                    .collect(),
            })
        }
        None => None,
    };

    Ok(question)
}

pub fn delete_question_by_id(question_id: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    diesel::delete(answers::table.filter(answers::question_id.eq(question_id)))
        .execute(&mut conn)
        .map_err(|e| {
            
            ServiceError::InternalServerError
        })?;

    diesel::delete(questions::table.filter(questions::id.eq(question_id)))
        .execute(&mut conn)
        .map_err(|e| {
            
            ServiceError::InternalServerError
        })?;

    Ok(())
}

pub fn update_question(
    question_id: i32,
    new_question: CreateQuestionInputDto,
) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Result<(), Box<dyn Error>> = conn.transaction(|tx| {
        diesel::delete(answers::table.filter(answers::question_id.eq(question_id))).execute(tx)?;

        diesel::update(questions::table.filter(questions::id.eq(question_id)))
            .set(questions::question.eq(&new_question.question))
            .execute(tx)?;

        for answer in new_question.answers {
            diesel::insert_into(answers::table)
                .values(&NewAnswer {
                    answer: answer.answer,
                    is_correct: answer.is_correct,
                    question_id,
                })
                .execute(tx)?;
        }

        Ok(())
    });

    if result.is_err() {
        return Err(ServiceError::InternalServerError);
    }

    Ok(())
}

pub fn list_questions() -> Result<Vec<Question>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let questions = questions::table
        .order_by(questions::id)
        .load::<Question>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(questions)
}
