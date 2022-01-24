use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv();
    tracing_subscriber::fmt().init();

    HttpServer::new(|| {
        App::new()
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
