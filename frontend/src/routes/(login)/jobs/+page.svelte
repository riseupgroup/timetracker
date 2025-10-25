<script lang="ts">
    import { Heading } from "flowbite-svelte";
    import { Job, ListResult, MouseClick, Tracker } from "../../../app";
    import SearchTable from "../../../components/SearchTable.svelte";
    import Delete from "../../../components/Delete.svelte";
    import Create from "../../../components/job/Create.svelte";
    import Edit from "../../../components/job/Edit.svelte";

    let jobsTable: SearchTable<Job>;
    let isDeleteOpen: boolean = false;
    let deleteJob: Job;
    let isCreateOpen: boolean = false;
    let editJob: Edit;

    async function fetchJobs(): Promise<ListResult<Job>> {
        let res = await fetch("/api/jobs");
        if (res.ok) {
            let jobs: ListResult<Job> = new ListResult();
            jobs.items = await res.json();
            jobs.count = jobs.items.length;
            for (let i = 0; i < jobs.count; i += 1) {
                jobs.items[i] = Object.assign(new Job(), jobs.items[i]);
                if (jobs.items[i].activeTracker != null) {
                    jobs.items[i].activeTracker = Object.assign(new Tracker(), jobs.items[i].activeTracker);
                }
            }
            return jobs;
        } else {
            alert(await res.text());
            return new ListResult();
        }
    }

    let orderOptions: [string, string, string][] = [
        ["Date", "(New to Old)", "Date"],
        ["Date", "(Old to New)", "DateRev"],
        ["Company name", "(A-Z)", "Company"],
        ["Company name", "(Z-A)", "CompanyRev"],
    ];

    let fields: [string, (item: Job) => string, ((item: Job, mouseClick: MouseClick) => void) | null][] = [
        ["Company", (item) => item.companyName || "-", null],
        ["Description", (item) => item.description || "-", null],
        ["Name", (item) => item.name || "-", null],
        ["Active Tracker", (item) => item.activeTracker?.display() || "-", (item, mouseClick) => {
            if (item.activeTracker != null) mouseClick.goto("/jobs/" + item.id + "/trackers/" + item.activeTracker.id);
        }]
    ];

    let onRowClick = (item: Job, click: MouseClick) => {
        click.goto("/jobs/" + item.id);
    };

    let actions: [string, (item: Job, click: MouseClick) => void][] = [
        ["Details", onRowClick],
        ["Edit", (item, _click) => {
            editJob.edit(item, null);
        }],
        ["Delete", (item, _click) => {
            deleteJob = item;
            isDeleteOpen = true;
        }]
    ];

    function itemSearch(x: Job, s: string): boolean {
        return (
            x.name?.toLowerCase().includes(s)
            || x.companyName?.toLocaleLowerCase().includes(s)
            || x.description?.toLocaleLowerCase().includes(s)
            || false
        );
    }

    function itemCompare(a: Job, b: Job, order: string): number {
        switch (order) {
            case "Date":
                return b.created.localeCompare(a.created);
            case "DateRev":
                return a.created.localeCompare(b.created);
            case "Company":
                if (a.companyName == null) return 1;
                if (b.companyName == null) return -1;
                return a.companyName.localeCompare(b.companyName);
            case "CompanyRev":
                if (b.companyName == null) return 1;
                if (a.companyName == null) return -1;
                return b.companyName.localeCompare(a.companyName);
        }
        return 0;
    }

</script>

<SearchTable
    bind:this={jobsTable}
    getList={fetchJobs}
    order="Date"
    {orderOptions}
    persistentOrder={[]}
    {fields}
    {actions}
    {onRowClick}
    newItem={() => new Job()}
    {itemSearch}
    {itemCompare}
>
    <Heading
        tag="h1"
        class="-ml-0.25 mb-2 text-4xl font-semibold text-gray-900 dark:text-gray-50"
    >
        Jobs <button
            class="cursor-pointer text-primary-600 hover:underline dark:text-primary-500"
            on:click={() => (isCreateOpen = true)}>+new</button
        >
    </Heading>
    <span class="text-base font-normal text-gray-500 dark:text-gray-400">
        This is a list of all jobs / collections
    </span>
</SearchTable>

<Delete bind:isOpen={isDeleteOpen} bind:entity={deleteJob} on:deleted={(e) => {
    jobsTable.manualUpdate((list) => {
        list.items = list.items.filter(item => item.id != e.detail.id);
        list.count--;
        return list;
    })
}} />

<Create
    bind:isOpen={isCreateOpen}
    on:created={(e) => {
        jobsTable.manualUpdate((list) => {
            list.items.push(e.detail);
            list.count++;;
            return list;
        });
    }}
/>

<Edit bind:this={editJob} on:update={(e) => {
    jobsTable.manualUpdate((list) => {
        let updatedJob = e.detail;
        let index = list.items.findIndex(item => item.id === updatedJob.id);
        if (index !== -1) {
            list.items[index] = updatedJob;
        }
        return list;
    })
}}/>
