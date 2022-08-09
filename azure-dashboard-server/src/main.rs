#![allow(dead_code)]
#![allow(unused_imports)]

use crate::settings::Settings;
use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

mod settings;

struct AccessTokenCache {
    counter: Mutex<i32>,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>, data: web::Data<AccessTokenCache>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Hello {name}! (counter = {counter})")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
    log::debug!(" - loading the configuration file");
    let settings = Settings::new().unwrap();
    log::debug!(" - loading settings = {:?}", settings);

    // Initialize the token cache
    let token_cache = web::Data::new(AccessTokenCache {
        counter: Mutex::new(0),
    });
    // Start the Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(token_cache.clone())
            .route("/hello", web::get().to(|| async { "Hello world" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
