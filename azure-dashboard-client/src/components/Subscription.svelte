<script lang="ts">

    import type {SubscriptionViewModel} from "../apis/get-dashboard";
    import ResourceGroup from "./ResourceGroup.svelte";

    // The subscription we're displaying
    export let subscription: SubscriptionViewModel
</script>

<div class="subscription">
    <h2 class="mt-3 mx-3 px-3">{subscription.displayName}</h2>
    <div class="card-container">
        {#each subscription.resourceGroups as resourceGroup}
            <ResourceGroup {subscription} {resourceGroup}/>
        {/each}
    </div>
</div>

<style lang="scss">
    div.card-container {
        display: flex;
        flex-wrap: wrap;
        flex-direction: row;
    }
</style>

