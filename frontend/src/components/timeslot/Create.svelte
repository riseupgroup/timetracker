<script lang="ts">
    import { Button, Modal } from "flowbite-svelte";
    import { onMount } from "svelte";
    import InputTrash from "../InputTrash.svelte";
    import { createEventDispatcher } from "svelte";
    import { getLocalTimestamp, Timeslot } from "../../app";
    import TextArea from "../TextArea.svelte";

    const dispatch = createEventDispatcher();

    export let isOpen = false;
    export let api: string;

    type NewTimeslot = {
        start: Date | null;
        end: Date | null;
        comment: string | null;
    };

    class NewTimeslotInputs {
        start: string = "";
        end: string = "";
        comment: string = "";

        constructor() {
            this.start = getLocalTimestamp(new Date());
        }

        toNewTimeslot(): NewTimeslot {
            return {
                start: this.start.trim().length > 0 ? new Date(this.start) : null,
                end: this.end.trim().length > 0 ? new Date(this.end) : null,
                comment: this.comment.trim().length > 0 ? this.comment : null
            };
        }
    }

    let newTimeslotInputs: NewTimeslotInputs;

    onMount(() => {
        resetForm();
    });

    function resetForm() {
        newTimeslotInputs = new NewTimeslotInputs();
    }

    async function submit() {
        let res = await fetch(api, {
            method: "POST",
            headers: { "content-type": "application/json" },
            body: JSON.stringify(newTimeslotInputs.toNewTimeslot())
        });

        if (res.ok) {
            isOpen = false;
            resetForm();
            dispatch("created", Object.assign(new Timeslot(), await res.json()));
        } else {
            alert(await res.text());
        }
    }
</script>

<Modal bind:open={isOpen} size="xs" autoclose={false} class="w-full" outsideclose>
    <div class="flex flex-col space-y-6">
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Create Timeslot</h3>

        <TextArea bind:text={newTimeslotInputs.comment} placeholder="Leave a comment">Comment</TextArea>

        <InputTrash name="Start" bind:value={newTimeslotInputs.start} type="datetime-local" classBackground="bg-white dark:bg-gray-800" />

        <InputTrash name="End" bind:value={newTimeslotInputs.end} type="datetime-local" classBackground="bg-white dark:bg-gray-800" />

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
