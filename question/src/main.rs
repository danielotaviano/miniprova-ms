use actix_web::{middleware::Logger, web, App, HttpServer};
use db::DB_MANAGER;
use dotenvy::dotenv;
use eureka::init_eureka;
use role::enm::RoleEnum::*;

mod auth;
mod db;
mod errors;
mod eureka;
mod exam;
mod middleware;
mod question;
mod role;
mod schema;
extern crate diesel;

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
                web::scope("/questions")
                    .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                    .wrap(middleware::Authentication)
                    .service(
                        web::resource("/{question_id}")
                            .get(question::controller::get_question_by_id)
                            .delete(question::controller::delete_question_by_id)
                            .patch(question::controller::update_question_by_id),
                    )
                    .service(
                        web::resource("")
                            .post(question::controller::create_question)
                            .get(question::controller::list_questions),
                    ),
            )
            .service(
                web::scope("/exams")
                    .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                    .wrap(middleware::Authentication)
                    .service(
                        web::resource("")
                            .get(exam::controller::get_exams)
                            .post(exam::controller::create_exam),
                    )
                    .service(
                        web::resource("/{exam_id}")
                            .get(exam::controller::get_exam_by_id)
                            .delete(exam::controller::delete_exam)
                            .patch(exam::controller::update_exam),
                    )
                    .service(
                        web::resource("/{exam_id}/questions")
                            .post(exam::controller::update_questions_in_exam)
                            .get(exam::controller::get_exam_questions),
                    ),
            )
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
