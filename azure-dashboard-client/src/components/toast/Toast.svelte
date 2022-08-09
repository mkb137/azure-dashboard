<script lang="ts">
    import * as bootstrap from 'bootstrap'
    import { onMount } from 'svelte'

    import type { ToastItem } from './toastStores'
    import { removeToast, ToastType } from './toastStores'

    // The toast item we're showing.
    export let toastItem: ToastItem | undefined = undefined
    // Any additional toast options
    export let toastOptions: bootstrap.Toast.Options = { animation: true, autohide: true, delay: 5000 }
    // The div element that will be the toast
    let toastDiv: HTMLElement | undefined = undefined
    // The div element that will be the progress, if we have it
    let progressDiv: HTMLDivElement | undefined = undefined
    // The bootstrap toast object
    let bsToast: bootstrap.Toast | undefined = undefined

    // The classes for our toast types
    const TOAST_CLASS_DEFAULT = 'toast bg-body text-body'
    const TOAST_CLASS_WARNING = 'toast bg-danger text-white'
    const PROGRESS_CLASS_DEFAULT = 'progress-bar progress-bar-striped progress-bar-animated'
    const PROGRESS_CLASS_WARNING = 'progress-bar progress-bar-striped progress-bar-animated bg-danger'

    // Called when the toast is hidden.
    const handleToastHidden = () => {
        if (toastDiv) {
            // Stop listening to when it's hidden
            toastDiv.removeEventListener('hidden.bs.toast', handleToastHidden)
        }
        // If we have the bootstrap toast (and we should)...
        if (bsToast) {
            // Dispose of the toast
            bsToast.dispose()
        }
        // If we have an item (and we should)...
        if (toastItem) {
            // Remove the toast, which will cause it to be un-rendered and onDestroy to be called.
            removeToast(toastItem)
        }
    }

    // An animation (via in:shrink) that shrinks the width of the progress bar to show how the toast is closing
    const shrink = (node: HTMLElement) => {
        return {
            duration: toastOptions.delay,
            tick: t => {
                let width = 100.0 * (1.0 - t)
                node.style.width = `${width}%`
            }
        }
    }

    // Listen to when the div is mounted.
    onMount(() => {
        if (toastDiv && toastItem) {
            // Update the class depending on the toast type
            toastDiv.className = ToastType.Default === toastItem.toastType ? TOAST_CLASS_DEFAULT : TOAST_CLASS_WARNING
            // Create the toast
            bsToast = new bootstrap.Toast(toastDiv, toastOptions)
            // Show it
            bsToast.show()
            // Listen to when it's hidden
            toastDiv.addEventListener('hidden.bs.toast', handleToastHidden)
            // If we have a progress div...
            if (progressDiv) {
                // Update its class depending on the type
                progressDiv.className = ToastType.Default === toastItem.toastType ? PROGRESS_CLASS_DEFAULT : PROGRESS_CLASS_WARNING
            }
        }
    })
</script>

<!-- Create the toast item -->
<div bind:this={toastDiv} class="toast" role="alert" aria-live="assertive" aria-atomic="true" data-bs-dismiss="toast">
    <div class="toast-body">
        <!-- Split any multi-line message into lines -->
        {#if toastItem.message}
            {#each toastItem.message?.split('\n') as line}
                <div>{line}</div>
            {/each}
        {/if}
        {#if toastOptions.autohide}
            <div class="progress mt-3">
                <div
                    in:shrink
                    bind:this={progressDiv}
                    class="progress-bar"
                    style="width: 100%"
                    role="progressbar"
                    aria-valuenow="100"
                    aria-valuemin="0"
                    aria-valuemax="100"
                />
            </div>
        {/if}
    </div>
</div>

<style>
    div.progress {
        height: 5px;
    }
</style>
