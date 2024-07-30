use crate::{
    api::{self, GetExamApi},
    auth::models::LoggedUser,
    errors::ServiceError,
    exam::{self, model::NewExam, repository::ImportQuestion},
};

use super::{
    dto::{AddExamToClassDto, CreateClassInputDto, UpdateClassInputDto},
    model::{Class, NewClass, UpdateClass},
    repository,
};

pub fn create_class(user_id: i32, class: CreateClassInputDto) -> Result<Class, ServiceError> {
    let already_exists = repository::get_class_by_code(&class.code)?;

    if already_exists.is_some() {
        return Err(ServiceError::BadRequest(
            "Class with this code already exists".to_string(),
        ));
    }

    let new_class = repository::create_class(NewClass {
        code: &class.code,
        description: &class.description,
        name: &class.name,
        user_id,
    })?;

    Ok(new_class)
}

pub fn get_class_by_id(class_id: i32) -> Result<Option<Class>, ServiceError> {
    let class = repository::get_class_by_id(class_id)?;

    Ok(class)
}

pub fn delete_class_by_id(user_id: i32, class_id: i32) -> Result<(), ServiceError> {
    let class = repository::get_class_by_id(class_id)?;

    if class.is_none() {
        return Err(ServiceError::BadRequest("Class not found".to_string()));
    }

    if class.unwrap().user_id != user_id {
        return Err(ServiceError::Forbidden);
    }

    repository::delete_class_by_id(class_id)?;

    Ok(())
}

pub fn update_class(
    user_id: i32,
    class_id: i32,
    class: UpdateClassInputDto,
) -> Result<Class, ServiceError> {
    let existing = repository::get_class_by_id(class_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Class not found".to_string()));
    }

    if existing.unwrap().user_id != user_id {
        return Err(ServiceError::Forbidden);
    }

    let updated_class = repository::update_class(
        class_id,
        UpdateClass {
            description: class.description,
            name: class.name,
        },
    )?;

    Ok(updated_class)
}

pub fn enroll_student(class_id: i32, student_id: i32) -> Result<(), ServiceError> {
    let class = repository::get_class_by_id(class_id)?;

    if class.is_none() {
        return Err(ServiceError::BadRequest("Class not found".to_string()));
    }

    let is_student_already_enrolled = repository::is_student_enrolled(class_id, student_id)?;

    if is_student_already_enrolled {
        return Err(ServiceError::BadRequest(
            "Student already enrolled".to_string(),
        ));
    }

    repository::enroll_student(class_id, student_id)?;

    Ok(())
}

pub fn list_classes_that_student_is_enrolled(student_id: i32) -> Result<Vec<Class>, ServiceError> {
    let classes = repository::list_classes_that_student_is_enrolled(student_id)?;

    Ok(classes)
}

pub fn list_classes_that_student_is_not_enrolled(
    student_id: i32,
) -> Result<Vec<Class>, ServiceError> {
    let classes = repository::list_classes_that_student_is_not_enrolled(student_id)?;

    Ok(classes)
}

pub fn list_classes_by_teacher(teacher_id: i32) -> Result<Vec<Class>, ServiceError> {
    let classes = repository::list_classes_by_teacher(teacher_id)?;

    Ok(classes)
}

pub fn is_class_teacher(user_id: i32, class_id: i32) -> Result<bool, ServiceError> {
    let class = repository::get_class_by_id(class_id)?;

    if class.is_none() {
        return Err(ServiceError::BadRequest("Class not found".to_string()));
    }

    Ok(class.unwrap().user_id == user_id)
}

pub fn is_student_enrolled(class_id: i32, student_id: i32) -> Result<bool, ServiceError> {
    let is_enrolled = repository::is_student_enrolled(class_id, student_id)?;

    Ok(is_enrolled)
}

pub async fn add_exam_to_class(
    class_id: i32,
    exam: AddExamToClassDto,
    user: &LoggedUser,
) -> Result<(), ServiceError> {
    let class = repository::get_class_by_id(class_id)?;

    if class.is_none() {
        return Err(ServiceError::BadRequest("Class not found".to_string()));
    }

    let api_exam = api::get_exam(exam.exam_id, user.jwt.clone()).await?;
    println!("{:?}", api_exam);
    let questions = api::get_exam_questions(api_exam.id, user.jwt.clone()).await?;
    println!("{:?}", questions);

    let formated_questions: Vec<ImportQuestion> = questions
        .iter()
        .map(|q| ImportQuestion {
            question: q.question.clone(),
            answers: q
                .answers
                .iter()
                .map(|a| (a.answer.clone(), a.is_correct))
                .collect(),
        })
        .collect();

    let db_exam_id = exam::repository::import_exam(
        NewExam {
            name: api_exam.name,
        },
        formated_questions,
    )?;

    repository::add_exam_to_class(class_id, db_exam_id, exam.start_date, exam.end_date)?;

    Ok(())
}
