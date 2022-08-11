// Settings for a database to be displayed in the dashboard.
import {getAsync} from "./api-utils";

export type DatabaseViewModel = {
    // The server name
    serverName: string,
    // The database name
    databaseName: string,
}

// Settings for an elastic pool to be displayed in the dashboard.
export type ElasticPoolViewModel = {
    // The server name
    serverName: string,
    // The elastic pool name
    elasticPoolName: string,
}

// Settings for a resource group.
export type ResourceGroupViewModel = {
    // The resource group name
    resourceGroupName: string,
    // The databases to be displayed in the dashboard
    databases: DatabaseViewModel[],
    // The elastic pools to be displayed in the dashboard
    elasticPools: ElasticPoolViewModel[],
}

// The settings relating to a single subscription.
export type SubscriptionViewModel = {
    // The display name for this subscription
    displayName: string,
    // The subscription ID (a GUID)
    subscriptionId: string,
    // The resource groups in the subscription.
    resourceGroups: ResourceGroupViewModel[],
}

// The dashboard settings.
export type DashboardViewModel = {
    subscriptions: SubscriptionViewModel[],
}

// Fetches the dashboard from the server.
export const getDashboard = (): Promise<DashboardViewModel> => getAsync<DashboardViewModel>("api/dashboard")
