use chrono::{NaiveDateTime, Utc};
use diesel::{sql_query, QueryableByName, RunQueryDsl};

use crate::{db::DB_MANAGER, errors::ServiceError};

#[derive(QueryableByName)]
struct ExamId {
    #[sql_type = "diesel::sql_types::Integer"]
    exam_id: i32,

    #[sql_type = "diesel::sql_types::Text"]
    class_name: String,

    #[sql_type = "diesel::sql_types::Timestamp"]
    start_time: NaiveDateTime,

    #[sql_type = "diesel::sql_types::Timestamp"]
    end_time: NaiveDateTime,
}

pub fn get_student_open_exams(
    uid: i32,
) -> Result<Vec<(i32, String, NaiveDateTime, NaiveDateTime)>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let current_time = Utc::now().naive_utc();

    let query = sql_query(
        r#"
       SELECT
            ce.exam_id, c."name" "class_name", ce.start_time, ce.end_time
        FROM
            classes_students cs
        INNER JOIN class_exams ce ON
            ce.class_id = cs.class_id
        INNER JOIN classes c ON
            c.id = ce.class_id
        WHERE
            student_id = $1
            AND ce.start_time < $2
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
        .map(|e| (e.exam_id, e.class_name.clone(), e.start_time, e.end_time))
        .collect())
}
