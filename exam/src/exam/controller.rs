use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::{auth::models::LoggedUser, errors::ServiceError};

use super::service;

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
