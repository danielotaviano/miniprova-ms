// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        answer -> Varchar,
        is_correct -> Bool,
        question_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    exam_questions (exam_id, question_id) {
        exam_id -> Int4,
        question_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    exams (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        question -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(exam_questions -> exams (exam_id));
diesel::joinable!(exam_questions -> questions (question_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    exam_questions,
    exams,
    questions,
);
