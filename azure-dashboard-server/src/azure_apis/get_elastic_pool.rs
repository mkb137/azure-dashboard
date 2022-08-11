use crate::{AccessTokenCacheMap, AzureDashboardError};
use actix_web::http;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use std::io::repeat;

use crate::AzureDashboardError::AzureApiError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElasticPoolPerDatabaseSettings {
    // The minimum number of databases in the pool?
    pub min_capacity: f64,
    // The maximum number of databases in the pool?
    pub max_capacity: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElasticPoolProperties {
    // The pool state, e.g. "Ready"
    pub state: String,
    // The date the pool was created
    pub creation_date: DateTime<Utc>,
    // The pool maximum size in bytes
    pub max_size_bytes: u64,
    // The per-database settings
    pub per_database_settings: ElasticPoolPerDatabaseSettings,
    // Whether the pool is zone-redundant
    pub zone_redundant: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElasticPoolSku {
    // The SKU name, e.g. "StandardPool"
    pub name: String,
    // The SKU tier, e.g. "Standard"
    pub tier: String,
    // The tier capacity
    pub capacity: i16,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElasticPool {
    // The pool's SKU
    pub sku: Option<ElasticPoolSku>,
    // The pool kind, e.g. "pool"
    pub kind: String,
    // The pool properties
    pub properties: ElasticPoolProperties,
    // The pool location, e.g. "westus2"
    pub location: String,
    // The pool ID
    pub id: String,
    // The pool name
    pub name: String,
    // The pool type, e.g. "Microsoft.Sql/servers/elasticPools"
    #[serde(rename(deserialize = "type"))]
    pub pool_type: String,
}

pub async fn get_elastic_pool(
    token_cache_map: &AccessTokenCacheMap,
    subscription_id: String,
    resource_group_name: String,
    server_name: String,
    elastic_pool_name: String,
) -> anyhow::Result<ElasticPool> {
    // Try to get an access token for this subscription
    let access_token = token_cache_map
        .access_token(subscription_id.clone())
        .await?;
    // Get the URL
    let url = format!(
        "https://management.azure.com\
        /subscriptions/{subscription_id}\
        /resourceGroups/{resource_group_name}\
        /providers/Microsoft.Sql\
        /servers/{server_name}\
        /elasticPools/{elastic_pool_name}\
        ?api-version=2021-05-01-preview"
    );
    // Create a client
    let client = reqwest::Client::new();
    let response = client
        // Get the data from the URL
        .get(url)
        // Add the auth header
        .header("Authorization", format!("Bearer {access_token}"))
        // Make the request
        .send()
        .await?;
    // If successful...
    if StatusCode::OK == response.status() {
        // Get the response as json
        let elastic_pool = response.json::<ElasticPool>().await?;
        // Return it
        Ok(elastic_pool)
    } else {
        // Get the response as text
        let text = response.text().await?;
        // Log it
        log::debug!("Error: {text}");
        // Return that we had an error
        Err(anyhow::anyhow!("test"))
    }
}
