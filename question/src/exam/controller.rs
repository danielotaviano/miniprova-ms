use actix_web::{web, HttpResponse, Responder};

use crate::errors::ServiceError;

use super::{dto::CreateExamInputDto, models::UpdateExam, service};

pub async fn create_exam(input: web::Json<CreateExamInputDto>) -> impl Responder {
    match input.validate() {
        Err(e) => return HttpResponse::from_error(ServiceError::BadRequest(e)),
        _ => (),
    }

    let exam = match service::create_exam(input.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(exam) => exam,
    };

    HttpResponse::Created().json(exam).into()
}

pub async fn get_exam_by_id(path: web::Path<i32>) -> impl Responder {
    let exam_id = path.into_inner();
    match service::get_exam_by_id(exam_id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(e) => match e {
            None => HttpResponse::NotFound().into(),
            Some(ex) => HttpResponse::Ok().json(ex).into(),
        },
    }
}

pub async fn delete_exam(path: web::Path<i32>) -> impl Responder {
    let exam_id = path.into_inner();

    match service::delete_exam(exam_id) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}

pub async fn update_exam(path: web::Path<i32>, input: web::Json<UpdateExam>) -> impl Responder {
    let exam_id = path.into_inner();

    let exam = match service::update_exam(exam_id, input.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(exam) => exam,
    };

    HttpResponse::Ok().json(exam).into()
}

pub async fn update_questions_in_exam(
    path: web::Path<i32>,
    input: web::Json<Vec<i32>>,
) -> impl Responder {
    let exam_id = path.into_inner();

    match service::update_questions_in_exam(exam_id, input.into_inner()) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}

pub async fn get_exam_questions(path: web::Path<i32>) -> impl Responder {
    let exam_id = path.into_inner();

    match service::get_exam_questions(exam_id) {
        Err(e) => HttpResponse::from_error(e),
        Ok(questions) => HttpResponse::Ok().json(questions).into(),
    }
}
