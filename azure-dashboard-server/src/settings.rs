// Settings for a database to be displayed in the dashboard.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    // The server name
    pub server_name: String,
    // The database name
    pub database_name: String,
}

// Settings for a database to be displayed in the dashboard.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ElasticPoolSettings {
    // The server name
    pub server_name: String,
    // The elastic pool name
    pub elastic_pool_name: String,
}

// Settings for a resource group.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ResourceGroupSettings {
    // The resource group name
    pub resource_group_name: String,
    // The databases to be displayed in the dashboard
    pub databases: Vec<DatabaseSettings>,
    // The elastic pools to be displayed in the dashboard
    pub elastic_pools: Vec<ElasticPoolSettings>,
}

// The settings relating to a single subscription.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct SubscriptionSettings {
    // The Azure AD App Registration client ID (a GUID)
    pub client_id: String,
    // The Azure AD App Registration secret value (a long string of random chars)
    // Note: Not the secret ID (a GUID)
    // Note2: This secret expires 6, 12, or however many months were specified at time of
    // creation and will have to be updated.
    pub client_secret: String,
    // The display name for this subscription
    pub display_name: String,
    // The OAuth2 resource name, e.g. "https://management.azure.com"
    pub resource: String,
    // The subscription ID (a GUID)
    pub subscription_id: String,
    // The tenant ID (a GUID)
    pub tenant_id: String,
    // The Azure OAuth2 base auth URL, e.g. https://login.microsoftonline.com/TENANT_ID/oauth2/token
    pub token_url: String,
    // The resource groups
    pub resource_groups: Vec<ResourceGroupSettings>,
}

// The application configuration settings.
#[derive(Debug, serde::Deserialize)]
pub struct DashboardSettings {
    // The host we'll run on (e.g. "localhost")
    pub host: String,
    // The port we'll run on
    pub port: u16,
    // The subscriptions.
    pub subscriptions: Vec<SubscriptionSettings>,
}

impl DashboardSettings {
    // Loads the settings from file.
    pub fn new() -> Result<Self, config::ConfigError> {
        log::debug!("Settings.new");
        // Get the run mode from the environment variables.
        // Default to "local" if not specified.
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "local".into());
        log::debug!(" - run_mode = {:?}", run_mode);
        // Load the settings from configuration
        let settings = config::Config::builder()
            // Add the default configuration file
            .add_source(config::File::with_name("config.json"))
            // Add the file for the selected run mode, if present
            .add_source(
                config::File::with_name(&format!("config.{}.json", run_mode)).required(false),
            )
            // Build the file
            .build()?;
        // Try to deserialize the file
        settings.try_deserialize()
    }
}
