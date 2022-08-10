// Settings for a database to be displayed in the dashboard.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    // The server name
    server_name: String,
    // The database name
    database_name: String,
}
impl DatabaseSettings {
    pub fn database_name(&self) -> String {
        self.database_name.clone()
    }
    pub fn server_name(&self) -> String {
        self.server_name.clone()
    }
}

// Settings for a database to be displayed in the dashboard.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ElasticPoolSettings {
    // The server name
    server_name: String,
    // The elastic pool name
    elastic_pool_name: String,
}
impl ElasticPoolSettings {
    pub fn elastic_pool_name(&self) -> String {
        self.elastic_pool_name.clone()
    }
    pub fn server_name(&self) -> String {
        self.server_name.clone()
    }
}

// The settings relating to a single subscription.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct SubscriptionSettings {
    // The Azure AD App Registration client ID (a GUID)
    client_id: String,
    // The Azure AD App Registration secret value (a long string of random chars)
    // Note: Not the secret ID (a GUID)
    // Note2: This secret expires 6, 12, or however many months were specified at time of
    // creation and will have to be updated.
    client_secret: String,
    // The display name for this subscription
    display_name: String,
    // The OAuth2 resource name, e.g. "https://management.azure.com"
    resource: String,
    // The subscription ID (a GUID)
    subscription_id: String,
    // The tenant ID (a GUID)
    tenant_id: String,
    // The Azure OAuth2 base auth URL, e.g. https://login.microsoftonline.com/TENANT_ID/oauth2/token
    token_url: String,
    // The databases to be displayed in the dashboard
    databases: Vec<DatabaseSettings>,
    // The elastic pools to be displayed in the dashboard
    elastic_pools: Vec<ElasticPoolSettings>,
}
impl SubscriptionSettings {
    pub fn client_id(&self) -> String {
        self.client_id.clone()
    }
    pub fn client_secret(&self) -> String {
        self.client_secret.clone()
    }
    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }
    pub fn subscription_id(&self) -> String {
        self.subscription_id.clone()
    }
    pub fn resource(&self) -> String {
        self.resource.clone()
    }
    pub fn tenant_id(&self) -> String {
        self.tenant_id.clone()
    }
    pub fn token_url(&self) -> String {
        self.token_url.clone()
    }
    pub fn databases(&self) -> &Vec<DatabaseSettings> {
        &self.databases
    }
    pub fn elastic_pools(&self) -> &Vec<ElasticPoolSettings> {
        &self.elastic_pools
    }
}

// The application configuration settings.
#[derive(Debug, serde::Deserialize)]
pub struct DashboardSettings {
    // The host we'll run on (e.g. "localhost")
    host: String,
    // The port we'll run on
    port: u16,
    // The subscriptions.
    subscriptions: Vec<SubscriptionSettings>,
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
    // The configured host.
    pub fn host(&self) -> String {
        self.host.clone()
    }
    // The configured port.
    pub fn port(&self) -> u16 {
        self.port
    }
    // The subscriptions
    pub fn subscriptions(&self) -> &Vec<SubscriptionSettings> {
        &self.subscriptions
    }
}
