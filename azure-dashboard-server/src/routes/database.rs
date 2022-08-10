use crate::azure_apis::get_database_usage::get_database_usage;
use crate::settings::{
    DashboardSettings, DatabaseSettings, ElasticPoolSettings, SubscriptionSettings,
};
use crate::AzureDashboardError::AzureApiError;
use crate::{AccessTokenCacheMap, AzureDashboardError};
use actix_web::{get, web};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseViewModel {
    // The amount of data used
    database_size_used: f64,
    // The amount of data allocated
    database_size_allocated: f64,
    // The maximum size of the database
    database_size_max: f64,
}

// Returns info related to a database as JSON
#[get("/api/subscription/{subscription_id}/resource-group/{resource_group_name}/server/{server_name}/database/{database_name}")]
pub async fn database(
    path: web::Path<(String, String, String, String)>,
    settings: web::Data<DashboardSettings>,
    token_cache_map: web::Data<AccessTokenCacheMap>,
) -> Result<web::Json<DatabaseViewModel>, AzureDashboardError> {
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
    // We want database properties
    let mut database_size: f64 = 0.0;
    let mut database_size_max: f64 = 0.0;
    let mut database_size_allocated: f64 = 0.0;
    // Look for a "database_size" value
    if let Some(value) = database_usage_response.find_value_by_name("database_size") {
        // If found, use the current value as the size and the limit as the max size.
        database_size = value.properties().current_value();
        database_size_max = value.properties().limit();
    } else {
        log::debug!(" - failed to find 'database_size' value.")
    }
    // Look for a "database_allocated_size" value
    if let Some(value) = database_usage_response.find_value_by_name("database_allocated_size") {
        // If found, use the current value as the allocated size.  The limit should be the same as above.
        database_size_allocated = value.properties().current_value();
    } else {
        log::debug!(" - failed to find 'database_allocated_size' value.")
    }
    // Create the view model
    let view_model = DatabaseViewModel {
        database_size_used: database_size,
        database_size_allocated,
        database_size_max,
    };
    // Return the view model as JSON
    Ok(web::Json(view_model))
}
