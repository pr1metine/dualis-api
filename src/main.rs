use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod session;
use session::DualisSession;

#[get("/semesters")]
async fn hello() -> impl Responder {
    let session = DualisSession::log_into_dualis().await;
    if let Err(e) = session {
        return HttpResponse::InternalServerError().body(format!("Error during login process: '{}'", e));
    }

    let session = session.unwrap();

    let semesters = session.get_semesters().await;
    if let Err(e) = semesters {
        return HttpResponse::InternalServerError().body(format!("Failed to fetch available semesters: '{}'", e));
    }
    let semesters = semesters.unwrap();

    HttpResponse::Ok().body(format!("Semesters: {:?}", semesters))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".into());
    let port = std::env::var("PORT")
        .unwrap_or("8080".into())
        .parse()
        .unwrap_or(8080);
    let path_prefix = std::env!("CARGO_PKG_VERSION");

    HttpServer::new(|| App::new().service(web::scope(path_prefix).service(hello)))
        .bind((host, port))?
        .run()
        .await
}
