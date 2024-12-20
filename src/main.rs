use std::sync::Mutex;
use std::time::{Duration, Instant};

use actix_web::web::{Data, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

mod index;
mod price;

use index::get_index;
use price::{get_price, Prices};

const SYNC_PERIOD: Duration = Duration::from_secs(3600);

struct PriceSync {
    last_sync: Instant,
    prices: Prices,
}

struct AppState {
    price_sync: Mutex<PriceSync>,
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_data = Data::new(AppState {
        price_sync: Mutex::new(PriceSync {
            last_sync: Instant::now() - 2 * SYNC_PERIOD,
            prices: Prices {
                prices: Vec::new(),
                dates: Vec::new(),
            },
        }),
    });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(app_data).service(get_index).service(get_price);
    };

    Ok(config.into())
}
