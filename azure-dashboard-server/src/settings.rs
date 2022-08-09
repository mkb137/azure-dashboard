// The settings relating to a single subscription.
#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct SubscriptionSettings {
    // The Azure OAuth2 base auth URL, e.g. https://login.microsoftonline.com/TENANT_ID/oauth2/token
    token_url: String,
    // The tenant ID (a GUID)
    tenant_id: String,
    // The subscription ID (a GUID)
    subscription_id: String,
    // The Azure AD App Registration client ID (a GUID)
    client_id: String,
    // The Azure AD App Registration secret value (a long string of random chars)
    // Note: Not the secret ID (a GUID)
    // Note2: This secret expires 6, 12, or however many months were specified at time of
    // creation and will have to be updated.
    client_secret: String,
    // The OAuth2 resource name, e.g. "https://management.azure.com"
    resource: String,
}
impl SubscriptionSettings {
    pub fn token_url(&self) -> String {
        self.token_url.clone()
    }
    pub fn tenant_id(&self) -> String {
        self.tenant_id.clone()
    }
    pub fn subscription_id(&self) -> String {
        self.subscription_id.clone()
    }
    pub fn client_id(&self) -> String {
        self.client_id.clone()
    }
    pub fn client_secret(&self) -> String {
        self.client_secret.clone()
    }
    pub fn resource(&self) -> String {
        self.resource.clone()
    }
}

// The application configuration settings.
#[derive(Debug, serde::Deserialize)]
#[allow(unused)]
pub struct Settings {
    // The port we'll run on
    port: u16,
    // The subscriptions.
    subscriptions: Vec<SubscriptionSettings>,
}

impl Settings {
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
    // The configured port.
    pub fn port(&self) -> u16 {
        self.port
    }
    // The subscriptions
    pub fn subscriptions(&self) -> &Vec<SubscriptionSettings> {
        &self.subscriptions
    }
}
