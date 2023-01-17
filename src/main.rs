use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

mod session;
mod data;

use log::info;
use session::DualisSession;
use data::query::CoursesQuery;

#[get("/semesters")]
async fn semesters() -> impl Responder {
    let session = match DualisSession::log_into_dualis().await {
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error during login process: '{}'", e))
        }
        Ok(ses) => ses,
    };

    let semesters = match session.get_semesters().await {
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to fetch available semesters: '{}'", e))
        }
        Ok(sem) => sem,
    };

    HttpResponse::Ok().body(format!("Semesters: {:?}", semesters))
}

#[get("/courses")]
async fn courses(query: web::Query<CoursesQuery>) -> impl Responder {
    let session = match DualisSession::log_into_dualis().await {
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error during login process: '{}'", e))
        }
        Ok(ses) => ses,
    };

    let courses = match session.get_courses(&query.semester_id).await {
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to fetch courses for given semester '{}': {}", query.semester_id, e))
        }
        Ok(c) => c
    };

    HttpResponse::Ok().body(format!("Courses: {:?}", courses))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Actix web server starting...");
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".into());
    let port = std::env::var("PORT")
        .unwrap_or("8080".into())
        .parse()
        .unwrap_or(8080);
    let path_prefix = std::env!("CARGO_PKG_VERSION");
    info!("Host: {host}");
    info!("Port: {port}");
    info!("Path prefix: {path_prefix}");

    HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(web::scope(path_prefix).service(semesters).service(courses))
    })
    .bind((host, port))?
    .run()
    .await
}
