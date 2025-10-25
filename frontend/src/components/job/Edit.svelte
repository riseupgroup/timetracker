<script lang="ts">
    import { Button, Label, Modal, Select, Toggle } from "flowbite-svelte";
    import { Job, Tracker, timezone } from "../../app";
    import InputTrash from "../InputTrash.svelte";

    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    type UpdateJob = {
        name: string;
        companyName: string;
        companyLogo: string;
        description: string;
        disabled: boolean;
        activeTracker: number;
    };

    let originalJob: Job | null;
    let trackerList: { name: string; value: number }[] = [];
    let job: UpdateJob | null;

    /**
     * Opens the edit modal for a job
     * 
     * @param oldJob The job to edit
     * @param trackers The trackers to display in the active tracker select
     */
    export async function edit(oldJob: Job, trackers: Tracker[] | null) {
        originalJob = oldJob;
        if (trackers == null) {
            trackers = await fetchTrackers(oldJob);
        }
        trackerList = trackers.map((t) => {
            return { name: t.display(), value: t.id };
        });
        trackerList.unshift({ name: "None", value: 0 });
        job = {
            name: oldJob.name || "",
            companyName: oldJob.companyName || "",
            companyLogo: oldJob.companyLogo || "",
            description: oldJob.description || "",
            disabled: oldJob.disabled,
            activeTracker: oldJob.activeTracker != null?oldJob.activeTracker.id:0,
        };
    }

    async function fetchTrackers(job: Job): Promise<Tracker[]> {
        let res = await fetch("/api/jobs/" + job.id + "/trackers");
        if (res.ok) {
            let trackers: Tracker[] = await res.json();
            for (let i = 0; i < trackers.length; i += 1) {
                trackers[i] = Object.assign(new Tracker(), trackers[i]);
            }
            return trackers;
        } else {
            alert(await res.text());
            return [];
        }
    }

    async function submit() {
        if (job != null && originalJob != null) {
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            let updateJob: any = {};
            
            job.name = job.name.trim();
            let name = job.name == ""?null:job.name;
            if (name != originalJob.name) updateJob.name = job.name;

            job.companyName = job.companyName.trim();
            let companyName = job.companyName == ""?null:job.companyName;
            if (companyName != originalJob.companyName) updateJob.companyName = job.companyName;

            job.companyLogo = job.companyLogo.trim();
            let companyLogo = job.companyLogo == ""?null:job.companyLogo;
            if (companyLogo != originalJob.companyLogo) updateJob.companyLogo = job.companyLogo;

            job.description = job.description.trim();
            let description = job.description == ""?null:job.description;
            if (description != originalJob.description) updateJob.description = job.description;

            let activeTracker = job.activeTracker==0?null:(job.activeTracker);
            if (activeTracker != originalJob.activeTracker) updateJob.activeTracker = activeTracker;        
            
            if (job.disabled != originalJob.disabled) updateJob.disabled = job.disabled;

            let res = await fetch(
                originalJob.resource() + "?tz=" + timezone,
                {
                    method: "PATCH",
                    headers: new Headers({ "content-type": "application/json" }),
                    body: JSON.stringify(updateJob)
                }
            );

            if (res.ok) {
                job = null;
                let resJob = Object.assign(new Job(), await res.json());
                resJob.activeTracker = resJob.activeTracker != null ? Object.assign(new Tracker(), resJob.activeTracker) : null;
                dispatch("update", resJob);
            } else {
                alert(await res.text());
            }
        }
    }
</script>

{#if job != null}
    <Modal
        open={true}
        on:close={() => (job = null)}
        size="xs"
        autoclose={false}
        class="w-full"
        outsideclose
    >
        <div class="flex flex-col space-y-6">
            <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
                Edit Job {originalJob != null ? (": " + originalJob.display()) : ""}
            </h3>
            <InputTrash
                name="Company name"
                bind:value={job.companyName}
                classBackground="bg-white dark:bg-gray-800"
            />
            <InputTrash
                name="Company logo"
                bind:value={job.companyLogo}
                classBackground="bg-white dark:bg-gray-800"
            />
            <InputTrash
                name="Description"
                bind:value={job.description}
                classBackground="bg-white dark:bg-gray-800"
            />
            <InputTrash
                name="Name"
                bind:value={job.name}
                classBackground="bg-white dark:bg-gray-800"
            />
            <Label>
                Active Tracker
                <Select
                    name="Active tracker"
                    bind:value={job.activeTracker}
                    bind:items={trackerList}
                    class="mt-2"
                    classBackground="bg-white dark:bg-gray-800"
                />
            </Label>
            <Toggle bind:checked={job.disabled} size="small" class="cursor-pointer">
                Disabled
            </Toggle>
            <div class="flex justify-end space-x-2">
                <Button on:click={() => (job = null)} outline>Cancel</Button>
                <Button on:click={submit}>Save</Button>
            </div>
        </div>
    </Modal>
{/if}