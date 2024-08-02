use chrono::{NaiveDateTime, Utc};
use diesel::{sql_query, QueryableByName, RunQueryDsl};
use serde::Serialize;

use crate::exam::dto::GetStudentAnswerDto;
use crate::{db::DB_MANAGER, errors::ServiceError, schema::answers};

use crate::diesel::ExpressionMethods;

use super::dto::{GetStudentExamResultAnswersDto, GetStudentExamResultDto, GetStudentQuestionDto};
use super::model::{NewAnswer, NewExam, NewQuestion};

#[derive(QueryableByName)]
struct ExamId {
    #[sql_type = "diesel::sql_types::Integer"]
    exam_id: i32,

    #[sql_type = "diesel::sql_types::Text"]
    class_name: String,

    #[sql_type = "diesel::sql_types::Text"]
    exam_name: String,

    #[sql_type = "diesel::sql_types::Timestamp"]
    start_time: NaiveDateTime,

    #[sql_type = "diesel::sql_types::Timestamp"]
    end_time: NaiveDateTime,
}

#[derive(QueryableByName)]
struct Question {
    #[sql_type = "diesel::sql_types::Integer"]
    id: i32,

    #[sql_type = "diesel::sql_types::Text"]
    question: String,

    #[sql_type = "diesel::sql_types::Integer"]
    answer_id: i32,

    #[sql_type = "diesel::sql_types::Text"]
    answer: String,

    #[sql_type = "diesel::sql_types::Bool"]
    marked: bool,
}

#[derive(QueryableByName, Serialize)]
pub struct StudentQuestionResult {
    #[sql_type = "diesel::sql_types::Integer"]
    id: i32,

    #[sql_type = "diesel::sql_types::Text"]
    question: String,

    #[sql_type = "diesel::sql_types::Integer"]
    answer_id: i32,

    #[sql_type = "diesel::sql_types::Text"]
    answer: String,

    #[sql_type = "diesel::sql_types::Bool"]
    marked: bool,

    #[sql_type = "diesel::sql_types::Bool"]
    is_correct: bool,
}

pub struct ImportQuestion {
    pub question: String,
    pub answers: Vec<(String, bool)>,
}

pub fn get_student_exam_result(
    exam_id: i32,
    user_id: i32,
) -> Result<Vec<GetStudentExamResultDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let query = sql_query(
        r#"
        SELECT
            q.id,
            q.question,
            a.id "answer_id",
            a.answer,
            sa.id IS NOT NULL "marked",
            a.is_correct
        FROM
            exams e
        INNER JOIN exam_questions eq ON
            eq.exam_id = e.id
        INNER JOIN questions q ON
            q.id = eq.question_id
        INNER JOIN answers a ON
            a.question_id = q.id
        LEFT JOIN student_answers sa ON
            sa.exam_id = e.id
            AND sa.question_id = q.id
            AND sa.answer_id = a.id
            AND sa.user_id = $2
        WHERE
            e.id = $1;
        "#,
    )
    .bind::<diesel::sql_types::Integer, _>(exam_id)
    .bind::<diesel::sql_types::Integer, _>(user_id);

    let results: Vec<StudentQuestionResult> = query
        .get_results::<StudentQuestionResult>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    let mut questions: Vec<GetStudentExamResultDto> = Vec::new();

    for result in results {
        let question = questions.iter_mut().find(|q| q.id == result.id);
        let question = match question {
            Some(q) => q,
            None => {
                let new_question = GetStudentExamResultDto {
                    id: result.id,
                    question: result.question.clone(),
                    answers: Vec::new(),
                };

                questions.push(new_question);
                questions.last_mut().unwrap()
            }
        };

        question.answers.push(GetStudentExamResultAnswersDto {
            answer: result.answer.clone(),
            correct: result.is_correct,
            id: result.answer_id,
            marked: result.marked,
        });
    }

    Ok(questions)
}

pub fn submit_answer_to_question_in_exam(
    exam_id: i32,
    question_id: i32,
    user_id: i32,
    answer_id: i32,
) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    diesel::insert_into(crate::schema::student_answers::table)
        .values((
            crate::schema::student_answers::exam_id.eq(exam_id),
            crate::schema::student_answers::question_id.eq(question_id),
            crate::schema::student_answers::user_id.eq(user_id),
            crate::schema::student_answers::answer_id.eq(answer_id),
        ))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

pub fn get_question_by_id(question_id: i32) -> Result<Option<GetStudentQuestionDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let query = sql_query(
        r#"
        SELECT
            q.id,
            q.question,
            a.id "answer_id",
            a.answer
        FROM
            questions q
        INNER JOIN answers a ON
            a.question_id = q.id
        WHERE
            q.id = $1;
        "#,
    )
    .bind::<diesel::sql_types::Integer, _>(question_id);

    let results: Vec<Question> = query
        .get_results::<Question>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    if results.is_empty() {
        return Ok(None);
    }

    let mut question = GetStudentQuestionDto {
        id: results[0].id,
        question: results[0].question.clone(),
        answers: Vec::new(),
    };

    for result in results {
        question.answers.push(GetStudentAnswerDto {
            answer: result.answer.clone(),
            id: result.answer_id,
            marked: false,
        });
    }

    Ok(Some(question))
}

pub fn get_student_questions(
    exam_id: i32,
    user_id: i32,
) -> Result<Vec<GetStudentQuestionDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let query = sql_query(
        r#"
        SELECT
            q.id,
            q.question,
            a.id "answer_id",
            a.answer,
            sa.id IS NOT NULL "marked"
        FROM
            exams e
        INNER JOIN exam_questions eq ON
            eq.exam_id = e.id
        INNER JOIN questions q ON
            q.id = eq.question_id
        INNER JOIN answers a ON
            a.question_id = q.id
        LEFT JOIN student_answers sa ON
            sa.exam_id = e.id
            AND sa.question_id = q.id
            AND sa.answer_id = a.id
            AND sa.user_id = $2
        WHERE
            e.id = $1;
        "#,
    )
    .bind::<diesel::sql_types::Integer, _>(exam_id)
    .bind::<diesel::sql_types::Integer, _>(user_id);

    let results: Vec<Question> = query
        .get_results::<Question>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    let mut questions: Vec<GetStudentQuestionDto> = Vec::new();

    for result in results {
        let question = questions.iter_mut().find(|q| q.id == result.id);
        let question = match question {
            Some(q) => q,
            None => {
                let new_question = GetStudentQuestionDto {
                    id: result.id,
                    question: result.question.clone(),
                    answers: Vec::new(),
                };

                questions.push(new_question);
                questions.last_mut().unwrap()
            }
        };

        question.answers.push(GetStudentAnswerDto {
            answer: result.answer.clone(),
            id: result.answer_id,
            marked: result.marked,
        });
    }

    Ok(questions)
}

pub fn import_exam(exam: NewExam, questions: Vec<ImportQuestion>) -> Result<i32, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let exam_id: i32 = diesel::insert_into(crate::schema::exams::table)
        .values(&exam)
        .returning(crate::schema::exams::id)
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    for question in questions {
        let question_id: i32 = diesel::insert_into(crate::schema::questions::table)
            .values(&NewQuestion {
                question: &question.question,
            })
            .returning(crate::schema::questions::id)
            .get_result(&mut conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        for answer in question.answers {
            diesel::insert_into(answers::table)
                .values(&NewAnswer {
                    answer: answer.0,
                    is_correct: answer.1,
                    question_id,
                })
                .execute(&mut conn)
                .map_err(|_| ServiceError::InternalServerError)?;
        }

        diesel::insert_into(crate::schema::exam_questions::table)
            .values((
                crate::schema::exam_questions::exam_id.eq(exam_id),
                crate::schema::exam_questions::question_id.eq(question_id),
            ))
            .execute(&mut conn)
            .map_err(|_| ServiceError::InternalServerError)?;
    }

    Ok(exam_id)
}

pub fn get_student_finished_exams(
    uid: i32,
) -> Result<Vec<(i32, String, String, NaiveDateTime, NaiveDateTime)>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let current_time = Utc::now().naive_utc();

    let query = sql_query(
        r#"
       SELECT
            ce.exam_id, c."name" "class_name", e."name" "exam_name",  ce.start_time, ce.end_time
        FROM
            classes_students cs
        INNER JOIN class_exams ce ON
            ce.class_id = cs.class_id
        INNER JOIN classes c ON
            c.id = ce.class_id
        INNER JOIN exams e ON e.id = ce.exam_id 
        WHERE
            student_id = $1
            AND ce.end_time < $2;
        "#,
    )
    .bind::<diesel::sql_types::Integer, _>(uid)
    .bind::<diesel::sql_types::Timestamp, _>(current_time);

    let results: Vec<ExamId> = query
        .get_results::<ExamId>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(results
        .iter()
        .map(|e| {
            (
                e.exam_id,
                e.class_name.clone(),
                e.exam_name.clone(),
                e.start_time,
                e.end_time,
            )
        })
        .collect())
}

pub fn get_student_open_exams(
    uid: i32,
) -> Result<Vec<(i32, String, String, NaiveDateTime, NaiveDateTime)>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let current_time = Utc::now().naive_utc();

    let query = sql_query(
        r#"
        SELECT
            ce.exam_id, c."name" "class_name", e."name" "exam_name",  ce.start_time, ce.end_time
        FROM
            classes_students cs
        INNER JOIN class_exams ce ON
            ce.class_id = cs.class_id
        INNER JOIN classes c ON
            c.id = ce.class_id
        INNER JOIN exams e ON e.id = ce.exam_id 
        WHERE
            student_id = $1
            AND ce.end_time > $2;
        "#,
    )
    .bind::<diesel::sql_types::Integer, _>(uid)
    .bind::<diesel::sql_types::Timestamp, _>(current_time);

    let results: Vec<ExamId> = query
        .get_results::<ExamId>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(results
        .iter()
        .map(|e| {
            (
                e.exam_id,
                e.class_name.clone(),
                e.exam_name.clone(),
                e.start_time,
                e.end_time,
            )
        })
        .collect())
}
