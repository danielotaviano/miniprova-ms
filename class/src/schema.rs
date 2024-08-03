// @generated automatically by Diesel CLI.

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

diesel::joinable!(classes_students -> classes (class_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    classes_students,
);
