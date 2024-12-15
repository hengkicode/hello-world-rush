use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Error;
use odbc_api::{buffers::TextRowSet, ConnectionOptions, Environment, Cursor, ResultSetMetadata};
use std::collections::HashMap;
use std::env;

/// Maksimum jumlah baris yang diambil dalam satu batch
const BATCH_SIZE: usize = 5000;

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

/// Fungsi untuk mengambil data dari database SQL Server dengan kolom dinamis
async fn fetch_stock_data() -> Result<Vec<HashMap<String, String>>, Error> {
    dotenv::dotenv().ok();
    let connection_string = env::var("DATABASE_URL")
        .expect("DATABASE_URL harus diatur di file .env");
    let query = env::var("QUERY").unwrap_or("SELECT top 1000 * FROM stock".to_string());

    let environment = Environment::new()?;
    let connection = environment.connect_with_connection_string(
        &connection_string,
        ConnectionOptions::default(),
    )?;

    let mut results = Vec::new();
    if let Some(mut cursor) = connection.execute(&query, ())? {
        // Get column names before binding the buffer
      // Assuming this is part of your fetch_stock_data function
    let column_names: Vec<String> = cursor.column_names()?
    .map(|name_result| name_result.map(|name| name.to_string()))
    .collect::<Result<Vec<String>, _>>()?;


        let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
        let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

        while let Some(batch) = row_set_cursor.fetch()? {
            println!("Fetched {} rows", batch.num_rows());
            for row_index in 0..batch.num_rows() {
                let mut row_data = HashMap::new();
                for (col_index, col_name) in column_names.iter().enumerate() {
                    let col_value = String::from_utf8_lossy(batch.at(col_index, row_index).unwrap_or(&[])).to_string();
                    row_data.insert(col_name.clone(), col_value);
                }
                results.push(row_data);
            }
        }
    }

    println!("Total rows fetched: {}", results.len());
    Ok(results)
}

/// Fungsi utama untuk menjalankan server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Memulai server Actix Web
    HttpServer::new(|| {
        App::new().route("/stock", web::get().to(get_stock))
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
