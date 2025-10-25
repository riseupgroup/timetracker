<script lang="ts">
    import  Page from "../../../../components/tracker/Page.svelte";
    import { page } from "$app/stores";
    let tracker = $page.params.tracker;
</script>

<Page {tracker}/>
