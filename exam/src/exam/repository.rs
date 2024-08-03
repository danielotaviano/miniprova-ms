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
    exam_name: String,

    #[sql_type = "diesel::sql_types::Integer"]
    class_id: i32,

    #[sql_type = "diesel::sql_types::Timestamp"]
    start_time: NaiveDateTime,

    #[sql_type = "diesel::sql_types::Timestamp"]
    end_time: NaiveDateTime,
}

#[derive(QueryableByName)]
pub struct ExamById {
    #[sql_type = "diesel::sql_types::Integer"]
    id: i32,

    #[sql_type = "diesel::sql_types::Text"]
    name: String,
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
}

#[derive(QueryableByName)]
struct StudentQuestion {
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

#[derive(QueryableByName, Serialize)]
pub struct StudentQuestionAsTeacherResult {
    #[sql_type = "diesel::sql_types::Integer"]
    pub id: i32,

    #[sql_type = "diesel::sql_types::Integer"]
    pub score: i32,

    #[sql_type = "diesel::sql_types::Integer"]
    pub total_questions: i32,

    #[sql_type = "diesel::sql_types::Integer"]
    pub answered_questions: i32,
}

pub struct ImportQuestion {
    pub question: String,
    pub answers: Vec<(String, bool)>,
}

pub fn get_exam_results_as_teacher(
    exam_id: i32,
) -> Result<Vec<StudentQuestionAsTeacherResult>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let query = sql_query(
        r#"
        select
            user_id "id",
            count(distinct a.id) filter (
        where
            a.is_correct)::int4 "score",
            count(distinct eq.question_id)::int4 "total_questions",
            count(distinct sa.id)::int4 "answered_questions"
        from
            student_answers sa
        inner join answers a 
            on
            a.id = sa.answer_id
        inner join exam_questions eq on
            eq.exam_id = sa.exam_id
        where
            sa.exam_id = $1
        group by
            sa.user_id;
        "#,
    )
    .bind::<diesel::sql_types::Integer, _>(exam_id);
    println!("asdasdasdasdasd13 {:?}", query);

    let results: Vec<StudentQuestionAsTeacherResult> = query
        .get_results::<StudentQuestionAsTeacherResult>(&mut conn)
        .map_err(|e| {
            println!("error: {:?}", e);
            ServiceError::InternalServerError
        })?;

    Ok(results)
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

    let results: Vec<Question> = query.get_results::<Question>(&mut conn).map_err(|e| {
        println!("error: {:?}", e);
        ServiceError::InternalServerError
    })?;

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

    let results: Vec<StudentQuestion> = query
        .get_results::<StudentQuestion>(&mut conn)
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

pub fn get_classes_exams(
    uid: Vec<i32>,
) -> Result<Vec<(i32, String, i32, NaiveDateTime, NaiveDateTime)>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let current_time = Utc::now().naive_utc();

    let query = sql_query(
        r#"
        select
            ce.exam_id,
            e."name" "exam_name",
            ce.class_id,
            ce.start_time,
            ce.end_time
        from
            class_exams ce
        inner join exams e on
            e.id = ce.exam_id
        where
            ce.class_id = any($1);
        "#,
    )
    .bind::<diesel::sql_types::Array<diesel::sql_types::Integer>, _>(uid)
    .bind::<diesel::sql_types::Timestamp, _>(current_time);

    let results: Vec<ExamId> = query
        .get_results::<ExamId>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(results
        .iter()
        .map(|e| {
            (
                e.exam_id,
                e.exam_name.clone(),
                e.class_id,
                e.start_time,
                e.end_time,
            )
        })
        .collect())
}

pub fn get_classes_finished_exams(
    uid: Vec<i32>,
) -> Result<Vec<(i32, String, i32, NaiveDateTime, NaiveDateTime)>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let current_time = Utc::now().naive_utc();

    let query = sql_query(
        r#"
        select
            ce.exam_id,
            e."name" "exam_name",
            ce.class_id,
            ce.start_time,
            ce.end_time
        from
            class_exams ce
        inner join exams e on
            e.id = ce.exam_id
        where
            ce.class_id = any($1)
            and ce.end_time < $2;
        "#,
    )
    .bind::<diesel::sql_types::Array<diesel::sql_types::Integer>, _>(uid)
    .bind::<diesel::sql_types::Timestamp, _>(current_time);

    let results: Vec<ExamId> = query
        .get_results::<ExamId>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(results
        .iter()
        .map(|e| {
            (
                e.exam_id,
                e.exam_name.clone(),
                e.class_id,
                e.start_time,
                e.end_time,
            )
        })
        .collect())
}

pub fn get_classes_open_exams(
    uid: Vec<i32>,
) -> Result<Vec<(i32, String, i32, NaiveDateTime, NaiveDateTime)>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let current_time = Utc::now().naive_utc();

    let query = sql_query(
        r#"
        select
            ce.exam_id,
            e."name" "exam_name",
            ce.class_id,
            ce.start_time,
            ce.end_time
        from
            class_exams ce
        inner join exams e on
            e.id = ce.exam_id
        where
            ce.class_id = any($1)
            and ce.end_time > $2;
        "#,
    )
    .bind::<diesel::sql_types::Array<diesel::sql_types::Integer>, _>(uid)
    .bind::<diesel::sql_types::Timestamp, _>(current_time);

    let results: Vec<ExamId> = query
        .get_results::<ExamId>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(results
        .iter()
        .map(|e| {
            (
                e.exam_id,
                e.exam_name.clone(),
                e.class_id,
                e.start_time,
                e.end_time,
            )
        })
        .collect())
}

pub fn get_exam_by_id(exam_id: i32) -> Result<Option<ExamById>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let query = sql_query(
        r#"
        select
            e.id,
            e."name"
        from
            exams e
        where
            e.id = $1;

        "#,
    )
    .bind::<diesel::sql_types::Integer, _>(exam_id);

    let results: ExamById = query.get_result::<ExamById>(&mut conn).map_err(|e| {
        println!("error: {:?}", e);
        ServiceError::InternalServerError
    })?;

    Ok(Some(results))
}
