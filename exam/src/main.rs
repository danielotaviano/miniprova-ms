use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use db::DB_MANAGER;
use dotenvy::dotenv;
use eureka::init_eureka;
use role::enm::RoleEnum::*;

mod api;
mod auth;
mod class;
mod db;
mod errors;
mod eureka;
mod exam;
mod middleware;
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
                web::resource("/health")
                    .to(|| async { HttpResponse::Ok().body("Service is up and running") }),
            )
            .service(
                web::scope("/exams")
                    .wrap(middleware::Authentication)
                    .service(
                        web::resource("/student/open")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .route(web::get().to(exam::controller::get_student_open_exams)),
                    )
                    .service(
                        web::resource("/student/finished")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .route(web::get().to(exam::controller::get_student_finished_exams)),
                    )
                    .service(
                        web::resource("/student/exam/{exam_id}/questions")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .route(web::get().to(exam::controller::get_student_questions)),
                    )
                    .service(
                        web::resource("/student/{exam_id}/question/{question_id}/submit")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .post(exam::controller::submit_answer_to_question_in_exam),
                    )
                    .service(
                        web::resource("/student/{exam_id}/results")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .get(exam::controller::get_student_exam_result),
                    ),
            )
            .service(
                web::scope("/classes")
                    .wrap(middleware::Authentication)
                    .service(
                        web::resource("")
                            .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                            .post(class::controller::create_class),
                    )
                    .service(
                        web::resource("/students/enrolled")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .route(
                                web::get()
                                    .to(class::controller::list_classes_that_student_is_enrolled),
                            ),
                    )
                    .service(
                        web::resource("/students/unenrolled")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .route(
                                web::get().to(
                                    class::controller::list_classes_that_student_is_not_enrolled,
                                ),
                            ),
                    )
                    .service(
                        web::resource("/teachers")
                            .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                            .route(web::get().to(class::controller::list_classes_by_teacher)),
                    )
                    .service(
                        web::resource("/{class_id}")
                            .route(
                                web::get()
                                    .to(class::controller::get_class_by_id)
                                    .wrap(middleware::Authentication),
                            )
                            .route(
                                web::patch()
                                    .to(class::controller::update_class)
                                    .wrap(middleware::RoleMiddleware(vec![TEACHER])),
                            )
                            .route(
                                web::delete()
                                    .to(class::controller::delete_class)
                                    .wrap(middleware::RoleMiddleware(vec![TEACHER])),
                            ),
                    )
                    .service(
                        web::resource("/{class_id}/exams")
                            .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                            .route(web::post().to(class::controller::add_exam_to_class)),
                    )
                    .service(
                        web::resource("/{class_id}/enroll")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .route(web::post().to(class::controller::enroll_student)),
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
