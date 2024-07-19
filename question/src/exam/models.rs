use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Exam {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = exams)]
pub struct NewExam {
    pub name: String,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = exams)]
pub struct UpdateExam {
    pub name: Option<String>,
}
