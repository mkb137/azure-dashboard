use crate::settings::{
    DashboardSettings, DatabaseSettings, ElasticPoolSettings, SubscriptionSettings,
};
use crate::AzureDashboardError::AzureApiError;
use crate::{AccessTokenCacheMap, AzureDashboardError};
use actix_web::{get, web};

// Returns info related to an elastic pool as JSON
#[get("/api/subscription/{subscription_id}/resource-group/{resource_group_name}/server/{server_name}/database/{database_name}")]
pub async fn database(
    path: web::Path<(String, String, String, String)>,
    settings: web::Data<DashboardSettings>,
    token_cache_map: web::Data<AccessTokenCacheMap>,
) -> Result<web::Json<String>, AzureDashboardError> {
    Err(AzureDashboardError::InternalError)
}
