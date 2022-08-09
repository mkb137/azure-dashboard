#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::azure_token_cache::{AccessTokenCache, AccessTokenCacheMap};
use crate::errors::AzureDashboardError;
use crate::settings::Settings;
use crate::static_file_handlers::static_file;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use std::sync::Mutex;

mod azure_token_cache;
mod errors;
mod routes;
mod settings;
mod static_file_handlers;

#[get("/api/hello/{name}")]
async fn greet(
    name: web::Path<String>,
    data: web::Data<AccessTokenCacheMap>,
) -> Result<String, AzureDashboardError> {
    log::debug!("greet - name = {name}");
    let name_value = name.into_inner();
    let access_token = data
        .access_token(name_value.clone())
        .await
        .map_err(move |e| AzureDashboardError::CouldNotGetAccessToken(name_value))?;
    Ok(access_token)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
    log::debug!(" - loading the configuration file");
    let settings = Settings::new().unwrap();
    log::debug!(" - loading settings = {:?}", settings);
    // Save the host and port
    let host = settings.host();
    let port = settings.port();
    // Create a token cache map as web data
    let token_caches = web::Data::new(AccessTokenCacheMap::new(settings.subscriptions()));
    // Make the settings available as web data
    let settings_data = web::Data::new(settings);
    // Start the Actix server
    HttpServer::new(move || {
        App::new()
            // Make the token cache map available to all routes
            .app_data(token_caches.clone())
            // Make the settings available to all routes
            .app_data(settings_data.clone())
            // Add API routes
            .route("/hello", web::get().to(|| async { "Hello world" }))
            .service(greet)
            // Add static file handling
            .route("/{filename:.*.*}", web::get().to(static_file))
    })
    .bind((host, port))?
    .run()
    .await
    // Map the std::io::Error to an anyhow::Error
    .map_err(anyhow::Error::from)
}
