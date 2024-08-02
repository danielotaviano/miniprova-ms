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
    class_exams (id) {
        id -> Int4,
        class_id -> Int4,
        exam_id -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
    }
}

diesel::table! {
    classes (id) {
        id -> Int4,
        name -> Varchar,
        code -> Varchar,
        description -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    classes_students (class_id, student_id) {
        class_id -> Int4,
        student_id -> Int4,
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

diesel::table! {
    student_answers (id) {
        id -> Int4,
        user_id -> Int4,
        exam_id -> Int4,
        question_id -> Int4,
        answer_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(class_exams -> classes (class_id));
diesel::joinable!(class_exams -> exams (exam_id));
diesel::joinable!(classes_students -> classes (class_id));
diesel::joinable!(exam_questions -> exams (exam_id));
diesel::joinable!(exam_questions -> questions (question_id));
diesel::joinable!(student_answers -> answers (answer_id));
diesel::joinable!(student_answers -> exams (exam_id));
diesel::joinable!(student_answers -> questions (question_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    class_exams,
    classes,
    classes_students,
    exam_questions,
    exams,
    questions,
    student_answers,
);
