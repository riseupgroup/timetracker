<script lang="ts">
    import { writable, type Writable } from "svelte/store";
    import { DisplayRangeUnit, formatDuration, type Timeslot, type Unit } from "../../app";
    import { onDestroy } from "svelte";

    export let timeslots: Timeslot[] | Unit[];
    export let filterUnit: string | null = null;
    export let filterPrimary: number | null = null;
    export let filterSecondary: number | null = null;
    let interval: number | null;
    let updateTimeout: number | null;
    let output: Writable<string> = writable();
    $: timeslots, update();

    function calculateTimeWorked(timeslots: Timeslot[], now: Date): [number, number, Date | null] {
        let time = 0;
        let counting = 0;
        let update: Date | null = null;
        for (let timeslot of timeslots) {
            let start = new Date(timeslot.start);
            if (filterUnit == null
                || filterUnit == DisplayRangeUnit[DisplayRangeUnit.Week]
                || (filterUnit == DisplayRangeUnit[DisplayRangeUnit.Month] && start.getMonth()+1 == filterSecondary)
                || (filterUnit == DisplayRangeUnit[DisplayRangeUnit.Year] && start.getFullYear() == filterPrimary))
            {
                let end;
                if (timeslot.end == null) {
                    if (start > now) {
                        if (update == null || start.getTime() < update.getTime()) {
                            update = start;
                        }
                        continue;
                    }
                    end = now;
                    counting += 1;
                } else {
                    end = new Date(timeslot.end);
                }
                time += end.getTime() - start.getTime();
            }
        }
        return [time, counting, update]
    }

    function update() {
        if (interval) {
            clearInterval(interval);
            interval = null;
        }
        if (updateTimeout) {
            clearTimeout(updateTimeout);
            updateTimeout = null;
        }
        if (timeslots.length != 0) {
            let now = new Date();
            let time = 0;
            let counting = 0;
            let updateDate: Date | null = null;

            if ((<Unit>timeslots[0]).timeslots !== undefined) {
                for (let unit of timeslots as Unit[]) {
                    let [unitTime, unitCounting, unitUpdate] = calculateTimeWorked(unit.timeslots, now);
                    time += unitTime;
                    counting += unitCounting;
                    if (unitUpdate && (updateDate == null || unitUpdate.getTime() < updateDate.getTime())) {
                        updateDate = unitUpdate;
                    }
                }
            } else {
                let [unitTime, unitCounting, unitUpdate] = calculateTimeWorked(timeslots as Timeslot[], now);
                time = unitTime;
                counting = unitCounting;
                updateDate = unitUpdate;
            }
            output.set(formatDuration(time/1000, counting == 0));
            if (counting != 0) {
                interval = setInterval(() => {
                    let newTime = time + (new Date().getTime() - now.getTime()) * counting;
                    output.set(formatDuration(newTime/1000, false));
                }, 1000);
            }
            if (updateDate) {
                updateTimeout = setTimeout(update, Math.max(0, updateDate.getTime() - new Date().getTime()))
            }
        } else {
            output.set("-");
        }
    }

    onDestroy(() => {
        if (interval) {
            clearInterval(interval);
            interval = null;
        }
        if (updateTimeout) {
            clearTimeout(updateTimeout);
            updateTimeout = null;
        }
    })
</script>

{$output}
