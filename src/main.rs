mod routes;

use actix_web::{http::KeepAlive, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(routes::routes))
        .keep_alive(KeepAlive::Os)
        .shutdown_timeout(30)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
