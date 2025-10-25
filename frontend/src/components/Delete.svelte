<script lang="ts">
    import { Button, Modal } from "flowbite-svelte";
    import { ExclamationCircleOutline } from "flowbite-svelte-icons";
    import type { Entity } from "../app";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();
    export let isOpen: boolean = false;
    export let entity: Entity;

    async function submit() {
        isOpen = false;
        let res = await fetch(entity.resource(), { method: "DELETE" });
        if (res.ok) {
            dispatch("deleted", entity);
        } else {
            alert(await res.text());
        }
    }
</script>

<Modal bind:open={isOpen} size="xs" autoclose={false} class="w-full" outsideclose>
    <div class="text-center">
        <ExclamationCircleOutline class="mx-auto mb-4 h-12 w-12 text-gray-400 dark:text-gray-200" />
        <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
            Are you sure you want to delete this {entity.resourceName()}?<br /><span
                class="text-primary-600 dark:text-primary-500"
                style="word-break: break-all;">{entity.display()}</span
            >
        </h3>
        <Button color="alternative" on:click={() => (isOpen = false)}>No, cancel</Button>
        <Button color="red" class="me-2" on:click={submit} autofocus>Yes, I'm sure</Button>
    </div>
</Modal>
