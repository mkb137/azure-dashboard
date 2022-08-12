use crate::AccessTokenCacheMap;
use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::format;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseProperties {
    // The database language, e.g. "SQL_Latin1_General_CP1_CI_AS"
    pub collation: String,
    // The database maximum size
    pub max_size_bytes: f64,
    // If in an elastic pool, the elastic pool ID.
    pub elastic_pool_id: Option<String>,
    // The database status, e.g. "Online"
    pub status: String,
    // The database ID (a GUID)
    pub database_id: String,
    // The database creation date
    pub creation_date: DateTime<Utc>,
    // pub current_service_objective_name: String,
    // pub requested_service_objective_name: String,
    pub default_secondary_location: String,
    pub catalog_collation: String,
    pub zone_redundant: bool,
    pub earliest_restore_date: Option<DateTime<Utc>>,
    pub read_scale: String,
    pub current_sku: DatabaseSku,
    pub current_backup_storage_redundancy: String,
    pub requested_backup_storage_redundancy: String,
    pub maintenance_configuration_id: String,
    pub is_ledger_on: bool,
    pub is_infra_encryption_enabled: bool,
}

// A database SKU
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseSku {
    // The SKU name, e.g. "ElasticPool" if in an elastic pool.
    pub name: String,
    // The SKU tier, e.g. "Standard"
    pub tier: String,
    // The SKU capacity???
    pub capacity: i64,
}

// A database.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    // The database's SKU
    pub sku: Option<DatabaseSku>,
    // The database kind, e.g. "v12"
    pub kind: String,
    // The database properties
    pub properties: DatabaseProperties,
    // The database location, e.g. "westus2"
    pub location: String,
    // The database ID
    pub id: String,
    // The database name
    pub name: String,
    // The database type.
    #[serde(rename = "type")]
    pub database_type: String,
}

// The response set by the server when requesting the list of databases
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseListResponse {
    // The response "value", which is a list of values
    #[serde(rename(deserialize = "value"))]
    values: Vec<Database>,
}

impl DatabaseListResponse {
    pub fn values(&self) -> &Vec<Database> {
        &self.values
    }
}

// Lists the databases in an elastic pool.
pub async fn list_databases_in_elastic_pool(
    token_cache_map: &AccessTokenCacheMap,
    subscription_id: String,
    resource_group_name: String,
    server_name: String,
    elastic_pool_name: String,
) -> anyhow::Result<DatabaseListResponse> {
    // Try to get an access token for this subscription
    let access_token = token_cache_map
        .access_token(subscription_id.clone())
        .await?;
    // Form the URL
    let url = format!(
        "https://management.azure.com\
        /subscriptions/{subscription_id}\
        /resourceGroups/{resource_group_name}\
        /providers/Microsoft.Sql\
        /servers/{server_name}\
        /elasticPools/{elastic_pool_name}\
        /databases\
        ?api-version=2021-11-01-preview"
    );
    // Create a client
    let client = reqwest::Client::new();
    super::get_json::<DatabaseListResponse>(client, url, access_token).await
}
