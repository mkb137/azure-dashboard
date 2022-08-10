use crate::settings::{
    DashboardSettings, DatabaseSettings, ElasticPoolSettings, SubscriptionSettings,
};
use crate::AzureDashboardError::AzureApiError;
use crate::{AccessTokenCacheMap, AzureDashboardError};
use actix_web::{get, web};

// The properties within a usage value.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsageProperties {
    // The display name of the value, e.g. "Database Size"
    display_name: String,
    // The current value of the property, e.g. if Database Size, the size in bytes.
    current_value: f64,
    // The maximum value of the property, e.g. if Database Size, the maximum size in bytes.
    limit: f64,
    // The data unit, e.g. "Bytes"
    unit: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsageValue {
    // The value properties
    properties: DatabaseUsageProperties,
    // The value name, e.g. "database_size"
    name: String,
}
// The response set by the server when requesting database usages
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsageResponse {
    // The response "value", which is a list of values
    value: Vec<DatabaseUsageValue>,
}

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
         server_name = {server_name}, \
         resource_group_name = {resource_group_name}, \
         database_name = {database_name}",
    );
    // Try to get an access token for this subscription
    let access_token = token_cache_map
        .access_token(subscription_id.clone())
        .await
        .map_err(|e| AzureDashboardError::CouldNotGetAccessToken(subscription_id.clone()))?;
    log::debug!(" - got access_token {:?}", access_token);

    // Call the azure API for the database
    let url = format!(
        "https://management.azure.com\
        /subscriptions/{subscription_id}\
        /resourceGroups/{resource_group_name}\
        /providers/Microsoft.Sql\
        /servers/{server_name}\
        /databases/{database_name}\
        /usages\
        ?api-version=2022-02-01-preview"
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
        .await
        // If we got an error, convert it to an Azure API error
        .map_err(|e| AzureApiError(e.to_string()))?
        // Get the response as json
        .json::<DatabaseUsageResponse>()
        .await
        // If we got an error, convert it to an Azure API error
        .map_err(|e| AzureApiError(e.to_string()))?;
    log::debug!(" - got response\r\n{:?}", response);
    // We want database propertes
    let mut database_size: f64 = 0.0;
    let mut database_size_max: f64 = 0.0;
    let mut database_size_allocated: f64 = 0.0;
    // Look for a "database_size" value
    if let Some(value) = response
        .value
        .iter()
        .find(|value| value.name == "database_size")
    {
        // If found, use the current value as the size and the limit as the max size.
        database_size = value.properties.current_value;
        database_size_max = value.properties.limit;
    } else {
        log::debug!(" - failed to find 'database_size' value.")
    }
    // Look for a "database_allocated_size" value
    if let Some(value) = response
        .value
        .iter()
        .find(|value| value.name == "database_allocated_size")
    {
        // If found, use the current value as the allocated size.  The limit should be the same as above.
        database_size_allocated = value.properties.current_value;
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
