use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use reqwest::header::HeaderValue;

use crate::{auth::models::LoggedUser, errors::ServiceError, role::enm::RoleEnum};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S: 'static, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        
        let svc = self.service.clone();

        Box::pin(async move {
            let jwt = match req
                .headers()
                .get(AUTHORIZATION)
                .map(|val| val.to_str().ok())
                .flatten()
            {
                Some(jwt_str) => jwt_str.to_owned(),
                None => return Err(Error::from(ServiceError::Unauthorized)),
            };

            let gateway_url = match std::env::var("GATEWAY_URL") {
                Ok(url) => url,
                Err(_) => return Err(Error::from(ServiceError::InternalServerError)),
            };

            let client = reqwest::Client::new();

            let user_result = client
                .get(format!("{}/auth/me", gateway_url))
                .header(AUTHORIZATION.as_str(), HeaderValue::from_str(&jwt).unwrap())
                .send()
                .await;

            
            let response = match user_result {
                Ok(r) => r,
                Err(_) => return Err(Error::from(ServiceError::InternalServerError)),
            };
            

            let user: Option<LoggedUser> = match response.json().await {
                Ok(u) => u,
                Err(_) => return Err(Error::from(ServiceError::InternalServerError)),
            };

            

            let user = match user {
                Some(u) => u,
                None => return Err(Error::from(ServiceError::Unauthorized)),
            };

            req.extensions_mut().insert(user);

            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}

pub struct RoleMiddleware(pub Vec<RoleEnum>);

impl<S> Transform<S, ServiceRequest> for RoleMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = RoleAuthentication<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleAuthentication {
            service,
            role: self.0.clone(),
        }))
    }
}

pub struct RoleAuthentication<S> {
    service: S,
    role: Vec<RoleEnum>,
}

impl<S> Service<ServiceRequest> for RoleAuthentication<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let has_role;
        {
            let ext = req.extensions();
            let logged_user = ext.get::<LoggedUser>().unwrap();

            has_role = logged_user
                .roles
                .iter()
                .any(|role| self.role.contains(role))
                || logged_user.roles.contains(&RoleEnum::ADMIN);
        }

        let fut: <S as Service<ServiceRequest>>::Future = match has_role {
            true => self.service.call(req),
            false => {
                let res = req.error_response(ServiceError::Forbidden);
                return Box::pin(async { Ok(res) });
            }
        };

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
