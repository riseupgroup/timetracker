<script lang="ts">
    import { Button, Modal, Label, Select } from "flowbite-svelte";
    import { onMount } from "svelte";
    import InputTrash from "../InputTrash.svelte";
    import { createEventDispatcher } from "svelte";
    import { DisplayRangeUnit, TimePensumUnit, Tracker } from "../../app";

    const dispatch = createEventDispatcher();

    export let isOpen = false;
    export let job: number | null = null;

    class NewTracker {
        name: string | null = null;
        timePensum: number | null = null;
        timePensumUnit: string | null = null;
        displayRangeUnit: string | null = null;
        validFrom: Date | null = null;
        validUntil: Date | null = null;
    }

    class NewTrackerInputs {
        name: string = "";
        timePensum: string = "";
        timePensumUnit: string = "";
        displayRangeUnit: string = "";
        validFrom: string = "";
        validUntil: string = "";

        toNewTracker(): NewTracker {
            let newTracker = new NewTracker();
            newTracker.name = this.name.trim().length > 0 ? this.name : null;
            newTracker.timePensum = this.timePensum.trim().length > 0 ? Number(this.timePensum) : null;
            this.timePensumUnit = this.timePensumUnit.trim();
            if (this.timePensumUnit != "") newTracker.timePensumUnit = this.timePensumUnit;
            this.displayRangeUnit = this.displayRangeUnit.trim();
            if (this.displayRangeUnit != "") newTracker.displayRangeUnit = this.displayRangeUnit;
            newTracker.validFrom = this.validFrom.trim().length > 0 ? new Date(this.validFrom) : null;
            newTracker.validUntil = this.validUntil.trim().length > 0 ? new Date(this.validUntil) : null;
            return newTracker;
        }
    }

    let newTrackerInputs: NewTrackerInputs;

    onMount(() => {
        resetForm();
    });

    function resetForm() {
        newTrackerInputs = new NewTrackerInputs();
    }

    async function submit() {
        let res = await fetch(job ? "/api/jobs/" + job + "/trackers" : "/api/trackers", {
            method: "POST",
            headers: { "content-type": "application/json" },
            body: JSON.stringify(newTrackerInputs.toNewTracker())
        });

        if (res.ok) {
            isOpen = false;
            resetForm();
            dispatch("created", Object.assign(new Tracker(), await res.json()));
        } else {
            alert(await res.text());
        }
    }
</script>

<Modal bind:open={isOpen} size="xs" autoclose={false} class="w-full" outsideclose>
    <div class="flex flex-col space-y-6">
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Create Tracker</h3>

        <InputTrash name="Name" bind:value={newTrackerInputs.name} classBackground="bg-white dark:bg-gray-800" />

        <InputTrash
            name="Time Pensum (Minutes)"
            type="number"
            bind:value={newTrackerInputs.timePensum}
            classBackground="bg-white dark:bg-gray-800"
        />

        <Label>
            Time Pensum Unit
            <Select
                class="mt-2"
                items={Object.keys(TimePensumUnit)
                    .filter((item) => isNaN(Number(item)))
                    .map((x) => {
                        return { name: x, value: x };
                    })}
                bind:value={newTrackerInputs.timePensumUnit}
            />
        </Label>

        <Label>
            Display Unit
            <Select
                class="mt-2"
                items={Object.keys(DisplayRangeUnit)
                    .filter((item) => isNaN(Number(item)))
                    .map((x) => {
                        return { name: x, value: x };
                    })}
                bind:value={newTrackerInputs.displayRangeUnit}
            />
        </Label>

        <InputTrash
            name="Valid from"
            type="datetime-local"
            bind:value={newTrackerInputs.validFrom}
            classBackground="bg-white dark:bg-gray-800"
        />

        <InputTrash
            name="Valid until"
            type="datetime-local"
            bind:value={newTrackerInputs.validUntil}
            classBackground="bg-white dark:bg-gray-800"
        />

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
