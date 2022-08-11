<script lang="ts">
    import type {DatabaseUsageViewModel} from "../apis/get-database-usage";
    import {getDatabaseUsage} from "../apis/get-database-usage";
    import {onMount} from "svelte";
    import type {DatabaseViewModel, ResourceGroupViewModel, SubscriptionViewModel} from "../apis/get-dashboard";
    import {showError} from "../apis/api-utils";
    import UsageGauge from "./UsageGauge.svelte";
    import LoadingSpinner from "./LoadingSpinner.svelte";

    // The parent subscription
    export let subscription: SubscriptionViewModel
    // The resource-group
    export let resourceGroup: ResourceGroupViewModel
    // The database we're displaying
    export let database: DatabaseViewModel
    // The database usage
    let databaseUsage: DatabaseUsageViewModel|undefined = undefined

    onMount(() => {
        getDatabaseUsage(subscription.subscriptionId, resourceGroup.resourceGroupName, database.serverName, database.databaseName)
            .then(value => {
                console.log(` - got database ${database.serverName}.${database.databaseName} usage`, value)
                databaseUsage = value
            })
            .catch(showError(`Failed to get usage info for database ${database.serverName}.${database.databaseName}`))
    })
</script>

<div class="database card border-0">
    <div class="card-body">
        <h5 class="card-title">{database.serverName}.{database.databaseName}</h5>
        {#if undefined === databaseUsage}
            <LoadingSpinner/>
        {:else}
            <UsageGauge
                used={databaseUsage.databaseSizeUsed}
                allocated={databaseUsage.databaseSizeAllocated}
                total={databaseUsage.databaseSizeMax}
            />
        {/if}
    </div>
</div>

<style lang="scss">

</style>

