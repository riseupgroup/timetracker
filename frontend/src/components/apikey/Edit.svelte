<script lang="ts">
    import { Button, Modal, Toggle } from "flowbite-svelte";
    import { ApiKey, ApiKeyResponse, getLocalTimestamp } from "../../app";
    import InputTrash from "../InputTrash.svelte";
    import SecretModal from "./SecretModal.svelte";

    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    type UpdateKey = {
        name: string;
        disabled: boolean;
        regenerate: boolean;
        validUntil: string;
    };

    let originalKey: ApiKey | null;
    let key: UpdateKey | null;
    let createResponse: ApiKeyResponse | null;

    export function edit(k: ApiKey) {
        originalKey = k;
        key = {
            name: k.name || "",
            disabled: k.disabled,
            regenerate: false,
            validUntil: k.validUntil != null ? getLocalTimestamp(new Date(k.validUntil)) : ""
        };
    }

    async function submit() {
        if (key != null && originalKey != null) {
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            let updateKey: any = {};
            let name = null;
            if (key.name.trim() != "") name = key.name.trim();
            if (name != originalKey.name) updateKey.name = name;
            if (key.disabled != originalKey.disabled) updateKey.disabled = key.disabled;
            updateKey.regenerate = key.regenerate;

            let validUntil = null;
            if (key.validUntil.trim() != "") validUntil = new Date(key.validUntil).toISOString();
            if (validUntil != originalKey.validUntil) updateKey.validUntil = validUntil;

            let res = await fetch("/api/keys/" + originalKey.id, {
                method: "PATCH",
                headers: new Headers({ "content-type": "application/json" }),
                body: JSON.stringify(updateKey)
            });
            if (res.ok) {
                if (key.regenerate) {
                    createResponse = await res.json();
                }
                key = null;
                dispatch("update");
            } else {
                alert(await res.text());
            }
        }
    }
</script>

{#if key != null}
    <Modal
        open={true}
        on:close={() => (key = null)}
        size="xs"
        autoclose={false}
        class="w-full"
        outsideclose
    >
        <div class="flex flex-col space-y-6">
            <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
                Edit Key {originalKey != null ? (": " + originalKey.display()) : ""}
            </h3>

            <InputTrash
                name="Name"
                bind:value={key.name}
                classBackground="bg-white dark:bg-gray-800"
            />

            <InputTrash
                name="Valid until"
                bind:value={key.validUntil}
                type="datetime-local"
                classBackground="bg-white dark:bg-gray-800"
            />

            <Toggle bind:checked={key.disabled} size="small" class="cursor-pointer">
                Disabled
            </Toggle>

            <Toggle bind:checked={key.regenerate} size="small" class="cursor-pointer">
                Regenerate
            </Toggle>

            <div class="flex justify-end space-x-2">
                <Button on:click={() => (key = null)} outline>Cancel</Button>
                <Button on:click={submit}>Save</Button>
            </div>
        </div>
    </Modal>
{/if}

<SecretModal bind:key={createResponse} />
