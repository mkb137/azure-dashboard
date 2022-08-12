use crate::azure_apis::get_database_usage::get_database_usage;
use crate::settings::{
    DashboardSettings, DatabaseSettings, ElasticPoolSettings, SubscriptionSettings,
};
use crate::AzureDashboardError::AzureApiError;
use crate::{AccessTokenCacheMap, AzureDashboardError};
use actix_web::{get, web};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsageViewModel {
    // The amount of data used
    database_size_used: u64,
    // The amount of data allocated
    database_size_allocated: u64,
    // The maximum size of the database
    database_size_max: u64,
}

// Returns info related to a database as JSON
#[get("/api/subscription/{subscription_id}/resource-group/{resource_group_name}/server/{server_name}/database/{database_name}/usage")]
pub async fn database_usage(
    path: web::Path<(String, String, String, String)>,
    http_client: web::Data<reqwest::Client>,
    settings: web::Data<DashboardSettings>,
    token_cache_map: web::Data<AccessTokenCacheMap>,
) -> Result<web::Json<DatabaseUsageViewModel>, AzureDashboardError> {
    // Get the path components
    let (subscription_id, resource_group_name, server_name, database_name) = path.into_inner();
    log::debug!(
        "database - \
         subscription_id = {subscription_id}, \
         resource_group_name = {resource_group_name}, \
         server_name = {server_name}, \
         database_name = {database_name}",
    );
    // Get the database usages
    let database_usage_response = get_database_usage(
        http_client.get_ref(),
        token_cache_map.get_ref(),
        subscription_id.clone(),
        resource_group_name.clone(),
        server_name.clone(),
        database_name.clone(),
    )
    .await
    // If we got an error, convert it to an Azure API error
    .map_err(|e| AzureApiError(e.to_string()))?;
    log::debug!(" - got response\r\n{:?}", database_usage_response);
    // Get the databases sizes
    let (database_size_used, database_size_allocated, database_size_max) =
        database_usage_response.get_sizes();
    // Create the view model
    let view_model = DatabaseUsageViewModel {
        database_size_used,
        database_size_allocated,
        database_size_max,
    };
    // Return the view model as JSON
    Ok(web::Json(view_model))
}
