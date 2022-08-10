use crate::AccessTokenCacheMap;
use chrono::Utc;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElasticPoolPerDatabaseSettings {
    // The minimum number of databases in the pool?
    min_capacity: f64,
    // The maximum number of databases in the pool?
    max_capacity: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElasticPoolProperties {
    // The pool state, e.g. "Ready"
    state: String,
    // The date the pool was created
    creation_date: chrono::DateTime<Utc>,
    // The pool maximum size in bytes
    max_size_bytes: u64,
    // The per-database settings
    per_database_settings: ElasticPoolPerDatabaseSettings,
    // Whether the pool is zone-redundant
    zone_redundant: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElasticPoolSku {
    // The SKU name, e.g. "StandardPool"
    name: String,
    // The SKU tier, e.g. "Standard"
    tier: String,
    // The tier capacity
    capacity: i16,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElasticPool {
    // The pool's SKU
    sku: ElasticPoolSku,
    // The pool kind, e.g. "pool"
    kind: String,
    // The pool properties
    properties: ElasticPoolProperties,
    // The pool location, e.g. "westus2"
    location: String,
    // The pool ID
    id: String,
    // The pool name
    name: String,
    // The pool type, e.g. "Microsoft.Sql/servers/elasticPools"
    #[serde(rename(deserialize = "type"))]
    pool_type: String,
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
        .await?
        // Get the response as json
        .json::<ElasticPool>()
        .await?;
    // Return the response
    Ok(response)
}
