use std::time::Instant;

use actix_web::{error, get, web, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{AppState, SYNC_PERIOD};

#[derive(Serialize, Clone)]
pub struct Prices {
    pub prices: Vec<f32>,
    pub dates: Vec<String>,
}

#[derive(Deserialize)]
struct ApiPrices {
    license_info: String,
    unix_seconds: Vec<i64>,
    price: Vec<f32>,
    unit: String,
    deprecated: bool,
}

#[get("/price")]
pub async fn get_price(data: web::Data<AppState>) -> Result<web::Json<Prices>> {
    let mut price_sync = data
        .price_sync
        .lock()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;
    let now = Instant::now();

    if price_sync.last_sync + SYNC_PERIOD > now {
        info!("returning existing prices");
        return Ok(web::Json(price_sync.prices.clone()));
    }

    info!("requesting new prices");

    let text_response = reqwest::get("https://api.energy-charts.info/price?bzn=DE-LU")
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .text()
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let json = serde_json::from_str::<ApiPrices>(&text_response)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let mut dates = Vec::new();
    for date in json.unix_seconds {
        dates.push(
            DateTime::from_timestamp(date, 0)
                .ok_or_else(|| error::ErrorInternalServerError("date conversion failed"))?
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        );
    }

    let new_prices = Prices {
        prices: json.price,
        dates,
    };

    price_sync.prices = new_prices.clone();
    price_sync.last_sync = now;

    Ok(web::Json(new_prices))
}
