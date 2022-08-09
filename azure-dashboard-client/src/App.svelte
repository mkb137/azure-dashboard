<script lang="ts">
    import {ToastContainer} from "./components/index.js";
    import SubPage from "./components/SubPage.svelte";
    import {Route, Router} from 'svelte-routing'
    import Home from "./components/Home.svelte";
    import 'bootstrap'
</script>

<main>
    <!-- Add a toast container.  Toasts will pop-up in this div at the top of the document -->
    <ToastContainer />
    <Router>
        <!-- Add routes -->
        <Route path="/">
            <Home/>
        </Route>
        <Route path="/sub-page">
            <SubPage/>
        </Route>
    </Router>

</main>
