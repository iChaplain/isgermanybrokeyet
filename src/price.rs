use actix_web::{get, http::header::CONTENT_TYPE, HttpResponse};

#[get("/price")]
pub async fn get_price() -> HttpResponse {
    HttpResponse::Ok().append_header((CONTENT_TYPE, "application/json")).finish()
}
