use actix_web::http;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod get_database_usage;
pub mod get_elastic_pool;
pub mod list_databases_in_elastic_pool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AzureError {
    // The error code, e.g. "InvalidAuthenticationTokenTenant"
    pub code: String,
    // The error message
    pub message: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AzureErrorResponse {
    // The error that this response wraps.
    pub error: AzureError,
}

// Fetches an Azure response as json.
pub async fn get_json<'a, T>(
    http_client: &reqwest::Client,
    url: String,
    access_token: String,
) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    log::debug!("get_json");
    log::debug!(" - sending request");
    // We'll want to get a response
    let response = http_client
        // Call the URL
        .get(url)
        // Add the auth header
        .header("Authorization", format!("Bearer {access_token}"))
        // Make the request
        .send()
        .await?;
    log::debug!(" - got response with status code {:?}", response.status());
    // If successful...
    if http::StatusCode::OK == response.status() {
        // Get the response as json
        let value = response.json::<T>().await?;
        // Return it
        Ok(value)
    } else {
        // Get the response as error json
        let error_response = response.json::<AzureErrorResponse>().await?;
        // Get the underlying code and message
        let code = error_response.error.code;
        let message = error_response.error.message;
        // Format the message
        let formatted_message = format!("{code}: {message}");
        log::warn!("Error: {formatted_message}");
        // Return that we had an error
        Err(anyhow::anyhow!(formatted_message))
    }
}

/*
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
 */
