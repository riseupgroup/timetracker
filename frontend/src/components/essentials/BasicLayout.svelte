<script lang="ts">
    export let navbar: boolean = true;
    export let footer: boolean = true;
    export let hide_footer: boolean = true;

    import Navbar from "../../components/essentials/Navbar.svelte";
    import Footer from "../../components/essentials/Footer.svelte";
</script>

<div class="page-container bg-white text-gray-600 dark:bg-gray-900 dark:text-gray-400">
    {#if navbar}
        <Navbar />
    {/if}
    <div class="content-container">
        <main
            class="page-content container mx-auto px-4 pt-3 md:pt-6"
            style={hide_footer ? "flex-basis: 100%" : ""}
        >
            <slot />
        </main>
        {#if footer}
            <Footer />
        {/if}
    </div>
</div>

<style lang="scss">
    .page-container {
        position: fixed;
        inset: 0;

        display: flex;
        flex-direction: column;
        align-items: stretch;
        justify-content: stretch;
    }

    .content-container {
        height: 0;
        flex-grow: 1;
        overflow: auto;

        display: flex;
        flex-direction: column;
        align-items: stretch;
        justify-content: stretch;
    }

    .page-content {
        flex-shrink: 0;
        flex-grow: 1;
        padding-bottom: 2em;
    }
</style>
