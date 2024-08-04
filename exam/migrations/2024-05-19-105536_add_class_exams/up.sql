-- Your SQL goes here
CREATE TABLE class_exams (
    id SERIAL PRIMARY KEY,
    class_id INT NOT NULL,
    exam_id INT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    UNIQUE (class_id, exam_id)
);