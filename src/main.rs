mod text_manipulations;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use env_logger;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(text_manipulations::text_manipulation)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
