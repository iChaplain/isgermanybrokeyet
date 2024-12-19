use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

mod index;
mod price;

use index::get_index;
use price::get_price;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(get_index)
        .service(get_price);
    };

    Ok(config.into())
}
