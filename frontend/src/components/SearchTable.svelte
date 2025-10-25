<script lang="ts" generics="T extends Entity">
    import SearchTableCell from "./SearchTableCell.svelte";

    import { RowStyle, type Entity } from "../app";
    import {
        Button,
        Dropdown,
        Table,
        TableBody,
        TableBodyRow,
        TableHead,
        TableHeadCell,
        ButtonGroup,
        DropdownItem,
        Spinner
    } from "flowbite-svelte";
    import { ChevronDownOutline, BanOutline } from "flowbite-svelte-icons";
    import { getContext, onMount } from "svelte";
    import { ListResult, MouseClick } from "../app";
    import { page } from "$app/stores";
    import { type Writable } from "svelte/store";
    import Input from "./Input.svelte";

    export let apiPath: string | null = null;
    export let getList: (() => Promise<ListResult<T>>) | null = null;
    export let order: string;
    export let orderParam: string = "order";
    export let orderOptions: [string, string, string][];
    export let persistentOrder: string[] = [];
    export let search: string = "";
    export let searchParam: string = "search";
    export let pageSize: number = 100;

    export let fields: [
        string,
        (x: T) => string | Writable<string>,
        ((x: T, mouseClick: MouseClick) => void) | null
    ][];
    export let actions: [string, (x: T, mouseClick: MouseClick) => void][] | null = null;
    export let onRowClick: ((x: T, mouseClick: MouseClick) => void) | null = null;

    export let newItem: () => T;
    export let itemSearch: ((x: T, s: string) => boolean) | null = null;
    export let itemCompare: ((a: T, b: T, order: string) => number) | null = null;
    export let list: ListResult<T> | null = null;
    export let originalList: ListResult<T> | null = null;
    let loading = false;

    async function queryList(
        order: string,
        search: string,
        append: boolean = false,
        start: string | number | null = null,
        force: boolean = false
    ) {
        let params = new URLSearchParams();
        let pageParams = new URLSearchParams();
        params.append("order", order);
        pageParams.append(orderParam, order);
        if (search != "") {
            params.append("search", search);
            pageParams.append(searchParam, search);
        }

        if (!append) {
            window.history.replaceState(
                {},
                $page.data.title,
                window.location.origin + window.location.pathname + "?" + pageParams.toString()
            );
        }

        if (apiPath != null) {
            if (start != null) {
                params.append("start", String(start));
            }
            params.append("limit", String(pageSize));

            let res = await fetch(apiPath + "?" + params.toString());
            if (res.ok) {
                let resList: ListResult<T> = await res.json();
                for (let i = 0; i < resList.items.length; i += 1) {
                    resList.items[i] = Object.assign(newItem(), resList.items[i]);
                }
                if (append && list != null) {
                    if (loading) {
                        list.count = resList.count;
                        Array.prototype.push.apply(list.items, resList.items);
                        list.items = list.items;
                    }
                } else {
                    list = resList;
                }
                loading = false;
            } else {
                alert(await res.text());
            }
        } else if (getList != null) {
            if (originalList == null || force) originalList = await getList();
            let tmp: ListResult<T> = new ListResult();
            if (search != "") {
                search = search.toLowerCase();
                if (itemSearch != null) {
                    for (let item of originalList.items) {
                        if (itemSearch(item, search)) {
                            tmp.items.push(item);
                        }
                    }
                } else {
                    alert("Missing custom itemSearch function for SearchTable");
                }
            } else {
                tmp.items = originalList.items;
            }
            if (itemCompare != null) {
                tmp.items.sort((a, b) => itemCompare(a, b, order));
            } else {
                alert("Missing custom itemCompare function for SearchTable");
            }
            tmp.count = tmp.items.length;
            list = tmp;
        } else {
            alert("No data source for SearchTable");
        }
    }

    export function refreshList(force: boolean = false) {
        loading = false;
        queryList(order, search, false, null, force);
    }

    export function manualUpdate(fn: (oldList: ListResult<T>) => ListResult<T>) {
        originalList = fn(originalList??new ListResult<T>());
        refreshList(false);
    }

    onMount(() => {
        let newSort = $page.url.searchParams.get(orderParam);
        if (newSort != null) {
            order = newSort;
        }
        let newSearch = $page.url.searchParams.get(searchParam);
        if (newSearch != null) {
            search = newSearch;
        }
        queryList(order, search);
    });

    let orderDropdown: Dropdown;
    function setOrder(newOrder: string) {
        order = newOrder;
        orderDropdown.$$set({ open: false });
        refreshList();
    }

    function getClass(item: T): string {
        let style = item.rowStyle();
        if (style == null) {
            return "text-gray-900 dark:text-white";
        } else if (typeof style == "string") {
            return style;
        } else {
            switch (style as RowStyle) {
                case RowStyle.Disabled:
                    return "text-gray-500 dark:text-gray-400";
                case RowStyle.Underlined:
                    return "text-gray-900 dark:text-white underline";
                case RowStyle.Bold:
                    return "text-gray-900 dark:text-white font-bold";
            }
        }
    }

    if (apiPath != null) {
        let content: Writable<HTMLDivElement | null> = getContext("content");
        content.subscribe((content) => {
            content?.addEventListener("scroll", () => {
                if (list != null && persistentOrder.includes(order)) {
                    if (content.offsetHeight + content.scrollTop >= content.scrollHeight - 200) {
                        if (list.count > list.items.length && !loading) {
                            loading = true;
                            let start = list.items[list.items.length - 1].primary();
                            queryList(order, search, true, start);
                        }
                    }
                }
            });
        });
    }
</script>

<div class="items-center justify-between lg:flex">
    <div class="mb-4 mt-px lg:mb-0">
        <slot />
    </div>
    <div class="items-center justify-between gap-3 space-y-4 sm:flex sm:space-y-0">
        <div class="flex items-center">
            <Button id="order-button" color="alternative" class="w-fit whitespace-nowrap px-4 py-2">
                Sort by: {(orderOptions.find(([_x, _y, value]) => value == order) || [
                    "Unknown"
                ])[0]}
                <ChevronDownOutline size="lg" />
            </Button>
            <Dropdown
                class="overflow-hidden rounded-lg bg-gray-50 dark:bg-gray-800"
                placement="bottom-end"
                bind:this={orderDropdown}
            >
                {#each orderOptions as [name, aditional, value]}
                    <DropdownItem on:click={() => setOrder(value)}>{name} {aditional}</DropdownItem>
                {/each}
            </Dropdown>
        </div>
        <div class="flex items-center space-x-4">
            <ButtonGroup class="w-full">
                <Input
                    name="Search"
                    classInput="rounded-r-none"
                    bind:value={search}
                    on:change={() => refreshList()}
                    on:changeExtended={() => refreshList()}
                />
                <Button color="primary" on:click={() => refreshList()}>Search</Button>
            </ButtonGroup>
        </div>
    </div>
</div>

{#if list != null && list.items.length > 0}
    <Table
        hoverable={true}
        noborder
        striped
        class="mt-6 w-full divide-y divide-gray-200 dark:divide-gray-600"
    >
        <TableHead class="bg-gray-50 dark:bg-gray-700">
            {#each fields as [header, _]}
                <TableHeadCell class="whitespace-nowrap p-4 font-normal">{header}</TableHeadCell>
            {/each}
            {#if actions != null}
                <TableHeadCell class="whitespace-nowrap p-4 font-normal">Actions</TableHeadCell>
            {/if}
        </TableHead>
        <TableBody>
            {#each list.items as item}
                <TableBodyRow
                    class="{getClass(item)} {onRowClick!=null?"cursor-pointer":""}"
                    on:click={(e) => onRowClick?.call(null, item, new MouseClick(e))}
                >
                    {#each fields as [_, field, onClick]}
                        <SearchTableCell content={field(item)} clickable={onClick!=null} on:click={(e) => onClick?.call(null, item, e.detail)}/>
                    {/each}
                    {#if actions != null}
                        <td class="whitespace-nowrap p-4 font-normal">
                            {#each actions as [title, action]}
                                <button
                                    class="display-inline pr-2 text-primary-600 hover:underline dark:text-primary-500"
                                    on:mousedown={e => {
                                        e.stopPropagation();
                                        action(item, new MouseClick(e));
                                    }}
                                    on:click={e => e.stopPropagation()}
                                >
                                    {title}
                                </button>
                            {/each}
                        </td>
                    {/if}
                </TableBodyRow>
            {/each}
        </TableBody>
    </Table>
{:else if list != null && list.items.length == 0}
    <div class="mt-8 flex flex-col items-center">
        <BanOutline class="h-20 w-20" />
        <div>There are no items to display</div>
    </div>
{:else}
    <div class="mt-8 flex flex-col items-center">
        <Spinner class="h-16 w-16" />
        <div class="mt-2">Loading...</div>
    </div>
{/if}
