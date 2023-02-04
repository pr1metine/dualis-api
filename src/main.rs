use std::error::Error;
use std::time::Duration;

use actix_rt::time;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

mod config;
mod data;
mod error;
mod session;

use config::DualisCredentials;
use data::query::CoursesQuery;
use log::{error, info};
use session::DualisSession;

use crate::data::overview::PerformanceOverview;
use crate::data::{DHBWCourse, DHBWSemester};

#[get("/semesters")]
async fn semesters(cred: web::Data<DualisCredentials>) -> impl Responder {
    async fn body(
        cred: web::Data<DualisCredentials>,
    ) -> Result<Vec<DHBWSemester>, Box<dyn std::error::Error>> {
        let session =
            DualisSession::log_into_dualis(cred.url(), cred.usrname(), cred.pass()).await?;
        let semesters = session.get_semesters().await?;

        Ok(semesters)
    }

    match body(cred).await {
        Ok(semesters) => HttpResponse::Ok().body(format!("Semesters: {:?}", semesters)),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

#[get("/courses")]
async fn courses(
    cred: web::Data<DualisCredentials>,
    query: web::Query<CoursesQuery>,
) -> impl Responder {
    async fn body(
        cred: web::Data<DualisCredentials>,
        query: web::Query<CoursesQuery>,
    ) -> Result<Vec<DHBWCourse>, Box<dyn std::error::Error>> {
        let session =
            DualisSession::log_into_dualis(cred.url(), cred.usrname(), cred.pass()).await?;
        let courses = session.get_courses(&query.semester_id).await?;

        Ok(courses)
    }

    match body(cred, query).await {
        Ok(courses) => HttpResponse::Ok().body(format!("Courses: {:?}", courses)),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

#[get("/overview")]
async fn overview(data: web::Data<PerformanceOverview>) -> impl Responder {
    let data = data.read();
    HttpResponse::Ok().json(&(*data))
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
        Ok(c) => web::Data::new(c),
    };
    let shared_overview = web::Data::new(PerformanceOverview::new());
    info!("Loaded server config. Hostname: {hostname}, Port: {port}, Root path: {root_path}, User: {}", credentials.usrname());

    let other_credentials = credentials.clone();
    let other_overview = shared_overview.clone();
    actix_rt::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        // let other_overview = other_overview.clone();
        let credentials = other_credentials.clone();
        let session = DualisSession::log_into_dualis(
            credentials.url(),
            credentials.usrname(),
            credentials.pass(),
        ).await.unwrap();

        loop {
            interval.tick().await;
            let next_courses = session.get_overview().await.unwrap();
            other_overview.write().set_courses(next_courses);
            // println!("Hello, World for the {counter}. time!");
        }
    });

    let other_overview = shared_overview.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(credentials.clone())
            .app_data(other_overview.clone())
            .service(
                web::scope(&root_path)
                    .service(semesters)
                    .service(courses)
                    .service(overview), // .service(test)
            )
    })
    .bind((hostname, port))?
    .run()
    .await?;

    Ok(())
}
