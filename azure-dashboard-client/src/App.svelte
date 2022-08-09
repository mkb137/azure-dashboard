<script lang="ts">
    import svelteLogo from './assets/svelte.svg'
    import Counter from './lib/Counter.svelte'
    import {ToastContainer} from "./components/index.js";
    import SubPage from "./components/SubPage.svelte";
    import {Router, Route, Link} from 'svelte-routing'
    import Home from "./components/Home.svelte";
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
