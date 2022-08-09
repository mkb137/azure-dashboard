use crate::settings::SubscriptionSettings;
use chrono::TimeZone;
use std::collections::HashMap;
use std::ops::Sub;
use std::sync::{Arc, Mutex};

// The response from an access token request.
#[derive(Debug, serde::Deserialize)]
struct TokenResponse {
    // The token type (should be "Bearer")
    token_type: String,
    // The number of seconds in which the token expires
    expires_in: String,
    // The date/time, in unix seconds since the Epoch, on which the token expires
    expires_on: String,
    // The access token
    access_token: String,
}
impl TokenResponse {
    pub fn token_type(&self) -> String {
        self.token_type.clone()
    }
    pub fn expires_in(&self) -> String {
        self.expires_in.clone()
    }
    pub fn expires_on(&self) -> String {
        self.expires_on.clone()
    }
    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }
}

// An access token
#[derive(Debug)]
struct AccessToken {
    access_token: String,
    expiry_date: chrono::DateTime<chrono::Utc>,
}
impl AccessToken {
    // Creates a new access token
    pub fn new(access_token: String, expiry_date: chrono::DateTime<chrono::Utc>) -> Self {
        AccessToken {
            access_token,
            expiry_date,
        }
    }
    // The access token.
    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }
    // The expiry date.
    pub fn expiry_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.expiry_date.clone()
    }
    // Whether the token has expired.
    pub fn is_expired(&self) -> bool {
        // The token is expired if the expiry date is earlier than now
        self.expiry_date < chrono::offset::Utc::now()
    }
}

// Tries to create an access token from a token response.
impl TryFrom<TokenResponse> for AccessToken {
    type Error = anyhow::Error;

    // Tries to create an access token from a token response.
    fn try_from(value: TokenResponse) -> Result<Self, Self::Error> {
        // Try to convert the expiry date string to seconds
        let unix_expiry_date = value
            .expires_on
            .parse::<i64>()
            .map_err(anyhow::Error::from)?;
        // Create a UTC date
        let expiry_date = chrono::Utc.timestamp(unix_expiry_date, 0);
        // Create the access token
        let access_token = AccessToken::new(value.access_token(), expiry_date);
        // Return it
        Ok(access_token)
    }
}

// Gets a new access token by making a request to the token URL and parsing the response.
async fn get_access_token(
    token_url: String,
    client_id: String,
    client_secret: String,
    resource: String,
) -> anyhow::Result<AccessToken> {
    log::debug!("get_access_token - token_url = {token_url}");
    // Create a client
    let client = reqwest::Client::new();
    // Create our parameters
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", client_id.as_str());
    params.insert("client_secret", client_secret.as_str());
    params.insert("resource", resource.as_str());
    // Post the request
    let response = client.post(token_url).form(&params).send().await?;
    // If the response was successful...
    if reqwest::StatusCode::OK == response.status() {
        // Deserialize the JSON to our TokenResponse type.
        let token_response: TokenResponse = response.json::<TokenResponse>().await?;
        log::debug!(" - got token response {:?}", token_response);
        // Create an access token from the token response
        let access_token: AccessToken = token_response.try_into()?;
        log::debug!(" - created access token {:?}", access_token);
        // Return the token
        Ok(access_token)
    }
    // If the response was unsuccessful...
    else {
        // Return an error.
        Err(anyhow::anyhow!(
            "Failed to get access token.  Error response: {:?} = {:?}",
            response.status(),
            response.text().await?
        ))
    }
}

// An access token cache for a particular subscription.
// When asked for a token:
// - If no token been fetched, fetches a new token.
// - If a token has been fetched previously and the token has not expired, returns the token.
// - If a token has been fetched previously and the token has expired, fetches a new token and caches it for future requests.
pub struct AccessTokenCache {
    // The subscription info
    subscription_settings: SubscriptionSettings,
    // The cached access token, guarded against multiple async calls.
    cached_token: Option<AccessToken>,
}

impl AccessTokenCache {
    // Creates a new token cache for the given subscription
    pub fn new(subscription_settings: SubscriptionSettings) -> Self {
        AccessTokenCache {
            // Use the given subscription settings
            subscription_settings,
            // Start without any cached token
            cached_token: None,
        }
    }
    // Tries to get a new access token
    async fn update_token(&mut self) -> anyhow::Result<String> {
        // Get an access token using the settings in the subscription
        let new_token = get_access_token(
            self.subscription_settings.token_url(),
            self.subscription_settings.client_id(),
            self.subscription_settings.client_secret(),
            self.subscription_settings.resource(),
        )
        .await?;
        // Save a copy of the access token value
        let access_token = new_token.access_token();
        // Save the new token as the cached token
        self.cached_token = Some(new_token);
        // Return the new token's access token
        Ok(access_token)
    }
    // Tries to get an access token
    pub async fn access_token(&mut self) -> anyhow::Result<String> {
        log::debug!("access_token()");
        // Get exclusive access to the cached token
        match &mut self.cached_token {
            // If we have a cached token...
            Some(cached_token) => {
                log::debug!(" - checking existing token");
                // If the cached token has expired...
                if cached_token.is_expired() {
                    log::debug!("   - existing token has expired.  Getting new token.");
                    // Get a new access token
                    let new_token = self.update_token().await?;
                    // Return it
                    Ok(new_token)
                }
                // If the cached token has NOT expired...
                else {
                    log::debug!("   - existing token has NOT expired.  Returning existing token.");
                    // Return the cached token's access token as-is
                    Ok(cached_token.access_token())
                }
            }
            None => {
                log::debug!(" - no exiting token.  Getting new token.");
                // Get a new access token
                let new_token = self.update_token().await?;
                // Return it
                Ok(new_token)
            }
        }
    }
}
