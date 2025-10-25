<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import type { Unsubscriber, Writable } from "svelte/store";
    import { MouseClick } from "../app";

    const dispatch = createEventDispatcher();
    export let content: string | Writable<string>;
    export let clickable: boolean = false;
    let element: HTMLElement;
    let unsubscriber: Unsubscriber | null;

    $: content, update();

    function update() {
        if (unsubscriber != null) {
            unsubscriber();
            unsubscriber = null;
        }
        if (typeof content == "string") {
            if (element) element.innerText = content;
        } else {
            unsubscriber = content.subscribe(text => {
                if (element) element.innerText = text;
            });
        }
    }

    onMount(update);
</script>

{#if clickable}
    <td
        class="whitespace-nowrap p-4"
        on:mousedown={(e) => {
            e.stopPropagation();
            dispatch("click", new MouseClick(e));
        }}
        on:click={e => e.stopPropagation()}
    >
        <span class="cursor-pointer hover:underline" bind:this={element}></span>
    </td>
{:else}
    <td class="whitespace-nowrap p-4" bind:this={element}></td>
{/if}
