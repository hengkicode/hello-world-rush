use actix_web::{App, HttpServer};
mod controllers;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/stock", actix_web::web::get().to(controllers::stock::get_stock))
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
