// @generated automatically by Diesel CLI.

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

diesel::joinable!(class_exams -> classes (class_id));
diesel::joinable!(classes_students -> classes (class_id));

diesel::allow_tables_to_appear_in_same_query!(
    class_exams,
    classes,
    classes_students,
);
