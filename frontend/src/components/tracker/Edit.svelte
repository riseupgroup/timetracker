<script lang="ts">
    import { Button, Label, Modal, Select } from "flowbite-svelte";
    import { DisplayRangeUnit, getLocalTimestamp, TimePensumUnit, timezone, Tracker } from "../../app";
    import InputTrash from "../InputTrash.svelte";

    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    type UpdateTracker = {
        name: string;
        validFrom: string;
        validUntil: string;
        timePensum: string;
        timePensumUnit: string;
        displayRangeUnit: string;
    };

    let originalTracker: Tracker | null;
    let tracker: UpdateTracker | null;

    
    export function edit(t: Tracker) {
        originalTracker = t;
        tracker = {
            name: t.name || "",
            validFrom: t.validFrom!=null?getLocalTimestamp(new Date(t.validFrom)):"",
            validUntil: t.validUntil!=null?getLocalTimestamp(new Date(t.validUntil)):"",
            timePensum: t.timePensum!=null?String(t.timePensum):"",
            timePensumUnit: t.timePensumUnit,
            displayRangeUnit: t.displayRangeUnit
        };
    }

    async function submit() {
        if (tracker != null && originalTracker != null) {
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            let updateTracker: any = {};
            
            tracker.name = tracker.name.trim();
            let name = tracker.name == ""?null:tracker.name;
            if (name != originalTracker.name) updateTracker.name = tracker.name;

            let validFrom = null;
            if (tracker.validFrom.trim() != "") validFrom = new Date(tracker.validFrom).toISOString();
            if (validFrom != originalTracker.validFrom) updateTracker.validFrom = validFrom;

            let validUntil = null;
            if (tracker.validUntil.trim() != "") validUntil = new Date(tracker.validUntil).toISOString();
            if (validUntil != originalTracker.validUntil) updateTracker.validUntil = validUntil;

            tracker.timePensum = tracker.timePensum.trim();
            let timePensum = tracker.timePensum == ""?null:Number(tracker.timePensum);
            if (timePensum != originalTracker.timePensum) updateTracker.timePensum = timePensum

            if (tracker.timePensumUnit != originalTracker.timePensumUnit) updateTracker.timePensumUnit = tracker.timePensumUnit;
            if (tracker.displayRangeUnit != originalTracker.displayRangeUnit) updateTracker.displayRangeUnit = tracker.displayRangeUnit;

            let res = await fetch(
                originalTracker.resource() + "?tz="+timezone,
                {
                    method: "PATCH",
                    headers: new Headers({ "content-type": "application/json" }),
                    body: JSON.stringify(updateTracker)
                }
            );

            if (res.ok) {
                tracker = null;
                dispatch("update", Object.assign(new Tracker(), await res.json()));
            } else {
                alert(await res.text());
            }
        }
    }
</script>

{#if tracker != null}
    <Modal
        open={true}
        on:close={() => (tracker = null)}
        size="xs"
        autoclose={false}
        class="w-full"
        outsideclose
    >
        <div class="flex flex-col space-y-6">
            <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
                Edit Tracker {originalTracker != null ? (": " + originalTracker.display()) : ""}
            </h3>

            <InputTrash
                name="Name"
                bind:value={tracker.name}
                classBackground="bg-white dark:bg-gray-800"
            />

            <Label>
                Time Pensum Unit
                <Select class="mt-2" items={
                    Object.keys(TimePensumUnit).filter(
                            item => isNaN(Number(item))
                        ).map(
                            x => {
                                return { name: x, value: x };
                            }
                        )
                    } 
                    bind:value={tracker.timePensumUnit} 
                />
            </Label>

            <Label>
                Display Unit
                <Select class="mt-2" items={
                    Object.keys(DisplayRangeUnit).filter(
                            item => isNaN(Number(item))
                        ).map(
                            x => {
                                return { name: x, value: x };
                            }
                        )
                    } 
                    bind:value={tracker.displayRangeUnit} 
                />
            </Label>

            <InputTrash
                name="Valid from"
                type="datetime-local"
                bind:value={tracker.validFrom}
                classBackground="bg-white dark:bg-gray-800"
            />

            <InputTrash
                name="Valid until"
                type="datetime-local"
                bind:value={tracker.validUntil}
                classBackground="bg-white dark:bg-gray-800"
            />

            <InputTrash
                name="Time pensum (Minutes)"
                type="number"
                bind:value={tracker.timePensum}
                classBackground="bg-white dark:bg-gray-800"
            />

            <div class="flex justify-end space-x-2">
                <Button on:click={() => (tracker = null)} outline>Cancel</Button>
                <Button on:click={submit}>Save</Button>
            </div>
        </div>
    </Modal>
{/if}