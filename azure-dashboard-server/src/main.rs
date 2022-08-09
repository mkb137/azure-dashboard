#![allow(dead_code)]
#![allow(unused_imports)]

use crate::settings::Settings;
use actix_web::{get, web, App, HttpServer, Responder};

mod settings;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
    log::debug!(" - loading the configuration file");
    let settings = Settings::new().unwrap();
    log::debug!(" - loading settings = {:?}", settings);

    // Start the Actix server
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello world" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
