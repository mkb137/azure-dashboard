use crate::azure_apis::get_elastic_pool::{get_elastic_pool, ElasticPool};
use crate::settings::{
    DashboardSettings, DatabaseSettings, ElasticPoolSettings, SubscriptionSettings,
};
use crate::AzureDashboardError::AzureApiError;
use crate::{AccessTokenCacheMap, AzureDashboardError};
use actix_web::{get, web};

// Returns info related to an elastic pool as JSON
#[get("/api/subscription/{subscription_id}/resource-group/{resource_group_name}/server/{server_name}/elastic-pool/{elastic_pool_name}")]
pub async fn elastic_pool(
    path: web::Path<(String, String, String, String)>,
    settings: web::Data<DashboardSettings>,
    token_cache_map: web::Data<AccessTokenCacheMap>,
) -> Result<web::Json<ElasticPool>, AzureDashboardError> {
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
    log::debug!(" - got response\r\n{:?}", elastic_pool_response);
    Ok(web::Json(elastic_pool_response))
}
