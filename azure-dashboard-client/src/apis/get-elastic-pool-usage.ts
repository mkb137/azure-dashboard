import {getAsync} from "./api-utils";

export type ElasticPoolUsageViewModel = {
    // The amount of data used
    databaseSizeUsed: number,
    // The amount of data allocated
    databaseSizeAllocated: number,
    // The maximum size of the elasticPool
    databaseSizeMax: number,
}

// Fetches an elastic pool from the server.
export const getElasticPoolUsage = (subscriptionId: string, resourceGroupName: string, serverName: string, elasticPoolName: string): Promise<ElasticPoolUsageViewModel> =>
    getAsync<ElasticPoolUsageViewModel>(`api/subscription/${subscriptionId}/resource-group/${resourceGroupName}/server/${serverName}/elastic-pool/${elasticPoolName}/usage`)
