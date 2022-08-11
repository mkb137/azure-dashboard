<script lang="ts">
    import {Link} from 'svelte-routing'
    import {onMount} from "svelte";
    import type {DashboardViewModel} from "../apis/get-dashboard";
    import {getDashboard} from "../apis/get-dashboard";
    import {showError} from "../apis/api-utils";
    import Subscription from "./Subscription.svelte";

    // The dashboard loaded from the server.
    let dashboard: DashboardViewModel | undefined = undefined

    onMount(() => {
        getDashboard()
            .then(value => {
                console.log(` - got dashboard `, value)
                dashboard = value
            })
            .catch(showError("Failed to fetch the dashboard from the server"))
    })
</script>
<div class="container-fluid">
    {#if undefined === dashboard}
        <div class="spinner-border text-primary" role="status">
            <span class="visually-hidden">Loading...</span>
        </div>
    {:else}
        <h1>Dashboard</h1>
        {#each dashboard.subscriptions as subscription, i}
            <Subscription {subscription}/>
        {/each}
    {/if}
</div>

