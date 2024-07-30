use crate::schema::*;
use diesel::{deserialize::Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = class_exams)]
pub struct ClassExams {
    pub id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = exams)]

pub struct NewExam {
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion<'a> {
    pub question: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = answers)]
pub struct NewAnswer {
    pub answer: String,
    pub is_correct: bool,
    pub question_id: i32,
}
