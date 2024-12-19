use actix_web::{get, http::header::CONTENT_TYPE, HttpResponse};

#[get("/")]
pub async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .append_header((CONTENT_TYPE, "text/html; charset=utf-8"))
        .body(include_str!("main.html"))
}
