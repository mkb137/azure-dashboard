<script lang="ts">
    import {ToastContainer} from "./components/index.js";
    import {Route, Router} from 'svelte-routing'
    import Dashboard from "./components/Dashboard.svelte";
    import 'bootstrap'
</script>

<main>
    <!-- Add a toast container.  Toasts will pop-up in this div at the top of the document -->
    <ToastContainer />
    <Router>
        <!-- Add routes -->
        <Route path="/">
            <Dashboard/>
        </Route>
    </Router>

</main>
