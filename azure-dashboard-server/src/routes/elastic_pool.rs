use crate::azure_apis::get_database_usage::get_database_usage;
use crate::azure_apis::get_elastic_pool::{get_elastic_pool, ElasticPool};
use crate::azure_apis::list_databases_in_elastic_pool::{
    list_databases_in_elastic_pool, Database, DatabaseListResponse,
};
use crate::settings::{
    DashboardSettings, DatabaseSettings, ElasticPoolSettings, SubscriptionSettings,
};
use crate::AzureDashboardError::AzureApiError;
use crate::{AccessTokenCacheMap, AzureDashboardError};
use actix_web::{get, web};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElasticPoolViewModel {
    // The amount of data used
    database_size_used: u64,
    // The amount of data allocated
    database_size_allocated: u64,
    // The maximum size of the database
    database_size_max: u64,
}

// Returns info related to an elastic pool as JSON
#[get("/api/subscription/{subscription_id}/resource-group/{resource_group_name}/server/{server_name}/elastic-pool/{elastic_pool_name}")]
pub async fn elastic_pool(
    path: web::Path<(String, String, String, String)>,
    settings: web::Data<DashboardSettings>,
    token_cache_map: web::Data<AccessTokenCacheMap>,
) -> Result<web::Json<ElasticPoolViewModel>, AzureDashboardError> {
    log::debug!("elastc_pool");
    // Get the path components
    let (subscription_id, resource_group_name, server_name, elastic_pool_name) = path.into_inner();
    log::debug!(
        "database - \
         subscription_id = {subscription_id}, \
         resource_group_name = {resource_group_name}, \
         server_name = {server_name}, \
         elastic_pool_name = {elastic_pool_name}",
    );
    // Get the elastic pool info
    let elastic_pool_response = get_elastic_pool(
        token_cache_map.get_ref(),
        subscription_id.clone(),
        resource_group_name.clone(),
        server_name.clone(),
        elastic_pool_name.clone(),
    )
    .await
    // If we got an error, convert it to an Azure API error
    .map_err(|e| AzureApiError(e.to_string()))?;
    log::debug!(" - got elastic pool response");
    // Get the databases in the elastic pool
    let database_list_response = list_databases_in_elastic_pool(
        token_cache_map.get_ref(),
        subscription_id.clone(),
        resource_group_name.clone(),
        server_name.clone(),
        elastic_pool_name.clone(),
    )
    .await
    // If we got an error, convert it to an Azure API error
    .map_err(|e| AzureApiError(e.to_string()))?;
    log::debug!(" - got database list response");

    // We want database properties
    let database_size_max: u64 = elastic_pool_response.properties.max_size_bytes;
    let mut database_size_used: u64 = 0;
    let mut database_size_allocated: u64 = 0;
    // Get the futures that will fetch the database usages for each database
    let database_usage_response_futures = database_list_response.values().iter().map(|database|
        // Get the database usages
        get_database_usage(
            token_cache_map.get_ref(),
            subscription_id.clone(),
            resource_group_name.clone(),
            server_name.clone(),
            database.name.clone(),
        ));
    // Execute the futures in parallel
    let database_usage_responses = futures::future::try_join_all(database_usage_response_futures)
        .await
        .unwrap();
    for database_usage_response in database_usage_responses {
        // Get the databases sizes
        let (size_used, size_allocated, _size_max) = database_usage_response.get_sizes();
        // Add them to the elastic pool's sizes
        database_size_used += size_used;
        database_size_allocated += size_allocated;
        log::debug!(
            " - adding sizes size = {:?}, allocated = {:?}",
            database_size_used,
            database_size_allocated
        );
    }
    log::debug!(
        " - final, size = {:?}, allocated = {:?}",
        database_size_used,
        database_size_allocated
    );
    // Create the view model
    let view_model = ElasticPoolViewModel {
        database_size_allocated,
        database_size_used,
        database_size_max,
    };
    // Return the view model as json
    Ok(web::Json(view_model))
}
