use actix_web::{HttpResponse, Responder, web};
use crate::services::fetch_data::fetch_stock_data;
use serde::Serialize;

pub async fn get_stock() -> impl Responder {
    match fetch_stock_data().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body("Error fetching stock data")
        }
    }
}

// Definisikan struct yang akan diubah menjadi JSON
#[derive(Serialize)]
struct IpResponse {
    ip: String,
}

pub async fn insert_tmp_data_so(ip: web::Path<String>) -> impl Responder {
    // Membuat struktur IpResponse dengan IP yang diterima
    let response = IpResponse {
        ip: ip.into_inner(),
    };

    // Mengembalikan response dalam format JSON
    HttpResponse::Ok().json(response)
}