use actix_web::{HttpResponse, Responder};
use crate::services::fetch_data::fetch_stock_data;

pub async fn get_stock() -> impl Responder {
    match fetch_stock_data().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body("Error fetching stock data")
        }
    }
}
