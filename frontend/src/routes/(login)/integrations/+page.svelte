<script lang="ts">
    import SearchTable from "../../../components/SearchTable.svelte";
    import { ApiKey, ListResult, MouseClick } from "../../../app";
    import Delete from "../../../components/Delete.svelte";
    import { Heading } from "flowbite-svelte";
    import Create from "../../../components/apikey/Create.svelte";
    import Edit from "../../../components/apikey/Edit.svelte";

    let keysTable: SearchTable<ApiKey>;
    let list: ListResult<ApiKey> = new ListResult();

    let orderOptions: [string, string, string][] = [
        ["Date", "(New to Old)", "Date"],
        ["Date", "(Old to New)", "DateRev"],
        ["Name", "(A to Z)", "Name"],
        ["Name", "(Z to A)", "NameRev"],
        ["Valid until", "(Expires first)", "Valid"],
        ["Valid until", "(Expires last)", "ValidRev"],
        ["Last used", "(New to Old)", "LastUsed"],
        ["Last used", "(Old to New)", "LastUsedRev"]
    ];

    let fields: [string, (k: ApiKey) => string, ((k: ApiKey, mouseClick: MouseClick) => void) | null][] = [
        ["Name", (k) => k.display(), null],
        ["Valid until", (k) => (k.validUntil ? new Date(k.validUntil).toLocaleDateString() : "never expires"), null],
        [
            "Last used",
            (k) => {
                if (k.lastUsed == null) return "never used";
                let date = new Date(k.lastUsed);
                let diff = new Date().getTime() - date.getTime();

                let days = Math.floor(diff / (1000 * 60 * 60 * 24));
                let hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
                let minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));

                if (days >= 5 * 7) {
                    return date.toLocaleDateString();
                } else if (days > 7) {
                    return Math.floor(days / 7) + " weeks ago";
                } else if (days > 1) {
                    return days + " days ago";
                } else if (hours > 1) {
                    return hours + " hours ago";
                } else if (minutes > 1) {
                    return minutes + " minutes ago";
                } else if (minutes == 1) {
                    return "a minute ago";
                } else {
                    return "just now";
                }
            },
            null
        ],
        ["Last changed", (k) => new Date(k.lastChanged).toLocaleDateString(), null],
        ["Added", (k) => new Date(k.added).toLocaleDateString(), null]
    ];

    async function fetchKeys(): Promise<ListResult<ApiKey>> {
        let res = await fetch("/api/keys");
        if (res.ok) {
            let keys: ListResult<ApiKey> = new ListResult();
            keys.items = await res.json();
            keys.count = keys.items.length;
            for (let i = 0; i < keys.count; i += 1) {
                keys.items[i] = Object.assign(new ApiKey(), keys.items[i]);
            }
            return keys;
        } else {
            alert(await res.text());
            return new ListResult();
        }
    }

    let deleteKey: ApiKey;
    let isDeleteOpen: boolean = false;
    let isCreateOpen: boolean = false;
    let editKey: Edit;
    let actions: [string, (k: ApiKey, c: MouseClick) => void][] = [
        ["Edit", (k, _c) => edit(k)],
        [
            "Delete",
            (k, _c) => {
                deleteKey = k;
                isDeleteOpen = true;
            }
        ]
    ];

    function edit(key: ApiKey) {
        editKey.edit(key);
    }

    function itemSearch(x: ApiKey, s: string): boolean {
        return x.name?.toLowerCase().includes(s) || x.display().toLowerCase().includes(s);
    }

    function itemCompare(a: ApiKey, b: ApiKey, order: string): number {
        switch (order) {
            case "Date":
                if (b.lastChanged == null) return 1;
                if (a.lastChanged == null) return -1;
                return b.lastChanged.localeCompare(a.lastChanged);
            case "DateRev":
                if (a.lastChanged == null) return 1;
                if (b.lastChanged == null) return -1;
                return a.lastChanged.localeCompare(b.lastChanged);
            case "Name":
                if (a.name == null) return 1;
                if (b.name == null) return -1;
                return a.name.localeCompare(b.name);
            case "NameRev":
                if (b.name == null) return 1;
                if (a.name == null) return -1;
                return b.name.localeCompare(a.name);
            case "Valid":
                if (a.validUntil == null) return 1;
                if (b.validUntil == null) return -1;
                return a.validUntil.localeCompare(b.validUntil);
            case "ValidRev":
                if (b.validUntil == null) return 1;
                if (a.validUntil == null) return -1;
                return b.validUntil.localeCompare(a.validUntil);
            case "LastUsed":
                if (b.lastUsed == null) return 1;
                if (a.lastUsed == null) return -1;
                return b.lastUsed.localeCompare(a.lastUsed);
            case "LastUsedRev":
                if (a.lastUsed == null) return 1;
                if (b.lastUsed == null) return -1;
                return a.lastUsed.localeCompare(b.lastUsed);
        }
        return 0;
    }
</script>

<SearchTable
    bind:this={keysTable}
    bind:list
    getList={fetchKeys}
    order="Date"
    {orderOptions}
    persistentOrder={[]}
    {fields}
    {actions}
    newItem={() => new ApiKey()}
    {itemSearch}
    {itemCompare}
>
    <Heading tag="h1" class="-ml-0.25 mb-2 text-4xl font-semibold text-gray-900 dark:text-gray-50"
        >Api Keys <button
            class="cursor-pointer text-primary-600 hover:underline dark:text-primary-500"
            on:click={() => (isCreateOpen = true)}>+new</button
        ></Heading
    >
    <span class="text-base font-normal text-gray-500 dark:text-gray-400">This is a list of all your api keys</span>
</SearchTable>

<Delete
    bind:isOpen={isDeleteOpen}
    bind:entity={deleteKey}
    on:deleted={(e) => {
        if (list == null) return;
        for (let i = 0; i < list.items.length; i++) {
            if (list.items[i].id == e.detail.id) {
                list.items.splice(i, 1);
                list.count -= 1;
                list = list;
                break;
            }
        }
    }}
/>

<Create bind:isOpen={isCreateOpen} on:created={() => keysTable.refreshList(true)} />
<Edit bind:this={editKey} on:update={() => keysTable.refreshList(true)} />
