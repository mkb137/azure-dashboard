<script lang="ts">

    import type {ResourceGroupViewModel, SubscriptionViewModel} from "../apis/get-dashboard";
    import Database from "./Database.svelte";
    import ElasticPool from "./ElasticPool.svelte";
    // The parent subscription
    export let subscription: SubscriptionViewModel
    // The resource-group we're displaying
    export let resourceGroup: ResourceGroupViewModel
</script>
<div class="resource-group">
    <h3>{resourceGroup.resourceGroupName}</h3>
    {#each resourceGroup.databases as database, i}
        <Database {subscription} {resourceGroup} {database}/>
    {/each}
    {#each resourceGroup.elasticPools as elasticPool, i}
        <ElasticPool {subscription} {resourceGroup} {elasticPool}/>
    {/each}
</div>
