use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod session;
mod data;

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
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".into());
    let port = std::env::var("PORT")
        .unwrap_or("8080".into())
        .parse()
        .unwrap_or(8080);
    let path_prefix = std::env!("CARGO_PKG_VERSION");

    HttpServer::new(|| {
        App::new().service(web::scope(path_prefix).service(semesters).service(courses))
    })
    .bind((host, port))?
    .run()
    .await
}
