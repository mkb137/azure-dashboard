use crate::{AccessTokenCacheMap, AzureDashboardError};

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

impl DatabaseUsageProperties {
    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }
    pub fn current_value(&self) -> f64 {
        self.current_value
    }
    pub fn limit(&self) -> f64 {
        self.limit
    }
    pub fn unit(&self) -> String {
        self.unit.clone()
    }
}

// A usage value.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsage {
    // The value properties
    properties: DatabaseUsageProperties,
    // The value name, e.g. "database_size"
    name: String,
}

impl DatabaseUsage {
    pub fn properties(&self) -> &DatabaseUsageProperties {
        &self.properties
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    // Returns true if the usage has the given name.
    pub fn has_name(&self, name: &str) -> bool {
        self.name.eq(name)
    }
}

// The response set by the server when requesting database usages
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUsageResponse {
    // The response "value", which is a list of values
    #[serde(rename(deserialize = "value"))]
    values: Vec<DatabaseUsage>,
}

impl DatabaseUsageResponse {
    pub fn values(&self) -> &Vec<DatabaseUsage> {
        &self.values
    }
    // Finds the first usage with the given name
    pub fn find_value_by_name(&self, name: &str) -> Option<&DatabaseUsage> {
        self.values.iter().find(|u| u.has_name(name))
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
        .await?
        // Get the response as json
        .json::<DatabaseUsageResponse>()
        .await?;
    // Return the response
    Ok(response)
}
