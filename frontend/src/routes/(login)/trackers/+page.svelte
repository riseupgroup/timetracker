<script lang="ts">
    import { Heading } from "flowbite-svelte";
    import { Tracker, ListResult, MouseClick, TimePensumUnit, formatDuration, browserLanguage } from "../../../app";
    import SearchTable from "../../../components/SearchTable.svelte";
    import Delete from "../../../components/Delete.svelte";
    import Create from "../../../components/tracker/Create.svelte";
    import Edit from "../../../components/tracker/Edit.svelte";

    let trackersTable: SearchTable<Tracker>;
    let isDeleteOpen: boolean = false;
    let deleteTracker: Tracker;
    let isCreateOpen: boolean = false;
    let editTracker: Edit;

    async function fetchTrackers(): Promise<ListResult<Tracker>> {
        let res = await fetch("/api/trackers");
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

    let orderOptions: [string, string, string][] = [
        ["Date", "(New to Old)", "Date"],
        ["Date", "(Old to New)", "DateRev"],
        ["Name", "(A-Z)", "Name"],
        ["Name", "(Z-A)", "NameRev"],
    ];

    let fields: [string, (item: Tracker) => string, ((item: Tracker, mouseClick: MouseClick) => void) | null][] = [
        ["Name", (item) => item.name || "-", null],
        ["TimePensum", (item) => {
            if (item.timePensum) {
                return formatDuration(item.timePensum * 60, true) + (item.timePensumUnit?" / " + item.timePensumUnit.toString():"");
            } else if (item.timePensumUnit != TimePensumUnit[TimePensumUnit.None]) {
                return item.timePensumUnit.toString();
            } else {
                return "None";
            }
        }, null],
        ["ValidFrom", (item) => item.validFrom?new Date(item.validFrom).toLocaleString(browserLanguage):"-", null],
        ["ValidUntil", (item) => item.validUntil?new Date(item.validUntil).toLocaleString(browserLanguage):"-", null],
        ["Created", (item) => item.created?new Date(item.created).toLocaleString(browserLanguage):"-", null],
    ];

    let onRowClick = (item: Tracker, click: MouseClick) => {
        click.goto("/trackers/" + item.id);
    };

    let actions: [string, (item: Tracker, click: MouseClick) => void][] = [
        ["Details", onRowClick],
        ["Edit", (item, _click) => {
            editTracker.edit(item);
        }],
        ["Delete", (item, _click) => {
            deleteTracker = item;
            isDeleteOpen = true;
        }]
    ];

    function itemSearch(x: Tracker, s: string): boolean {
        return (
            x.name?.toLowerCase().includes(s) || false
        );
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
    <Heading
        tag="h1"
        class="-ml-0.25 mb-2 text-4xl font-semibold text-gray-900 dark:text-gray-50"
    >
        Trackers <button
            class="cursor-pointer text-primary-600 hover:underline dark:text-primary-500"
            on:click={() => (isCreateOpen = true)}>+new</button
        >
    </Heading>
    <span class="text-base font-normal text-gray-500 dark:text-gray-400">
        This is a list of all standallone trackers
    </span>
</SearchTable>

<Delete bind:isOpen={isDeleteOpen} bind:entity={deleteTracker} on:deleted={(e) => {
    trackersTable.manualUpdate((list) => {
        list.items = list.items.filter(item => item.id != e.detail.id);
        list.count--;
        return list;
    })
}} />

<Create
    bind:isOpen={isCreateOpen}
    on:created={(e) => {
        trackersTable.manualUpdate((list) => {
            list.items.push(e.detail);
            list.count++;;
            return list;
        });
    }}
/>

<Edit bind:this={editTracker} on:update={(e) => {
    trackersTable.manualUpdate((list) => {
        let updatedTracker = e.detail;
        let index = list.items.findIndex(item => item.id === updatedTracker.id);
        if (index !== -1) {
            list.items[index] = updatedTracker;
        }
        return list;
    });
}}/>
