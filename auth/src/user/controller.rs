use actix_web::{web, HttpMessage, HttpResponse, Responder};

use crate::{
    auth::models::LoggedUser,
    errors::ServiceError,
    role::enm::RoleEnum,
    user::{dto::CreateUserInputDto, service},
};

use super::dto::UserResponse;

pub async fn create_user(new_user: web::Json<CreateUserInputDto>) -> impl Responder {
    if let Err(e) = new_user.validate() {
        return HttpResponse::from_error(ServiceError::BadRequest(e));
    }

    let user = match service::create_user(new_user.into_inner()) {
        Ok(user) => user,
        Err(e) => return HttpResponse::from_error(e),
    };

    HttpResponse::Ok().json(user).into()
}

pub async fn set_user_roles(path: web::Path<i32>, roles: web::Json<Vec<String>>) -> impl Responder {
    let user_id = path.into_inner();
    let roles: Vec<RoleEnum> = roles.into_inner().iter().map(|role| role.into()).collect();

    if roles.contains(&RoleEnum::INVALID) {
        return HttpResponse::from_error(ServiceError::BadRequest("Invalid role".into()));
    }

    match service::set_user_roles(user_id, roles) {
        Ok(_) => HttpResponse::NoContent().into(),
        Err(e) => HttpResponse::from_error(e),
    }
}

pub async fn me(req: actix_web::HttpRequest) -> impl Responder {
    let ext = req.extensions();
    let user = match ext.get::<LoggedUser>() {
        Some(user) => service::get_user_with_roles_and_avatar_by_id(user.id).unwrap(),
        None => return HttpResponse::from_error(ServiceError::Unauthorized),
    };

    if user.is_none() {
        return HttpResponse::from_error(ServiceError::Unauthorized);
    }

    let user = user.unwrap();

    let formatted_user: UserResponse = UserResponse {
        avatar: user.avatar,
        email: user.email,
        id: user.id,
        name: user.name,
        roles: user.roles,
    };

    HttpResponse::Ok().json(formatted_user).into()
}

pub async fn list_users() -> impl Responder {
    match service::list_users() {
        Ok(users) => {
            let formatted_users: Vec<UserResponse> = users
                .into_iter()
                .map(|user| UserResponse {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    roles: user.roles,
                    avatar: user.avatar,
                })
                .collect();

            HttpResponse::Ok().json(formatted_users)
        }
        Err(e) => HttpResponse::from_error(e),
    }
}
