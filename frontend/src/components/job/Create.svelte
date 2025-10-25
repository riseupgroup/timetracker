<script lang="ts">
    import { Button, Modal, Toggle } from "flowbite-svelte";
    import { onMount } from "svelte";
    import InputTrash from "../InputTrash.svelte";
    import { createEventDispatcher } from "svelte";
    import { Job } from "../../app";

    const dispatch = createEventDispatcher();

    export let isOpen = false;

    class NewJob {
        name: string | null = null;
        companyName: string | null = null;
        companyLogo: string | null = null;
        description: string | null = null;
        disabled: boolean = false;
    }

    class NewJobInputs {
        name: string = "";
        companyName: string = "";
        companyLogo: string = "";
        description: string = "";
        disabled: boolean = false;

        toNewJob(): NewJob {
            let newJob = new NewJob();
            newJob.name = this.name.trim().length > 0 ? this.name : null;
            newJob.companyName = this.companyName.trim().length > 0 ? this.companyName : null;
            newJob.companyLogo = this.companyLogo.trim().length > 0 ? this.companyLogo : null;
            newJob.description = this.description.trim().length > 0 ? this.description : null;
            newJob.disabled = this.disabled;
            return newJob;
        }
    }

    let newJobInputs: NewJobInputs;

    onMount(() => {
        resetForm();
    });

    function resetForm() {
        newJobInputs = new NewJobInputs();
    }

    async function submit() {
        let res = await fetch("/api/jobs", {
            method: "POST",
            headers: { "content-type": "application/json" },
            body: JSON.stringify(newJobInputs.toNewJob())
        });

        if (res.ok) {
            isOpen = false;
            resetForm();
            dispatch("created", Object.assign(new Job(), await res.json()));
        } else {
            alert(await res.text());
        }
    }
</script>

<Modal bind:open={isOpen} size="xs" autoclose={false} class="w-full" outsideclose>
    <div class="flex flex-col space-y-6">
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Create Job</h3>
        <InputTrash
            name="Company Name"
            bind:value={newJobInputs.companyName}
            classBackground="bg-white dark:bg-gray-800"
        />
        <InputTrash
            name="Company Logo"
            bind:value={newJobInputs.companyLogo}
            classBackground="bg-white dark:bg-gray-800"
        />
        <InputTrash
            name="Description"
            bind:value={newJobInputs.description}
            classBackground="bg-white dark:bg-gray-800"
        />
        <InputTrash
            name="Name"
            bind:value={newJobInputs.name}
            classBackground="bg-white dark:bg-gray-800"
        />
        <Toggle
            bind:checked={newJobInputs.disabled}
            size="small"
            color="red"
            class="cursor-pointer">Disable Job</Toggle
        >
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
