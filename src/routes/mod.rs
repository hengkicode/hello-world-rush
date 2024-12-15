use actix_web::web;
use crate::controllers::stock::get_stock;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/stock", web::get().to(get_stock))
    );
}
