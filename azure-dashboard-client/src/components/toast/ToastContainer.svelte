<script lang="ts">
    import Toast from './Toast.svelte'
    import { toasts } from './toastStores'
</script>

{#if 0 < $toasts.length}
    <!-- Create the container div, which floats in the middle of the page -->
    <div aria-live="polite" aria-atomic="true" class="toast-parent-container">
        <div class="toast-container position-absolute top-0">
            <!-- Show all the toasts as Toast components -->
            {#each $toasts as toastItem (toastItem.id)}
                <Toast {toastItem} />
            {/each}
        </div>
    </div>
{/if}

<style>
    div.toast-parent-container {
        top: 1em;
        left: 50%;
        transform: translateX(-50%);

        z-index: 9999;
        position: fixed;
        padding: 4px;
        width: 320px;
        box-sizing: border-box;
    }
</style>
