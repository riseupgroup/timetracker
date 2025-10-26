<script lang="ts">
    import { Button, Modal, Toggle } from "flowbite-svelte";
    import { onMount } from "svelte";
    import { ApiKeyResponse, getLocalTimestamp } from "../../app";
    import InputTrash from "../InputTrash.svelte";
    import { createEventDispatcher } from "svelte";
    import SecretModal from "./SecretModal.svelte";

    const dispatch = createEventDispatcher();

    export let isOpen = false;
    let newKey: ApiKeyResponse | null = null;

    export class NewApiKey {
        name: string | null = null;
        disabled: boolean = false;
        validUntil: string | null = null;
    }

    class NewApiKeyInputs {
        name: string = "";
        validUntil: string;
        disabled: boolean = false;

        constructor() {
            let date = new Date();
            date.setFullYear(date.getFullYear() + 1);
            this.validUntil = getLocalTimestamp(date);
        }

        toNewApiKey(): NewApiKey {
            let newApiKey = new NewApiKey();
            newApiKey.name = this.name.length > 0 ? this.name : null;
            newApiKey.validUntil = this.validUntil != "" ? new Date(this.validUntil).toISOString() : null;
            newApiKey.disabled = this.disabled;
            return newApiKey;
        }
    }

    let newApiKeyInputs: NewApiKeyInputs;

    onMount(() => {
        resetForm();
    });

    function resetForm() {
        newApiKeyInputs = new NewApiKeyInputs();
        newKey = null;
    }

    async function submit() {
        let res = await fetch("/api/keys", {
            method: "POST",
            headers: { "content-type": "application/json" },
            body: JSON.stringify(newApiKeyInputs.toNewApiKey())
        });

        if (res.ok) {
            isOpen = false;
            resetForm();
            dispatch("created");
            newKey = await res.json();
        } else {
            alert(await res.text());
        }
    }
</script>

<Modal bind:open={isOpen} size="xs" autoclose={false} class="w-full" outsideclose>
    <div class="flex flex-col space-y-6">
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Create API key</h3>

        <InputTrash name="Name" bind:value={newApiKeyInputs.name} classBackground="bg-white dark:bg-gray-800" />

        <InputTrash
            name="Valid until"
            bind:value={newApiKeyInputs.validUntil}
            type="datetime-local"
            classBackground="bg-white dark:bg-gray-800"
        />

        <Toggle bind:checked={newApiKeyInputs.disabled} size="small" class="cursor-pointer">Disable</Toggle>

        <div class="flex justify-end space-x-2">
            <Button
                on:click={() => {
                    isOpen = false;
                    resetForm();
                }}
                outline>Cancel</Button
            >
            <Button on:click={submit}>Create</Button>
        </div>
    </div>
</Modal>

<SecretModal bind:key={newKey} />
