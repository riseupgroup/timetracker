<script lang="ts">
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { browserLanguage, formatDuration, Job, ListResult, MouseClick, TimePensumUnit, timezone, Tracker } from "../../../../app";
    import { Heading, Badge } from "flowbite-svelte";
    import { EditSolid, TrashBinSolid } from "flowbite-svelte-icons";
    import Delete from "../../../../components/Delete.svelte";
    import SmallCard from "../../../../components/SmallCard.svelte";
    import SearchTable from "../../../../components/SearchTable.svelte";
    import Edit from "../../../../components/job/Edit.svelte";
    import Create from "../../../../components/tracker/Create.svelte";
    import EditTracker from "../../../../components/tracker/Edit.svelte";

    let paramJob = $page.params.job;

    let job: Job | null = null;
    let trackersTable: SearchTable<Tracker>;
    let isDeleteTrackerOpen: boolean = false;
    let deleteTracker: Tracker;
    let isCreateTrackerOpen: boolean = false;
    let editJob: Edit;
    let editTracker: EditTracker;
    let isDeleteOpen = false;

    async function fetchJob() {
        let res = await fetch("/api/jobs/" + paramJob + "?tz=" + timezone);
        if (res.ok) {
            job = Object.assign(new Job(), await res.json());
            if (job?.activeTracker != null) {
                job.activeTracker = Object.assign(new Tracker(), job.activeTracker);
            }
        } else {
            alert(await res.text());
        }
    }

    async function fetchTrackers(): Promise<ListResult<Tracker>> {
        let res = await fetch("/api/jobs/" + paramJob + "/trackers");
        if (res.ok) {
            let trackers: ListResult<Tracker> = new ListResult();
            trackers.items = await res.json();
            trackers.count = trackers.items.length;
            for (let i = 0; i < trackers.count; i += 1) {
                trackers.items[i] = Object.assign(new Tracker(), trackers.items[i]);
            }
            return trackers;
        } else {
            alert(await res.text());
            return new ListResult();
        }
    }

    onMount(async () => {
        await Promise.all([fetchJob(), fetchTrackers()]);
    });

    let orderOptions: [string, string, string][] = [
        ["Date", "(New to Old)", "Date"],
        ["Date", "(Old to New)", "DateRev"],
        ["Name", "(A-Z)", "Name"],
        ["Name", "(Z-A)", "NameRev"]
    ];

    let fields: [string, (item: Tracker) => string, ((item: Tracker, mouseClick: MouseClick) => void) | null][] = [
        ["Name", (item) => item.display(), null],
        [
            "TimePensum",
            (item) => {
                if (item.timePensum) {
                    return formatDuration(item.timePensum * 60, true) + (item.timePensumUnit ? " / " + item.timePensumUnit.toString() : "");
                } else if (item.timePensumUnit != TimePensumUnit[TimePensumUnit.None]) {
                    return item.timePensumUnit.toString();
                } else {
                    return "None";
                }
            },
            null
        ],
        ["ValidFrom", (item) => (item.validFrom ? new Date(item.validFrom).toLocaleString(browserLanguage) : "-"), null],
        ["ValidUntil", (item) => (item.validUntil ? new Date(item.validUntil).toLocaleString(browserLanguage) : "-"), null],
        ["Created", (item) => (item.created ? new Date(item.created).toLocaleString(browserLanguage) : "-"), null]
    ];

    let onRowClick = (item: Tracker, click: MouseClick) => {
        click.goto("/jobs/" + job?.id + "/trackers/" + item.id);
    };

    let actions: [string, (item: Tracker, click: MouseClick) => void][] = [
        ["Details", onRowClick],
        [
            "Edit",
            (item, _click) => {
                editTracker.edit(item);
            }
        ],
        [
            "Delete",
            (item, _click) => {
                deleteTracker = item;
                isDeleteTrackerOpen = true;
            }
        ]
    ];

    function itemSearch(x: Tracker, s: string): boolean {
        return x.name?.toLowerCase().includes(s) || false;
    }

    function itemCompare(a: Tracker, b: Tracker, order: string): number {
        switch (order) {
            case "Date":
                return b.created.localeCompare(a.created);
            case "DateRev":
                return a.created.localeCompare(b.created);
            case "Name":
                if (a.name == null) return 1;
                if (b.name == null) return -1;
                return a.name.localeCompare(b.name);
            case "NameRev":
                if (b.name == null) return 1;
                if (a.name == null) return -1;
                return b.name.localeCompare(a.name);
        }
        return 0;
    }
</script>

{#if job != null}
    <SmallCard class="mb-8 mt-4 w-full">
        <div class="float-right flex flex-row gap-2">
            <button
                class="inline-flex rounded-md border border-gray-500 px-2 py-1 text-gray-500 outline-none focus-within:ring-4 focus-within:ring-gray-300 hover:bg-gray-200 dark:border-gray-400 dark:text-gray-400 dark:focus-within:ring-gray-600 dark:hover:bg-gray-700"
                on:click={() => {
                    if (job == null) return;
                    let trackers = trackersTable.$$.ctx[trackersTable.$$.props.originalList].items;
                    editJob.edit(job, trackers);
                }}
            >
                <EditSolid class="h-5 w-5 sm:me-2" /><span class="hidden sm:block">Edit</span>
            </button>
            <button
                class="inline-flex rounded-md border border-gray-500 px-2 py-1 text-gray-500 outline-none focus-within:ring-4 focus-within:ring-gray-300 hover:bg-gray-200 dark:border-gray-400 dark:text-gray-400 dark:focus-within:ring-gray-600 dark:hover:bg-gray-700"
                on:click={() => (isDeleteOpen = true)}
            >
                <TrashBinSolid class="h-5 w-5 sm:me-2" /><span class="hidden sm:block">Delete</span>
            </button>
        </div>
        <Heading tag="h1" class="text-2xl">
            {job.display()}
        </Heading>
        {#if job.description}
            {job.description}
        {/if}
        <hr class="mb-4 mt-4" />
        {#if job.name}
            {job.name} -
        {/if}
        Added on
        {new Date(job.created).toLocaleString(browserLanguage, {
            year: "numeric",
            month: "short",
            day: "numeric"
        })}
        at
        {new Date(job.created).toLocaleTimeString(browserLanguage)}
        <div class="float-right">
            {#if job.disabled}
                <Badge>Disabled</Badge>
            {/if}
        </div>
    </SmallCard>

    <div class="mb-8 mt-8 grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <SmallCard>
            Active tracker:
            <span class="float-right">
                {#if job.activeTracker}
                    <button
                        class="hover:underline"
                        on:click={(e) => new MouseClick(e).goto("/jobs/" + job?.id + "/trackers/" + job?.activeTracker?.id)}
                        >{job.activeTracker.display()}</button
                    >
                {:else}
                    None
                {/if}
            </span>
        </SmallCard>
        {#if job.activeTracker}
            <SmallCard>
                Time worked: <span class="float-right font-mono">{formatDuration(job.activeTracker.timeWorked)}</span>
            </SmallCard>
        {/if}
    </div>

    <SearchTable
        bind:this={trackersTable}
        getList={fetchTrackers}
        order="Date"
        {orderOptions}
        persistentOrder={[]}
        {fields}
        {actions}
        {onRowClick}
        newItem={() => new Tracker()}
        {itemSearch}
        {itemCompare}
    >
        <Heading tag="h2" class="-ml-0.25 mb-2 text-4xl font-semibold text-gray-900 dark:text-gray-50">
            Trackers <button
                class="cursor-pointer text-primary-600 hover:underline dark:text-primary-500"
                on:click={() => (isCreateTrackerOpen = true)}>+new</button
            >
        </Heading>
        <span class="text-base font-normal text-gray-500 dark:text-gray-400"> This is a list of all trackers for this job </span>
    </SearchTable>

    <Delete
        bind:isOpen={isDeleteTrackerOpen}
        bind:entity={deleteTracker}
        on:deleted={(e) => {
            trackersTable.manualUpdate((list) => {
                list.items = list.items.filter((item) => item.id != e.detail.id);
                list.count--;
                return list;
            });
        }}
    />

    <Create
        job={job.id}
        bind:isOpen={isCreateTrackerOpen}
        on:created={(e) => {
            trackersTable.manualUpdate((list) => {
                list.items.push(e.detail);
                list.count++;
                return list;
            });
        }}
    />

    <Edit bind:this={editJob} on:update={(e) => (job = e.detail)} />

    <Delete bind:isOpen={isDeleteOpen} entity={job} on:deleted={() => (window.location.href = "/jobs")} />

    <EditTracker
        bind:this={editTracker}
        on:update={(e) => {
            trackersTable.manualUpdate((list) => {
                let updatedTracker = e.detail;
                let index = list.items.findIndex((item) => item.id === updatedTracker.id);
                if (index !== -1) {
                    list.items[index] = updatedTracker;
                }
                return list;
            });
        }}
    />
{/if}
