use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::{auth::models::LoggedUser, errors::ServiceError};

use super::{dto::StudentAnswerInputDto, service};

pub async fn get_student_open_exams(req: HttpRequest) -> impl Responder {
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::get_student_open_exams(user.id).await {
        Err(e) => return HttpResponse::from_error(e),
        Ok(exams) => HttpResponse::Ok().json(exams).into(),
    }
}

pub async fn get_student_finished_exams(req: HttpRequest) -> impl Responder {
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::get_student_finished_exams(user.id).await {
        Err(e) => return HttpResponse::from_error(e),
        Ok(exams) => HttpResponse::Ok().json(exams).into(),
    }
}

pub async fn get_student_questions(req: HttpRequest, path: web::Path<i32>) -> impl Responder {
    let exam_id = path.into_inner();
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::get_questions_as_student(exam_id, user.id).await {
        Err(e) => return HttpResponse::from_error(e),
        Ok(questions) => HttpResponse::Ok().json(questions).into(),
    }
}

pub async fn get_student_exam_result(req: HttpRequest, path: web::Path<i32>) -> impl Responder {
    let exam_id = path.into_inner();
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::get_student_exam_result(user.id, exam_id).await {
        Err(e) => return HttpResponse::from_error(e),
        Ok(result) => HttpResponse::Ok().json(result).into(),
    }
}

pub async fn submit_answer_to_question_in_exam(
    path: web::Path<(i32, i32)>,
    req: HttpRequest,
    input: web::Json<StudentAnswerInputDto>,
) -> impl Responder {
    let (exam_id, question_id) = path.into_inner();

    match input.validate() {
        Err(e) => return HttpResponse::from_error(ServiceError::BadRequest(e)),
        _ => (),
    }

    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::submit_answer_to_question_in_exam(
        user.id,
        exam_id,
        question_id,
        input.into_inner().answer_id,
    ) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}
