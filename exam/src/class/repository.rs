use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{
    sql_query, ExpressionMethods, QueryableByName, RunQueryDsl, Selectable, SelectableHelper, Table,
};

use crate::schema;
use crate::{db::DB_MANAGER, errors::ServiceError, schema::classes_students};

use super::dto::{AddExamToClassDto, ClassExamDto};
use super::model::{Class, NewClass, UpdateClass};
use crate::diesel::OptionalExtension;
use crate::diesel::QueryDsl;
use crate::schema::classes::dsl::*;

pub fn create_class(new_class: NewClass) -> Result<Class, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let class: Class = diesel::insert_into(classes)
        .values(&new_class)
        .returning(classes::all_columns())
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(class)
}

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

pub fn get_class_by_id(class_id: i32) -> Result<Option<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let class = classes
        .filter(id.eq(class_id))
        .select(Class::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(class)
}

pub fn delete_class_by_id(class_id: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    diesel::delete(classes.filter(id.eq(class_id)))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

pub fn update_class(class_id: i32, update_class: UpdateClass) -> Result<Class, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let class = diesel::update(classes.filter(id.eq(class_id)))
        .set(&update_class)
        .returning(classes::all_columns())
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(class)
}

pub fn get_class_by_code(class_code: &str) -> Result<Option<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let class = classes
        .filter(code.eq(class_code))
        .select(Class::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(class)
}

pub fn enroll_student(cid: i32, sid: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    diesel::insert_into(classes_students::table)
        .values((
            classes_students::class_id.eq(cid),
            classes_students::student_id.eq(sid),
        ))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

pub fn is_student_enrolled(cid: i32, sid: i32) -> Result<bool, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let is_enrolled = classes_students::table
        .filter(classes_students::class_id.eq(cid))
        .filter(classes_students::student_id.eq(sid))
        .select(classes_students::columns::student_id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(is_enrolled.is_some())
}

pub fn list_classes_that_student_is_enrolled(sid: i32) -> Result<Vec<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let ccs: Vec<Class> = classes
        .inner_join(classes_students::table)
        .filter(classes_students::student_id.eq(sid))
        .select(Class::as_select())
        .load::<Class>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(ccs)
}

pub fn list_classes_that_student_is_not_enrolled(sid: i32) -> Result<Vec<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let ccs: Vec<Class> = classes
        .left_join(classes_students::table)
        .filter(classes_students::student_id.is_null())
        .or_filter(classes_students::student_id.ne(sid))
        .select(Class::as_select())
        .load::<Class>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(ccs)
}

pub fn list_classes_by_teacher(tid: i32) -> Result<Vec<Class>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let ccs: Vec<Class> = classes
        .filter(user_id.eq(tid))
        .select(Class::as_select())
        .load::<Class>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(ccs)
}
