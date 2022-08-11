import {getAsync} from "./api-utils";

export type DatabaseUsageViewModel = {
    // The amount of data used
    databaseSizeUsed: number,
    // The amount of data allocated
    databaseSizeAllocated: number,
    // The maximum size of the database
    databaseSizeMax: number,
}

// Fetches a database from the server.
export const getDatabaseUsage = (subscriptionId: string, resourceGroupName: string, serverName: string, databaseName: string): Promise<DatabaseUsageViewModel> =>
    getAsync<DatabaseUsageViewModel>(`/api/subscription/${subscriptionId}/resource-group/${resourceGroupName}/server/${serverName}/database/${databaseName}/usage`)
