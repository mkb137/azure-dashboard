<script lang="ts">
    import type {ElasticPoolUsageViewModel} from "../apis/get-elastic-pool-usage";
    import {getElasticPoolUsage} from "../apis/get-elastic-pool-usage";
    import {onMount} from "svelte";
    import type {ElasticPoolViewModel, ResourceGroupViewModel, SubscriptionViewModel} from "../apis/get-dashboard";
    import {showError} from "../apis/api-utils";
    import UsageGauge from "./UsageGauge.svelte";
    import LoadingSpinner from "./LoadingSpinner.svelte";
    // The parent subscription
    export let subscription: SubscriptionViewModel
    // The resource-group
    export let resourceGroup: ResourceGroupViewModel
    // The elastic pool we're displaying
    export let elasticPool: ElasticPoolViewModel
    // The elastic pool usage
    let elasticPoolUsage: ElasticPoolUsageViewModel|undefined = undefined

    onMount(() => {
        getElasticPoolUsage(subscription.subscriptionId, resourceGroup.resourceGroupName, elasticPool.serverName, elasticPool.elasticPoolName)
            .then(value => {
                console.log(` - got elastic pool ${elasticPool.serverName}.${elasticPool.elasticPoolName} usage`, value)
                elasticPoolUsage = value
            })
            .catch(showError(`Failed to get usage info for elastic pool ${elasticPool.serverName}.${elasticPool.elasticPoolName}`))
    })
</script>

<div class="elastic-pool card border-0">
    <div class="card-body">
        <h5 class="card-title">{elasticPool.elasticPoolName}</h5>
        {#if undefined === elasticPoolUsage}
            <LoadingSpinner/>
        {:else}
            <UsageGauge
                used={elasticPoolUsage.databaseSizeUsed}
                allocated={elasticPoolUsage.databaseSizeAllocated}
                total={elasticPoolUsage.databaseSizeMax}
            />
        {/if}
    </div>
</div>

<style lang="scss">
</style>

