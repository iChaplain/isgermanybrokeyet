use actix_web::{get, http::header::CONTENT_TYPE, web::ServiceConfig, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> HttpResponse {
    HttpResponse::Ok()
        .append_header((CONTENT_TYPE, "text/html; charset=utf-8"))
        .body("<html><body><center><h1>Not sure yet</h1></center></body></html>")
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}
