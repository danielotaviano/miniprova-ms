use chrono::{NaiveDateTime, Utc};
use diesel::{sql_query, QueryableByName, RunQueryDsl};

use crate::{db::DB_MANAGER, errors::ServiceError, schema::answers};

use crate::diesel::ExpressionMethods;

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

pub struct ImportQuestion {
    pub question: String,
    pub answers: Vec<(String, bool)>,
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
