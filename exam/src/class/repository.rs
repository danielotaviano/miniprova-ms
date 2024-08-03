use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{sql_query, ExpressionMethods, QueryableByName, RunQueryDsl};

use crate::schema;
use crate::{db::DB_MANAGER, errors::ServiceError};

use super::dto::ClassExamDto;
use crate::diesel::OptionalExtension;

pub fn add_exam_to_class(
    cid: i32,
    exam_id: i32,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    diesel::insert_into(schema::class_exams::table)
        .values((
            schema::class_exams::class_id.eq(cid),
            schema::class_exams::exam_id.eq(exam_id),
            schema::class_exams::start_time.eq(start_date.naive_utc()),
            schema::class_exams::end_time.eq(end_date.naive_utc()),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            println!("Error: {:?}", e);
            ServiceError::InternalServerError
        })?;

    Ok(())
}

#[derive(QueryableByName, Debug)]
struct ClassExam {
    #[sql_type = "diesel::sql_types::Integer"]
    class_id: i32,
    #[sql_type = "diesel::sql_types::Integer"]
    exam_id: i32,
    #[sql_type = "diesel::sql_types::Timestamp"]
    start_time: NaiveDateTime,
    #[sql_type = "diesel::sql_types::Timestamp"]
    end_time: NaiveDateTime,
}

pub fn get_class_exam(exam_id: i32) -> Result<Option<ClassExamDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let query = sql_query(
        r#"
       SELECT
            ce.class_id,
            ce.exam_id,
            ce.start_time,
            ce.end_time
        FROM
            class_exams ce
        WHERE
            exam_id = $1
        LIMIT 1;
        "#,
    )
    .bind::<diesel::sql_types::Integer, _>(exam_id);

    println!("{:?}", query);
    let class_exam: Option<ClassExam> = query
        .get_result(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    println!("{:?}", class_exam);

    let class_exam = match class_exam {
        Some(ce) => {
            let dto = ClassExamDto {
                class_id: ce.class_id,
                exam_id: ce.exam_id,
                start_time: ce.start_time,
                end_time: ce.end_time,
            };
            Ok(Some(dto))
        }
        None => Ok(None),
    };

    class_exam
}
