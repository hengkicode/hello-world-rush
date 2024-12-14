use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Error;
use odbc_api::{buffers::TextRowSet, ConnectionOptions, Environment, Cursor};
use serde::Serialize;
use std::env;

/// Maksimum jumlah baris yang diambil dalam satu batch
const BATCH_SIZE: usize = 5000;

/// Struct untuk representasi JSON
#[derive(Serialize)]
struct Stock {
    bara: String,
    nama: String,
}

/// Handler untuk endpoint API
async fn get_stock() -> impl Responder {
    match fetch_stock_data().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body("Error fetching stock data")
        }
    }
}

/// Fungsi untuk mengambil data dari database SQL Server
async fn fetch_stock_data() -> Result<Vec<Stock>, Error> {
    // Memuat variabel lingkungan dari file .env
    dotenv::dotenv().ok();

    // Ambil connection string dari variabel lingkungan
    let connection_string = env::var("DATABASE_URL")
        .expect("DATABASE_URL harus diatur di file .env");
    let query = env::var("QUERY").unwrap_or("SELECT bara, nama FROM stock".to_string());

    // Inisialisasi lingkungan ODBC
    let environment = Environment::new()?;
    let connection = environment.connect_with_connection_string(
        &connection_string,
        ConnectionOptions::default(),
    )?;

    // Eksekusi query dan proses hasilnya
    let mut results = Vec::new();
    if let Some(mut cursor) = connection.execute(&query, ())? {
        let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
        let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

        while let Some(batch) = row_set_cursor.fetch()? {
            for row_index in 0..batch.num_rows() {
                let bara = String::from_utf8_lossy(batch.at(0, row_index).unwrap_or(&[])).to_string();
                let nama = String::from_utf8_lossy(batch.at(1, row_index).unwrap_or(&[])).to_string();
                results.push(Stock { bara, nama });
            }
        }
    }

    Ok(results)
}

/// Fungsi utama untuk menjalankan server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Memulai server Actix Web
    HttpServer::new(|| {
        App::new().route("/stock", web::get().to(get_stock))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
