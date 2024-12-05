use actix_files::Files;  // Untuk melayani file statis seperti CSS
use actix_web::{web, App, HttpResponse, HttpServer};
use tera::{Tera, Context};

#[derive(serde::Serialize)]
struct ApiResponse {
    message: String,
}

// API handler
async fn api_hello() -> HttpResponse {
    let response = ApiResponse {
        message: "Hello, this is a RESTful API!".to_string(),
    };
    HttpResponse::Ok().json(response)  // Mengirimkan response dalam format JSON
}

// Handler untuk halaman aplikasi dengan template
async fn get_app(tera: web::Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("title", "Welcome to the Application");

    match tera.render("app.html", &context) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(_) => HttpResponse::InternalServerError().body("Template rendering error."),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Memuat template Tera
    let tera = Tera::new("templates/**/*").unwrap();

    // Cetak pesan bahwa server berjalan dan dapat diakses secara publik
    println!("Server running at http://0.0.0.0:8082 (public)");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))  // Menyediakan template Tera ke seluruh aplikasi
            .route("/api/hello", web::get().to(api_hello))  // RESTful API endpoint
            .route("/app", web::get().to(get_app))   // Route untuk halaman aplikasi
            .service(Files::new("/static", "./static"))  // Melayani file statis seperti CSS dan JS
    })
    .bind(("0.0.0.0", 8082))?  // Bind ke 0.0.0.0 agar dapat diakses secara publik
    .run()
    .await
}