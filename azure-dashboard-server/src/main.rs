#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::azure_token_cache::{AccessTokenCache, AccessTokenCacheMap};
use crate::errors::AzureDashboardError;
use crate::settings::Settings;
use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

mod azure_token_cache;
mod errors;
mod settings;

#[get("/hello/{name}")]
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
    // Create a token cache map
    let token_caches = web::Data::new(AccessTokenCacheMap::new(settings.subscriptions()));
    // Start the Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(token_caches.clone())
            .route("/hello", web::get().to(|| async { "Hello world" }))
            .service(greet)
    })
    .bind(("127.0.0.1", settings.port()))?
    .run()
    .await
    // Map the std::io::Error to an anyhow::Error
    .map_err(anyhow::Error::from)
}
