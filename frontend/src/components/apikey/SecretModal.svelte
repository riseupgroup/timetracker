<script lang="ts">
    import { Button, Modal } from "flowbite-svelte";
    import { ApiKeyResponse } from "../../app";
    import Tooltip from "../Tooltip.svelte";
    import { EyeSolid, EyeSlashSolid, FileCopySolid, TerminalOutline } from "flowbite-svelte-icons";

    export let key: ApiKeyResponse | null;
    let show: boolean = false;
    let copiedTooltipOpen: boolean = false;

    async function copyToClipboard() {
        if (key == null) return;

        if (navigator.clipboard == undefined) {
            alert("Clipboard API not available");
            return;
        }

        navigator.clipboard.writeText(key.id + ":" + key.key).then(
            () => {
                copiedTooltipOpen = true;
                setTimeout(() => (copiedTooltipOpen = false), 3000);
            },
            (err) => {
                alert("Failed to copy to clipboard: " + err);
            }
        );
    }

    function close() {
        key = null;
        show = false;
    }
</script>

{#if key != null}
    <Modal open={true} on:close={close} size="xs" autoclose={false} class="w-full" outsideclose={false}>
        <div class="flex flex-col">
            <TerminalOutline class="mx-auto mb-2 h-12 w-12 text-gray-400 dark:text-gray-200" />
            <div class="mt-6 text-center font-mono text-primary-600 dark:text-primary-500" style="word-break: break-all;">
                {key.id}:{show ? key.key : "******-******-******-******-******"}
            </div>
            <div class="flex flex-row justify-center">
                <button
                    class="rounded-md px-2 py-1 text-gray-500 outline-none focus-within:ring-4 focus-within:ring-gray-300 hover:text-primary-600 dark:border-gray-400 dark:text-gray-400 dark:focus-within:ring-gray-600 dark:hover:text-primary-500"
                    id="copy-button"
                    on:click={() => (show = !show)}
                >
                    {#if show}
                        <EyeSolid />
                    {:else}
                        <EyeSlashSolid />
                    {/if}
                </button>
                <Tooltip bind:open={copiedTooltipOpen}>
                    <button
                        class="rounded-md px-2 py-1 text-gray-500 outline-none focus-within:ring-4 focus-within:ring-gray-300 dark:border-gray-400 dark:text-gray-400 dark:focus-within:ring-gray-600"
                        id="copy-button"
                        on:click={copyToClipboard}
                    >
                        <FileCopySolid class="h-5 w-5 hover:text-primary-600 dark:hover:text-primary-500" />
                    </button>
                    <div slot="content">Copied!</div>
                </Tooltip>
            </div>
            <h3 class="mt-6 text-center text-lg font-normal text-gray-500 dark:text-gray-400">
                This secret is only shown <span class="text-primary-600 dark:text-primary-500">once</span>.<br />Please make sure to
                <span class="text-primary-600 dark:text-primary-500">copy</span> it now.
            </h3>

            <Button on:click={close} class="mt-6">Close</Button>
        </div>
    </Modal>
{/if}
