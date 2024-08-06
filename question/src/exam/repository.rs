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

use super::{
    dto::GetExamWithQuestionCountDto,
    models::{Exam, NewExam, UpdateExam},
};
use crate::diesel::*;

pub fn create_exam(new_exam: NewExam, questions: Vec<i32>) -> Result<Exam, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Exam = diesel::insert_into(exams::table)
        .values(new_exam)
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    for question_id in questions {
        diesel::insert_into(exam_questions::table)
            .values((
                exam_questions::exam_id.eq(result.id),
                exam_questions::question_id.eq(question_id),
            ))
            .execute(&mut conn)
            .map_err(|_| ServiceError::InternalServerError)?;
    }

    Ok(result)
}

pub fn get_exams() -> Result<Vec<GetExamWithQuestionCountDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let exams: Vec<Exam> = exams::table
        .load(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    let result = exams
        .iter()
        .map(|exam| {
            let question_count: i64 = exam_questions::table
                .filter(exam_questions::exam_id.eq(exam.id))
                .count()
                .get_result(&mut conn)
                .map_err(|_| ServiceError::InternalServerError)?;

            Ok(GetExamWithQuestionCountDto {
                id: exam.id,
                name: exam.name.clone(),
                created_at: exam.created_at,
                question_count,
            })
        })
        .collect::<Result<Vec<GetExamWithQuestionCountDto>, ServiceError>>()?;

    Ok(result)
}

pub fn get_exam_by_id(exam_id: i32) -> Result<Option<GetExamWithQuestionCountDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let exam: Option<Exam> = exams::table
        .find(exam_id)
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    if exam.is_none() {
        return Ok(None);
    }

    let exam = exam.unwrap();

    let question_count: i64 = exam_questions::table
        .filter(exam_questions::exam_id.eq(exam_id))
        .count()
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(Some(GetExamWithQuestionCountDto {
        id: exam_id,
        name: exam.name,
        created_at: exam.created_at,
        question_count,
    }))
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

    // delete relations

    diesel::delete(exam_questions::table.filter(exam_questions::exam_id.eq(exam_id)))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

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
