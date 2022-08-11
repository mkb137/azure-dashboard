import {getAsync} from "./api-utils";

export type ElasticPoolUsageViewModel = {
    // The amount of data used
    elasticPoolSizeUsed: number,
    // The amount of data allocated
    elasticPoolSizeAllocated: number,
    // The maximum size of the elasticPool
    elasticPoolSizeMax: number,
}

// Fetches an elastic pool from the server.
export const getElasticPoolUsage = (subscriptionId: string, resourceGroupName: string, serverName: string, elasticPoolName: string): Promise<ElasticPoolUsageViewModel> =>
    getAsync<ElasticPoolUsageViewModel>(`api/subscription/${subscriptionId}/resource-group/${resourceGroupName}/server/${serverName}/elastic-pool/${elasticPoolName}/usage`)
