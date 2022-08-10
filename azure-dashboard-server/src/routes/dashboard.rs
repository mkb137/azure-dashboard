use crate::settings::{
    DashboardSettings, DatabaseSettings, ElasticPoolSettings, SubscriptionSettings,
};
use crate::AzureDashboardError;
use actix_web::{get, web};

// Settings for a database to be displayed in the dashboard.
#[derive(Debug, serde::Serialize)]
struct DatabaseViewModel {
    // The server name
    server_name: String,
    // The database name
    database_name: String,
}

// Creates a database from the settings.
impl From<&DatabaseSettings> for DatabaseViewModel {
    fn from(value: &DatabaseSettings) -> Self {
        Self {
            server_name: value.server_name(),
            database_name: value.database_name(),
        }
    }
}
// Settings for a database to be displayed in the dashboard.
#[derive(Debug, serde::Serialize)]
struct ElasticPoolViewModel {
    // The server name
    server_name: String,
    // The elastic pool name
    elastic_pool_name: String,
}

// Creates an elastic pool from the settings.
impl From<&ElasticPoolSettings> for ElasticPoolViewModel {
    fn from(value: &ElasticPoolSettings) -> Self {
        Self {
            server_name: value.server_name(),
            elastic_pool_name: value.elastic_pool_name(),
        }
    }
}
// The settings relating to a single subscription.
#[derive(Debug, serde::Serialize)]
struct SubscriptionViewModel {
    // The display name for this subscription
    display_name: String,
    // The subscription ID (a GUID)
    subscription_id: String,
    // The databases to be displayed in the dashboard
    databases: Vec<DatabaseViewModel>,
    // The elastic pools to be displayed in the dashboard
    elastic_pools: Vec<ElasticPoolViewModel>,
}

// Creates a subscription from the settings.
impl From<&SubscriptionSettings> for SubscriptionViewModel {
    fn from(value: &SubscriptionSettings) -> Self {
        Self {
            display_name: value.display_name(),
            subscription_id: value.subscription_id(),
            databases: value
                .databases()
                .iter()
                .map(|d| d.into())
                .collect::<Vec<_>>(),
            elastic_pools: value
                .elastic_pools()
                .iter()
                .map(|d| d.into())
                .collect::<Vec<_>>(),
        }
    }
}

// The dashboard settings.
#[derive(Debug, serde::Serialize)]
pub struct DashboardViewModel {
    // The subscriptions.
    subscriptions: Vec<SubscriptionViewModel>,
}

// Creates a dashboard from the settings.
impl From<&DashboardSettings> for DashboardViewModel {
    fn from(value: &DashboardSettings) -> Self {
        Self {
            subscriptions: value
                .subscriptions()
                .iter()
                .map(|s| s.into())
                .collect::<Vec<_>>(),
        }
    }
}

// Returns the dashboard info as JSON.
#[get("/api/dashboard")]
pub async fn dashboard(
    settings: web::Data<DashboardSettings>,
) -> Result<web::Json<DashboardViewModel>, AzureDashboardError> {
    log::debug!("dashboard - settings = {:?}", settings);
    // Create a dashboard view model from the settings
    let view_model: DashboardViewModel = settings.get_ref().into();
    // Convert the view model to json
    let json = web::Json(view_model);
    // Return the json
    Ok(json)
}
