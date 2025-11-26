<script lang="ts">
    import { Button, Modal, Toggle, FloatingLabelInput } from "flowbite-svelte";
    import { Timeslot, getLocalTimestamp } from "../../app";
    import { createEventDispatcher } from "svelte";
    import InputTrash from "../InputTrash.svelte";
    import TextArea from "../TextArea.svelte";

    const dispatch = createEventDispatcher();

    type UpdateTimeslot = {
        start: string;
        end: string;
        comment: string;
    };

    let originalTimeslot: Timeslot | null;
    let timeslot: UpdateTimeslot | null;
    let commentRows: number;

    export function edit(t: Timeslot) {
        originalTimeslot = t;
        timeslot = {
            start: getLocalTimestamp(new Date(t.start)),
            end: t.end ? getLocalTimestamp(new Date(t.end)) : "",
            comment: t.comment || ""
        };
        if (t.comment != null && t.comment.trim() != "") {
            let matches = t.comment.trim().match(/\n/g) || [];
            commentRows = matches.length + 1;
        }
        commentRows = Math.min(Math.max(commentRows, 3), 10);
    }

    async function submit() {
        if (timeslot != null && originalTimeslot != null) {
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            let updateTimeslot: any = {};

            timeslot.start = timeslot.start.trim();
            let start = timeslot.start != "" ? new Date(timeslot.start).toISOString() : null;
            if (start != originalTimeslot.start) updateTimeslot.start = start;

            timeslot.end = timeslot.end.trim();
            let end = timeslot.end != "" ? new Date(timeslot.end).toISOString() : null;
            if (end != originalTimeslot.end) updateTimeslot.end = end;

            timeslot.comment = timeslot.comment.trim();
            let comment = timeslot.comment == "" ? null : timeslot.comment;
            if (comment != originalTimeslot.comment) updateTimeslot.comment = timeslot.comment;

            let res = await fetch(originalTimeslot.resource(), {
                method: "PATCH",
                headers: new Headers({ "content-type": "application/json" }),
                body: JSON.stringify(updateTimeslot)
            });

            if (res.ok) {
                timeslot = null;
                dispatch("update", Object.assign(new Timeslot(), await res.json()));
            } else {
                alert(await res.text());
            }
        }
    }

    function toggleEnded(e: Event) {
        if (timeslot && e.target) {
            if ((e.target as HTMLInputElement).checked) {
                timeslot.end = getLocalTimestamp(new Date());
            } else {
                timeslot.end = "";
            }
        }
    }
</script>

{#if timeslot != null}
    <Modal open={true} on:close={() => (timeslot = null)} size="xs" autoclose={false} class="w-full" outsideclose>
        <div class="flex flex-col space-y-6">
            <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Edit Timeslot</h3>
            <TextArea rows={commentRows} bind:text={timeslot.comment} placeholder="Leave a comment">Comment</TextArea>
            <FloatingLabelInput
                type="datetime-local"
                bind:value={timeslot.start}
                style="outlined"
                classLabel="cursor-text bg-white dark:bg-gray-800"
            >
                Start
            </FloatingLabelInput>
            <Toggle checked={timeslot.end != ""} on:change={toggleEnded}>End timeslot</Toggle>
            {#if timeslot.end != ""}
                <InputTrash name="End" type="datetime-local" bind:value={timeslot.end} classBackground="bg-white dark:bg-gray-800" />
            {/if}
            <div class="flex justify-end space-x-2">
                <Button on:click={() => (timeslot = null)} outline>Cancel</Button>
                <Button on:click={submit}>Save</Button>
            </div>
        </div>
    </Modal>
{/if}
