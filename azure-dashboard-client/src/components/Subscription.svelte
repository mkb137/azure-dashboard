<script lang="ts">

    import type {SubscriptionViewModel} from "../apis/get-dashboard";
    import ResourceGroup from "./ResourceGroup.svelte";

    // The subscription we're displaying
    export let subscription: SubscriptionViewModel
</script>
<div class="subscription">
    <h2>{subscription.displayName}</h2>
    {#each subscription.resourceGroups as resourceGroup}
        <ResourceGroup {subscription} {resourceGroup}/>
    {/each}
</div>
