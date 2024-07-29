use crate::schema::*;
use diesel::{deserialize::Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = class_exams)]
pub struct ClassExams {
    pub id: i32,
}
