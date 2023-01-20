use std::error::Error;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

mod config;
mod data;
mod session;

use config::DualisCredentials;
use data::query::CoursesQuery;
use log::{error, info};
use session::DualisSession;

#[get("/semesters")]
async fn semesters(cred: web::Data<DualisCredentials>) -> impl Responder {
    let session =
        match DualisSession::log_into_dualis(cred.url(), cred.usrname(), cred.pass()).await {
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
async fn courses(
    cred: web::Data<DualisCredentials>,
    query: web::Query<CoursesQuery>,
) -> impl Responder {
    let session =
        match DualisSession::log_into_dualis(cred.url(), cred.usrname(), cred.pass()).await {
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error during login process: '{}'", e))
            }
            Ok(ses) => ses,
        };

    let courses = match session.get_courses(&query.semester_id).await {
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!(
                "Failed to fetch courses for given semester '{}': {}",
                query.semester_id, e
            ))
        }
        Ok(c) => c,
    };

    HttpResponse::Ok().body(format!("Courses: {:?}", courses))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Actix web server starting...");

    let hostname = DualisCredentials::get_hostname();
    let port = DualisCredentials::get_port();
    let root_path = DualisCredentials::get_root_path();
    let credentials = match DualisCredentials::from_env() {
        Err(e) => {
            error!("Could not retrieve DualisCredentials from environment variables...");
            return Err(e.into());
        }
        Ok(c) => c,
    };
    info!("Loaded server config. Hostname: {hostname}, Port: {port}, Root path: {root_path}");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(credentials.clone()))
            .service(web::scope(&root_path).service(semesters).service(courses))
    })
    .bind((hostname, port))?
    .run()
    .await?;

    Ok(())
}
