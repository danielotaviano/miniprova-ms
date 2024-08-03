use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::auth::models::LoggedUser;

use super::{dto::AddExamToClassDto, service};

pub async fn add_exam_to_class(
    path: web::Path<i32>,
    req: HttpRequest,
    body: web::Json<AddExamToClassDto>,
) -> impl Responder {
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();
    let class_id = path.into_inner();

    match service::add_exam_to_class(class_id, body.into_inner(), user).await {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}
