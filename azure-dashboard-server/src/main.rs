#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::azure_token_cache::{AccessTokenCache, AccessTokenCacheMap};
use crate::settings::Settings;
use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

mod azure_token_cache;
mod settings;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>, data: web::Data<AccessTokenCache>) -> impl Responder {
    // let mut counter = data.counter.lock().unwrap();
    // *counter += 1;
    // format!("Hello {name}! (counter = {counter})")
    "X"
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or(());
    log::debug!(" - loading the configuration file");
    let settings = Settings::new().unwrap();
    log::debug!(" - loading settings = {:?}", settings);
    /*
    // For each subscription...
    for subscription in settings.subscriptions() {
        // Create an access token cache
        let mut token_cache = AccessTokenCache::new(subscription.clone());
        // Try to get an access token
        log::debug!(" - getting access token");
        let access_token = token_cache.access_token().await?;
        log::debug!(" - got access token {:?}", access_token);
        log::debug!(" - getting another access token");
        let access_token = token_cache.access_token().await?;
        log::debug!(" - got access token {:?}", access_token);
    }
    */
    // Create a token cache map
    let token_caches = AccessTokenCacheMap::new(settings.subscriptions());
    // For each subscription...
    for subscription in settings.subscriptions() {
        // Get an access token
        log::debug!(
            " - getting access token for subscription {:?}",
            subscription.subscription_id()
        );
        let access_token = token_caches
            .access_token(subscription.subscription_id())
            .await?;
        log::debug!(" - got access token {:?}", access_token);
        log::debug!(" - getting another access token");
        let access_token = token_caches
            .access_token(subscription.subscription_id())
            .await?;
        log::debug!(" - got access token {:?}", access_token);
    }
    /*
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
    .bind(("127.0.0.1", settings.port()))?
    .run()
    .await
    // Map the std::io::Error to an anyhow::Error
    .map_err(anyhow::Error::from)
     */
    Ok(())
}
