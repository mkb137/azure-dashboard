use crate::{AccessTokenCacheMap, AzureDashboardError};
use actix_web::http::StatusCode;

use serde::{Deserialize, Serialize};
// Usage responses will be of the form:
// ```json
// {
//   "value": [
//     {
//       "properties": {
//         "displayName": "Database Size",
//         "currentValue": 16720592896,
//         "limit": 107374182400,
//         "unit": "Bytes"
//       },
//       "id": "/subscriptions/SUBSCRIPTION_ID/resourceGroups/RESOURCE_GROUP_NAME/providers/Microsoft.Sql/servers/SERVER_NAME/databases/DATABASE_NAME/usages/database_size",
//       "name": "database_size",
//       "type": "Microsoft.Sql/servers/databases/usages"
//     },
//     {
//       "properties": {
//         "displayName": "Database Allocated Size",
//         "currentValue": 16761487360,
//         "limit": 107374182400,
//         "unit": "Bytes"
//       },
//       "id": "/subscriptions/SUBSCRIPTION_ID/resourceGroups/RESOURCE_GROUP_NAME/providers/Microsoft.Sql/servers/SERVER_NAME/databases/DATABASE_NAME/usages/database_allocated_size",
//       "name": "database_allocated_size",
//       "type": "Microsoft.Sql/servers/databases/usages"
//     }
//   ]
// }
// ```

// The properties within a usage value.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsageProperties {
    // The display name of the value, e.g. "Database Size"
    pub display_name: String,
    // The current value of the property, e.g. if Database Size, the size in bytes.
    pub current_value: f64,
    // The maximum value of the property, e.g. if Database Size, the maximum size in bytes.
    pub limit: f64,
    // The data unit, e.g. "Bytes"
    pub unit: String,
}
// A usage value.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsage {
    // The value properties
    pub properties: DatabaseUsageProperties,
    // The value name, e.g. "database_size"
    pub name: String,
}
impl DatabaseUsage {
    // Returns true if the usage has the given name.
    pub fn has_name(&self, name: &str) -> bool {
        self.name.eq(name)
    }
}

// The response set by the server when requesting database usages
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsageResponse {
    // The response "value", which is a list of values
    #[serde(rename(deserialize = "value"))]
    pub values: Vec<DatabaseUsage>,
}

impl DatabaseUsageResponse {
    // Finds the first usage with the given name
    pub fn find_value_by_name(&self, name: &str) -> Option<&DatabaseUsage> {
        self.values.iter().find(|u| u.has_name(name))
    }
    // Get the sizes (used, allocated, and max)
    pub fn get_sizes(&self) -> (u64, u64, u64) {
        // Set default values for the sizes we're looking for
        let mut database_size_used: u64 = 0;
        let mut database_size_max: u64 = 0;
        let mut database_size_allocated: u64 = 0;
        // Look for a "database_size" value
        if let Some(value) = self.find_value_by_name("database_size") {
            // If found, use the current value as the size and the limit as the max size.
            database_size_used = value.properties.current_value.round() as u64;
            database_size_max = value.properties.limit.round() as u64;
        } else {
            log::debug!(" - failed to find 'database_size' value.")
        }
        // Look for a "database_allocated_size" value
        if let Some(value) = self.find_value_by_name("database_allocated_size") {
            // If found, use the current value as the allocated size.  The limit should be the same as above.
            database_size_allocated = value.properties.current_value.round() as u64;
        } else {
            log::debug!(" - failed to find 'database_allocated_size' value.")
        }
        // Return the sizes
        (
            database_size_used,
            database_size_allocated,
            database_size_max,
        )
    }
}

// Calls the Azure API to get the usage for the given database.
pub async fn get_database_usage(
    token_cache_map: &AccessTokenCacheMap,
    subscription_id: String,
    resource_group_name: String,
    server_name: String,
    database_name: String,
) -> anyhow::Result<DatabaseUsageResponse> {
    // Try to get an access token for this subscription
    let access_token = token_cache_map
        .access_token(subscription_id.clone())
        .await?;
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
        .await?;
    // If successful...
    if StatusCode::OK == response.status() {
        // Get the response as json
        let database_usage = response.json::<DatabaseUsageResponse>().await?;
        // Return it
        Ok(database_usage)
    } else {
        // Get the response as text
        let text = response.text().await?;
        // Log it
        log::debug!("Error: {text}");
        // Return that we had an error
        Err(anyhow::anyhow!("test"))
    }
}
