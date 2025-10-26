<script lang="ts">
    import { onMount } from "svelte";
    import {
        browserLanguage,
        DisplayRangeUnit,
        formatDuration,
        getWeekNumber,
        getYearFromWeek,
        MouseClick,
        TimePensumUnit,
        Timeslot,
        timezone,
        Tracker,
        type Unit
    } from "../../app";
    import { A, Heading, Label, Select, Spinner, Table, TableBody, TableBodyRow, TableHead, TableHeadCell, Tooltip } from "flowbite-svelte";
    import { EditSolid, TrashBinSolid, FolderPlusSolid, BanOutline } from "flowbite-svelte-icons";
    import Delete from "../../components/Delete.svelte";
    import SmallCard from "../../components/SmallCard.svelte";
    import Edit from "./Edit.svelte";
    import EditTimeslot from "../timeslot/Edit.svelte";
    import Create from "../timeslot/Create.svelte";
    import { writable, type Writable } from "svelte/store";
    import SearchTableCell from "../SearchTableCell.svelte";
    import { page } from "$app/stores";
    import TimeWorked from "./TimeWorked.svelte";

    let paramJob: string | null = null;
    let paramTracker: string;
    export { paramJob as job, paramTracker as tracker };

    function getRoute() {
        let r = "/api";
        if (paramJob != null) r += "/jobs/" + paramJob;
        r += "/trackers/" + paramTracker;
        return r;
    }

    let timeslots: Unit[] | null = null;
    let timeslotIntervals: number[] = [];
    let lastFetchedTimeslots: {
        unit: string | null;
        primary: number | null;
        secondary: number | null;
    } = {
        unit: null,
        primary: null,
        secondary: null
    };
    let timeslotsUnit: Writable<string | null> = writable(null);
    let timeslotsPrimary: Writable<number | null> = writable(null);
    let timeslotsPrimaryOptions: { value: number; name: number }[] = [];
    let timeslotsSecondary: Writable<number | null> = writable(null);
    let timeslotsSecondaryOptions: { value: number; name: string }[] = [];

    let fetched: boolean = false;
    let tracker: Tracker | null = null;
    let totalTimeWorked: number | null = null;
    let totalTimeWorkedCooldown: number = 0;
    let totalTimeWorkedInterval: number | null = null;
    let editTracker: Edit;
    let isDeleteOpen: boolean = false;

    function setTimeslotsRangeOptions(updateSecondary: boolean) {
        function getSecondary(date: Date, unit: string): number {
            switch (unit) {
                case DisplayRangeUnit[DisplayRangeUnit.Month]:
                    return date.getMonth() + 1;
                case DisplayRangeUnit[DisplayRangeUnit.Week]:
                    return getWeekNumber(date);
                default:
                    return 0;
            }
        }
        if (tracker) {
            let now = new Date();

            let start: Date;
            if (tracker.validFrom) {
                start = new Date(tracker.validFrom);
            } else {
                start = new Date(now.getTime());
                start.setFullYear(start.getFullYear() - 20);
            }

            let end: Date;
            if (tracker.validUntil) {
                end = new Date(tracker.validUntil);
            } else {
                end = new Date(now.getTime());
                end.setFullYear(end.getFullYear() + 1);
            }

            timeslotsPrimaryOptions = [];
            for (let i = start.getFullYear(); i <= end.getFullYear(); i += 1) {
                timeslotsPrimaryOptions.push({ value: i, name: i });
            }

            let unit = $timeslotsUnit || tracker.displayRangeUnit;
            let currentPrimary = $timeslotsPrimary || now.getFullYear();
            let currentSecondary = $timeslotsSecondary;

            let secondaryStart = 1;
            if (currentPrimary == start.getFullYear()) {
                secondaryStart = getSecondary(start, unit);
            }

            if (currentPrimary != end.getFullYear()) {
                end = new Date(currentPrimary, 11, 31);
            }
            let secondaryEnd = getSecondary(end, unit);
            if (secondaryEnd == 1 && end.getDate() > 14) {
                end.setDate(end.getDate() - 7);
                secondaryEnd = getSecondary(end, unit);
            }

            if (currentSecondary != null && updateSecondary) {
                if (currentSecondary < secondaryStart) {
                    timeslotsSecondary.set(secondaryStart);
                } else if (currentSecondary > secondaryEnd) {
                    timeslotsSecondary.set(secondaryEnd);
                }
            } else if (currentSecondary == null && updateSecondary) {
                if (currentPrimary == now.getFullYear()) {
                    if (unit == DisplayRangeUnit[DisplayRangeUnit.Month]) {
                        timeslotsSecondary.set(now.getMonth() + 1);
                    } else if (unit == DisplayRangeUnit[DisplayRangeUnit.Week]) {
                        timeslotsPrimary.set(getYearFromWeek(now));
                        timeslotsSecondary.set(getWeekNumber(now));
                    }
                } else {
                    timeslotsSecondary.set(secondaryStart);
                }
            }

            timeslotsSecondaryOptions = [];
            const date = new Date();
            for (let i = secondaryStart; i <= secondaryEnd; i += 1) {
                let name = String(i);
                if (unit == DisplayRangeUnit[DisplayRangeUnit.Month]) {
                    date.setMonth(i - 1);
                    name += " - ";
                    name += date.toLocaleString(browserLanguage, { month: "long" });
                }
                timeslotsSecondaryOptions.push({ value: i, name: name });
            }
        }
    }

    async function updateUnit(unit: string, force: boolean = false) {
        if (fetched || force) {
            if (unit != DisplayRangeUnit[DisplayRangeUnit.Year]) {
                setTimeslotsRangeOptions(true);
            }
            setTimeout(() => fetchTimeslots());
        }
    }
    timeslotsPrimary.subscribe(() => setTimeslotsRangeOptions(true));
    timeslotsPrimary.subscribe(() => fetchTimeslots());
    timeslotsSecondary.subscribe(() => fetchTimeslots());
    timeslotsUnit.subscribe((unit) => {
        if (unit) {
            updateUnit(unit);
        }
    });

    async function fetchTracker() {
        let res = await fetch(getRoute() + "?tz=" + timezone);
        if (res.ok) {
            displayTracker(Object.assign(new Tracker(), await res.json()));
        } else {
            alert(await res.text());
        }
    }

    function displayTracker(newTracker: Tracker) {
        tracker = newTracker;
        let unit = $page.url.searchParams.get("range_unit");
        if (!unit) {
            unit = newTracker.displayRangeUnit;
        }

        let now = new Date();
        let primary = Number($page.url.searchParams.get("range_primary"));
        let secondary = Number($page.url.searchParams.get("range_secondary"));

        if (!secondary) {
            if (unit == DisplayRangeUnit[DisplayRangeUnit.Month]) {
                secondary = now.getMonth() + 1;
                primary = now.getFullYear();
            } else if (unit == DisplayRangeUnit[DisplayRangeUnit.Week]) {
                secondary = getWeekNumber(now);
                primary = getYearFromWeek(now);
            }
        }
        if (!primary) {
            primary = now.getFullYear();
        }

        lastFetchedTimeslots.unit = unit;
        timeslotsUnit.set(unit);
        lastFetchedTimeslots.primary = primary;
        timeslotsPrimary.set(primary);
        lastFetchedTimeslots.secondary = secondary;
        timeslotsSecondary.set(secondary);
        updateUnit(unit, true);
        fetchTimeslots(true, unit);
        fetched = true;
    }

    async function fetchTimeslots(force: boolean = false, unit: string | null = null) {
        if (!unit) unit = $timeslotsUnit;
        let primary = $timeslotsPrimary;
        let secondary = $timeslotsSecondary;
        if (
            !force &&
            lastFetchedTimeslots.unit == unit &&
            lastFetchedTimeslots.primary == primary &&
            (lastFetchedTimeslots.secondary == secondary || unit == TimePensumUnit[TimePensumUnit.Year])
        ) {
            return;
        }
        if (fetched || force) {
            lastFetchedTimeslots = {
                unit: unit,
                primary: primary,
                secondary: secondary
            };
            timeslots = null;
            for (let interval of timeslotIntervals) {
                clearInterval(interval);
            }
            timeslotIntervals = [];

            let url = getRoute() + "/units?tz=" + timezone;
            if (unit) {
                let params = new URLSearchParams(window.location.search);
                url += "&range[unit]=" + unit;
                params.set("range_unit", unit);
                if (primary) {
                    url += "&range[value][year]=" + primary;
                    params.set("range_primary", String(primary));
                }
                if (secondary && unit != TimePensumUnit[TimePensumUnit.Year]) {
                    url += "&range[value][" + unit.toLowerCase() + "]=" + secondary;
                    params.set("range_secondary", String(secondary));
                }
                if (!force) {
                    window.history.replaceState(
                        {},
                        $page.data.title,
                        window.location.origin + window.location.pathname + "?" + params.toString()
                    );
                }
            }
            let res = await fetch(url);
            if (res.ok) {
                let slots: Unit[] = await res.json();
                for (let unit of slots) {
                    unit.timeslots = unit.timeslots.map((timeslot) => Object.assign(new Timeslot(), timeslot));
                }
                timeslots = slots;
            } else {
                alert(await res.text());
            }
        }
    }

    async function getTotalTimeWorked() {
        if (totalTimeWorkedCooldown > 0) return;
        let res = await fetch(getRoute() + "/time-worked?tz=" + timezone);
        if (res.ok) {
            totalTimeWorked = await res.json();
            totalTimeWorkedCooldown = 9;
            if (totalTimeWorkedInterval != null) clearInterval(totalTimeWorkedInterval);
            totalTimeWorkedInterval = setInterval(() => {
                totalTimeWorkedCooldown -= 1;
                if (totalTimeWorkedCooldown <= 0) {
                    if (totalTimeWorkedInterval) clearInterval(totalTimeWorkedInterval);
                    totalTimeWorkedInterval = null;
                }
            }, 1000);
        } else {
            alert(await res.text());
        }
    }

    onMount(fetchTracker);

    let fields: [string, (t: Timeslot) => string | Writable<string>, ((t: Timeslot, mouseClick: MouseClick) => void) | null, boolean][] = [
        [
            "Date",
            (t) => {
                let start = new Date(t.start);
                let end = t.end ? new Date(t.end) : null;

                function toString(date: Date): string {
                    switch (tracker?.timePensumUnit) {
                        case TimePensumUnit[TimePensumUnit.Week]:
                            return date.toLocaleString(browserLanguage, { weekday: "short" });
                        case TimePensumUnit[TimePensumUnit.Month]:
                            return String(date.getDate());
                        case TimePensumUnit[TimePensumUnit.Year]:
                            return date.toLocaleString(browserLanguage, {
                                month: "short",
                                day: "numeric"
                            });
                        default:
                            return date.toLocaleString(browserLanguage, {
                                year: "numeric",
                                month: "short",
                                day: "numeric"
                            });
                    }
                }
                if (
                    end != null &&
                    (start.getFullYear() != end.getFullYear() || start.getMonth() != end.getMonth() || start.getDate() != end.getDate())
                ) {
                    if (tracker?.timePensumUnit == TimePensumUnit[TimePensumUnit.Month]) return toString(start) + "-" + toString(end);
                    else return toString(start) + " - " + toString(end);
                } else {
                    return toString(start);
                }
            },
            null,
            false
        ],
        ["Start", (t) => new Date(t.start).toLocaleTimeString(browserLanguage), null, false],
        ["End", (t) => (t.end ? new Date(t.end).toLocaleTimeString(browserLanguage) : "active"), null, false],
        [
            "Duration",
            (t) => {
                function getEnd(end: Date): string {
                    let duration = end.getTime() - new Date(t.start).getTime();
                    return formatDuration(Math.floor(duration / 1000));
                }
                if (t.end == null) {
                    let ret = writable(getEnd(new Date()));
                    let startRefresh = () => timeslotIntervals.push(setInterval(() => ret.set(getEnd(new Date())), 1000));
                    let delayUntilCounting = new Date(t.start).getTime() - new Date().getTime();
                    if (delayUntilCounting > 0) {
                        // This works since setTimeout() and setInterval() ids are shared
                        // https://developer.mozilla.org/docs/Web/API/Window/clearTimeout
                        timeslotIntervals.push(setTimeout(startRefresh, delayUntilCounting));
                    } else {
                        startRefresh();
                    }
                    return ret;
                } else {
                    return getEnd(new Date(t.end));
                }
            },
            null,
            true
        ],
        [
            "Comment",
            (t) => {
                if (t.comment == null) return "-";
                let comment = t.comment.trim();
                comment = comment.replace(/\n/g, " ").trim();
                if (comment.length > 50) {
                    return comment.substring(0, 47).trim() + "...";
                }
                return comment;
            },
            (t) => {
                editTimeslot.edit(t);
            },
            false
        ]
    ];

    let deleteTimeslot: Timeslot;
    let isDeleteTimeslotOpen: boolean = false;
    let editTimeslot: EditTimeslot;
    let isCreateOpen: boolean = false;
    let actions: [string, (t: Timeslot, c: MouseClick) => void][] = [
        [
            "Edit",
            (t, _) => {
                editTimeslot.edit(t);
            }
        ],
        [
            "Delete",
            (t, _) => {
                isDeleteTimeslotOpen = true;
                deleteTimeslot = t;
            }
        ]
    ];

    function openEdit() {
        if (tracker) editTracker.edit(tracker);
    }

    function getEndUnit(unit: Unit, unitIdx: number): Date | null {
        if (timeslots && unit.timeslots.length == 0) {
            for (let i = unitIdx + 1; i < timeslots.length; i += 1) {
                if (timeslots[i].timeslots.length != 0) {
                    if (i - 1 > unitIdx) return new Date(timeslots[i - 1].start);
                    return null;
                }
            }
            if (timeslots.length - 1 > unitIdx) return new Date(timeslots[timeslots.length - 1].start);
        }
        return null;
    }

    function getUnitTitle(unit: Unit, unitIdx: number, titleUnit: string | null): string {
        if (tracker && timeslots) {
            let start = new Date(unit.start);
            let end = getEndUnit(unit, unitIdx);
            if (!titleUnit) titleUnit = TimePensumUnit[TimePensumUnit.None];
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            let trackerUnit = Number(TimePensumUnit[tracker.timePensumUnit as any]);
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            let displayUnit = Number(TimePensumUnit[titleUnit as any]);
            titleUnit = TimePensumUnit[Math.min(trackerUnit, displayUnit)];

            function format(week: boolean, func: (x: Date) => string) {
                let out = "";
                let endYear = null;
                if (week) {
                    let startYear = getYearFromWeek(start);
                    let endYearTmp = end ? getYearFromWeek(end) : null;
                    if (startYear != $timeslotsPrimary || (endYearTmp != null && startYear != endYearTmp)) {
                        out += String(startYear) + " ";
                        endYear = endYearTmp;
                    }
                    out += "Week ";
                }
                out += func(start);
                if (end) {
                    out += " - ";
                    if (endYear) {
                        out += String(endYear) + " ";
                        if (week) {
                            out += "Week ";
                        }
                    }
                    out += func(end);
                }
                return out;
            }

            switch (titleUnit) {
                case TimePensumUnit[TimePensumUnit.Week]:
                    return format(true, (x) => String(getWeekNumber(x)));
                case TimePensumUnit[TimePensumUnit.Month]:
                    return format(false, (x) => x.toLocaleString(browserLanguage, { year: "numeric", month: "long" }));
                case TimePensumUnit[TimePensumUnit.Year]:
                    return format(false, (x) => String(x.getFullYear()));
                default:
                    return "Timeslots";
            }
        } else {
            return "Timeslots";
        }
    }
    function getWeekDates(unit: Unit, unitIdx: number): string {
        let date = new Date(unit.start);
        let end = new Date(getEndUnit(unit, unitIdx) || date);
        end.setDate(end.getDate() + 7);
        function displayDate(date: Date): string {
            return date.toLocaleString(browserLanguage, {
                year: "numeric",
                month: "numeric",
                day: "numeric"
            });
        }
        return displayDate(date) + " - " + displayDate(end);
    }
</script>

{#if tracker != null}
    <SmallCard class="mt-4 w-full">
        <div class="float-right flex flex-row gap-2">
            <button
                class="inline-flex rounded-md border border-gray-500 px-2 py-1 text-gray-500 outline-none focus-within:ring-4 focus-within:ring-gray-300 hover:bg-gray-200 dark:border-gray-400 dark:text-gray-400 dark:focus-within:ring-gray-600 dark:hover:bg-gray-700"
                on:click={() => (isCreateOpen = true)}
            >
                <FolderPlusSolid class="h-5 w-5 sm:me-2" /><span class="hidden sm:block">Timeslot</span>
            </button>
            <button
                class="inline-flex rounded-md border border-gray-500 px-2 py-1 text-gray-500 outline-none focus-within:ring-4 focus-within:ring-gray-300 hover:bg-gray-200 dark:border-gray-400 dark:text-gray-400 dark:focus-within:ring-gray-600 dark:hover:bg-gray-700"
                on:click={openEdit}
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
            {tracker.display()}
        </Heading>
        {#if tracker.job != null}
            <button
                class="cursor-pointer hover:underline"
                on:click={(mouseEvent) => {
                    mouseEvent.preventDefault();
                    if (tracker && tracker.job) {
                        new MouseClick(mouseEvent).goto("/jobs/" + tracker.job);
                    }
                }}
            >
                Go back to job
            </button>
        {/if}
        <hr class="mb-4 mt-4" />
        Added on {new Date(tracker.created).toLocaleString(browserLanguage, {
            year: "numeric",
            month: "short",
            day: "numeric"
        })} at {new Date(tracker.created).toLocaleTimeString(browserLanguage)}
    </SmallCard>
    <div class="mb-8 mt-8 grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <SmallCard>
            {#if tracker.timePensum}
                Time pensum: <span class="float-right">
                    {formatDuration(tracker.timePensum * 60, true)}
                    {tracker.timePensumUnit ? " / " + tracker.timePensumUnit : ""}
                </span>
            {:else if tracker.timePensumUnit != TimePensumUnit[TimePensumUnit.None]}
                Time pensum Unit: <span class="float-right">
                    {tracker.timePensumUnit}
                </span>
            {:else}
                Time pensum: <span class="float-right">None</span>
            {/if}
        </SmallCard>
        <SmallCard>
            Time worked: <span class="float-right font-mono" id="time-worked">
                {#if timeslots}
                    <TimeWorked
                        {timeslots}
                        filterUnit={$timeslotsUnit}
                        filterPrimary={$timeslotsPrimary}
                        filterSecondary={$timeslotsSecondary}
                    />
                {/if}
            </span>
        </SmallCard>
        <SmallCard>
            Total time worked:
            <button class="float-right {totalTimeWorkedCooldown > 0 ? 'cursor-progress' : 'hover:underline'}" on:click={getTotalTimeWorked}>
                {#if totalTimeWorked}
                    {#if totalTimeWorkedCooldown > 0}
                        <span class="font-mono">
                            {"(" + totalTimeWorkedCooldown + "s) "}
                        </span>
                    {/if}
                    {formatDuration(totalTimeWorked)}
                {:else}
                    (calculate)
                {/if}
            </button>
        </SmallCard>
    </div>

    <div class="flex flex-row gap-2">
        {#each Object.keys(DisplayRangeUnit).filter((key) => isNaN(Number(key))) as displayRangeUnit}
            <A
                asButton
                color={$timeslotsUnit == displayRangeUnit
                    ? "text-primary-600 dark:text-primary-500"
                    : "text-secondary-600 dark:text-secondary-500"}
                on:click={() => timeslotsUnit.set(displayRangeUnit)}>{displayRangeUnit}</A
            >
        {/each}
    </div>
    <div class="flex flex-row flex-wrap justify-items-stretch gap-2 md:justify-items-start">
        <div class="grow md:grow-0">
            <Label class={$timeslotsUnit != DisplayRangeUnit[DisplayRangeUnit.Year] ? "mt-2" : "hidden"}>Year:</Label>
            <Select
                class={$timeslotsUnit != DisplayRangeUnit[DisplayRangeUnit.Year] ? "" : "mt-2"}
                items={timeslotsPrimaryOptions}
                bind:value={$timeslotsPrimary}
            />
        </div>
        <Label class={$timeslotsUnit != DisplayRangeUnit[DisplayRangeUnit.Year] ? "mt-2 grow md:grow-0" : "hidden"}>
            {#if $timeslotsUnit == "Month"}
                Month:
            {:else if $timeslotsUnit == "Week"}
                Week:
            {/if}
            <Select items={timeslotsSecondaryOptions} bind:value={$timeslotsSecondary} />
        </Label>
    </div>

    {#if timeslots != null}
        <Table hoverable noborder striped class="w-full">
            {#each timeslots as unit, i}
                {@const shouldShow =
                    i == 0 || unit.timeslots.length != 0 || (unit.timeslots.length == 0 && timeslots[i - 1].timeslots.length != 0)}
                {#if shouldShow}
                    <thead>
                        <tr>
                            <th colspan={fields.length + 1}>
                                <Heading tag="h2" class="mb-4 mt-6 inline-block w-auto text-xl"
                                    >{getUnitTitle(unit, i, $timeslotsUnit)}
                                    {#if unit.timeslots.length > 0}
                                        <span class="text-gray-500 dark:text-gray-400">
                                            <span class="mx-1">|</span>
                                            <TimeWorked timeslots={unit.timeslots} />
                                        </span>
                                    {/if}
                                </Heading>
                                {#if tracker.timePensumUnit == TimePensumUnit[TimePensumUnit.Week]}
                                    <Tooltip placement="right" arrow={false}>{getWeekDates(unit, i)}</Tooltip>
                                {/if}
                            </th>
                        </tr>
                    </thead>
                    {#if unit.timeslots.length > 0}
                        <TableHead class="bg-gray-50 dark:bg-gray-700">
                            {#each fields as [header, _field, _onClick, _mono]}
                                <TableHeadCell class="whitespace-nowrap p-4 font-normal">{header}</TableHeadCell>
                            {/each}
                            <TableHeadCell class="whitespace-nowrap p-4 font-normal">Actions</TableHeadCell>
                        </TableHead>
                        <TableBody>
                            {#each unit.timeslots as item}
                                <TableBodyRow class="text-gray-900 dark:text-white">
                                    {#each fields as [_header, field, onClick, mono]}
                                        {#if mono}
                                            <span class="font-mono">
                                                <SearchTableCell
                                                    content={field(item)}
                                                    clickable={onClick != null}
                                                    on:click={(e) => onClick?.call(null, item, e.detail)}
                                                />
                                            </span>
                                        {:else}
                                            <SearchTableCell
                                                content={field(item)}
                                                clickable={onClick != null}
                                                on:click={(e) => onClick?.call(null, item, e.detail)}
                                            />
                                        {/if}
                                    {/each}

                                    <div class="whitespace-nowrap p-4 font-normal">
                                        {#each actions as [title, action]}
                                            <button
                                                class="display-inline pr-2 text-primary-600 hover:underline dark:text-primary-500"
                                                on:mousedown={(e) => action(item, new MouseClick(e))}
                                            >
                                                {title}
                                            </button>
                                        {/each}
                                    </div>
                                </TableBodyRow>
                            {/each}
                        </TableBody>
                    {:else}
                        <thead>
                            <tr>
                                <th colspan={fields.length + 1} class="border-none">
                                    <div class="flex flex-col items-center">
                                        <BanOutline class="h-20 w-20" />
                                        <div>There are no items to display</div>
                                    </div>
                                </th>
                            </tr>
                        </thead>
                    {/if}
                {/if}
            {/each}
        </Table>
        {#if timeslots.length == 0}
            <div class="mt-8 flex flex-col items-center">
                <BanOutline class="h-20 w-20" />
                <div>There are no items to display</div>
            </div>
        {/if}
    {:else}
        <div class="mt-8 flex flex-col items-center">
            <Spinner class="h-16 w-16" />
            <div class="mt-2">Loading...</div>
        </div>
    {/if}

    <Edit
        bind:this={editTracker}
        on:update={(e) => {
            displayTracker(e.detail);
        }}
    />

    <Delete bind:isOpen={isDeleteOpen} entity={tracker} on:deleted={() => (window.location.href = "/trackers")} />

    <Delete
        bind:isOpen={isDeleteTimeslotOpen}
        entity={deleteTimeslot}
        on:deleted={(e) => {
            if (timeslots == null) return;
            for (let unit of timeslots) {
                for (let i = 0; i < unit.timeslots.length; i += 1) {
                    if (unit.timeslots[i].id == e.detail.id) {
                        unit.timeslots.splice(i, 1);
                        timeslots = timeslots;
                        return;
                    }
                }
            }
        }}
    />

    <Create
        api={getRoute() + "/timeslots"}
        bind:isOpen={isCreateOpen}
        on:created={(e) => {
            let start = new Date(e.detail.start);
            let startUnit = new Date(
                start.getFullYear() + "-" + String(start.getMonth() + 1).padStart(2, "0") + "-" + String(start.getDate()).padStart(2, "0")
            );
            for (let unit of timeslots || []) {
                if (new Date(unit.start) > startUnit || new Date(unit.end) < startUnit) continue;
                for (let i = 0; i < unit.timeslots.length; i += 1) {
                    if (new Date(unit.timeslots[i].start) > start) {
                        unit.timeslots.splice(i, 0, e.detail);
                        timeslots = timeslots;
                        return;
                    }
                }
                unit.timeslots.push(e.detail);
                timeslots = timeslots;
                return;
            }
        }}
    />
    <EditTimeslot
        bind:this={editTimeslot}
        on:update={(e) => {
            for (let unit of timeslots || []) {
                for (let i = 0; i < unit.timeslots.length; i += 1) {
                    if (unit.timeslots[i].id == e.detail.id) {
                        unit.timeslots[i] = e.detail;
                        timeslots = timeslots;
                        return;
                    }
                }
            }
        }}
    />
{/if}
