use actix_web::{App, HttpServer};
mod controllers;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/stock", actix_web::web::get().to(controllers::stock::get_stock))
            .route("/insertTmpDataSO/{ip}", actix_web::web::post().to(controllers::stock::insert_tmp_data_so))
    })
    .bind("0.0.0.0:8083")? // Dengarkan koneksi dari semua antarmuka jaringan
    .run()
    .await
}
