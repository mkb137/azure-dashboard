use crate::settings::SubscriptionSettings;
use chrono::TimeZone;
use std::collections::HashMap;
use std::ops::{Deref, Sub};
use std::sync::{Arc, Mutex, RwLock};

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
        // Get the "expires_in" in seconds (which can be better than "expires_on" if the clocks are different
        let expires_in = value
            .expires_in
            .parse::<i64>()
            .map_err(anyhow::Error::from)?;
        // Add it to "now" to get the expiry date
        let expiry_date = chrono::Utc::now() + chrono::Duration::seconds(expires_in);
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
    params.insert(
        "scope",
        "subscriptions/72c42748-5070-4db3-bd42-e250884dbdd5",
    );
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
    cached_token: Arc<RwLock<Option<AccessToken>>>,
}

impl AccessTokenCache {
    // Creates a new token cache for the given subscription
    pub fn new(subscription_settings: SubscriptionSettings) -> Self {
        AccessTokenCache {
            // Use the given subscription settings
            subscription_settings,
            // Start without any cached token
            cached_token: Arc::new(RwLock::new(None)),
        }
    }

    // Tries to get an access token
    pub async fn access_token(&self) -> anyhow::Result<String> {
        log::debug!("access_token()");
        {
            // Get exclusive access to the cached token
            let arc = self.cached_token.clone();
            // Get a read-only lock
            let read_lock = arc.read().unwrap();
            // If we have a cached token...
            if let Some(cached_token) = read_lock.deref() {
                log::debug!(" - got cached token");
                // If it has not expired...
                if !cached_token.is_expired() {
                    log::debug!(" - cached token has not expired");
                    // Return the token's access token
                    return Ok(cached_token.access_token.clone());
                }
            }
        }
        // Either we don't have a token or it has expired.
        // Get an access token using the settings in the subscription
        let new_token = get_access_token(
            self.subscription_settings.token_url.clone(),
            self.subscription_settings.client_id.clone(),
            self.subscription_settings.client_secret.clone(),
            self.subscription_settings.resource.clone(),
        )
        .await?;
        // Get exclusive access to the cached token
        let arc = self.cached_token.clone();
        // Get a write lock
        let mut write_lock = arc.write().unwrap();
        // Insert the new access token into the cache
        let inserted_token = write_lock.insert(new_token);
        // Return the new access token
        Ok(inserted_token.access_token.clone())
    }
}

// A map of access token caches by subscription ID
pub struct AccessTokenCacheMap {
    // The access token caches by subscription ID.
    access_token_caches: HashMap<String, AccessTokenCache>,
}
impl AccessTokenCacheMap {
    // Create a new cache map from the list of subscriptions.
    pub fn new(subscriptions: &Vec<SubscriptionSettings>) -> Self {
        // Create the map of caches
        let mut caches = HashMap::new();
        // For each subscription...
        for subscription in subscriptions {
            // Add a new cache
            caches.insert(
                subscription.subscription_id.clone(),
                AccessTokenCache::new(subscription.clone()),
            );
        }
        // Return the map
        AccessTokenCacheMap {
            access_token_caches: caches,
        }
    }
    // Gets an access token for the given subscription.
    pub async fn access_token(&self, subscription_id: String) -> anyhow::Result<String> {
        // If we have an access token cache for this subscription...
        if let Some(access_token_cache) = self.access_token_caches.get(&subscription_id) {
            // Try to get an access token
            let access_token = access_token_cache.access_token().await?;
            // Return the token
            Ok(access_token)
        }
        // If we don't have an access token cache for this subscription...
        else {
            // Return an error.  The token caches should have been initialized on startup.
            Err(anyhow::anyhow!(
                "There is no token cache for subscription ID {:?}",
                subscription_id
            ))
        }
    }
}
