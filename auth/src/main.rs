use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use db::DB_MANAGER;
use dotenvy::dotenv;
use eureka::init_eureka;
use role::enm::RoleEnum::*;

mod auth;
mod avatar;
mod db;
mod errors;
mod eureka;
mod middleware;
mod role;
mod schema;
mod user;

extern crate diesel;
extern crate eureka_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let service_port: u16 = std::env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("port must be a number");

    DB_MANAGER.lock().unwrap().start_connection().await;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::resource("/health")
                    .to(|| async { HttpResponse::Ok().body("Service is up and running") }),
            )
            .service(
                web::resource("/me")
                    .wrap(middleware::AuthMiddleware)
                    .get(user::controller::me),
            )
            .service(
                web::scope("/users")
                    .service(web::resource("").post(user::controller::create_user))
                    .service(
                        web::resource("/{user_id}/roles")
                            .wrap(middleware::RoleMiddleware(vec![ADMIN]))
                            .wrap(middleware::AuthMiddleware)
                            .patch(user::controller::set_user_roles),
                    ),
            )
            .service(
                web::scope("/avatars")
                    .wrap(middleware::AuthMiddleware)
                    .service(
                        web::resource("")
                            .post(avatar::controller::update_user_avatar)
                            .delete(avatar::controller::delete_user_avatar),
                    ),
            )
            .service(web::resource("/login").post(auth::controller::login))
    })
    .bind(("0.0.0.0", service_port))?
    .run();

    let hostname = std::env::var("HOSTNAME").expect("HOSTNAME must be set");
    let eureka_url = std::env::var("EUREKA_HOSTNAME").expect("EUREKA_HOSTNAME must be set");
    let eureka_port: u16 = std::env::var("EUREKA_PORT")
        .expect("EUREKA_PORT must be set")
        .parse::<u16>()
        .expect("port must be a number");

    let eureka_client = init_eureka(eureka_url, eureka_port, hostname, service_port);

    let _ = server.await;

    drop(eureka_client);

    Ok(())
}
