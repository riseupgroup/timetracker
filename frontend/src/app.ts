// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
    interface Window {
        user: User | null;
        userPromise: Promise<User | null> | null;
        refreshUser: () => Promise<User | null>;
        getUser: () => Promise<User | null>;
    }
}

export const timezone = encodeURI(Intl.DateTimeFormat().resolvedOptions().timeZone);
export const browserLanguage = navigator.language || (navigator.languages || ["en-us"])[0];

export enum RowStyle {
    Disabled,
    Underlined,
    Bold
}

export interface Entity {
    primary(): number;
    resource(): string;
    resourceName(): string;
    display(): string;
    rowStyle(): RowStyle | string | null;
}

export type User = {
    id: string;
    name: string;
};

export class ListResult<T> {
    count: number = 0;
    items: T[] = [];
}

export type Unit = {
    start: string;
    end: string;
    timeslots: Timeslot[];
};

export enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
    Back = 3,
    Forward = 4
}

export class MouseClick {
    button: MouseButton;

    constructor(MouseEvent: MouseEvent) {
        switch (MouseEvent.button) {
            case 0:
                if (MouseEvent.ctrlKey || MouseEvent.metaKey) {
                    this.button = MouseButton.Middle;
                } else {
                    this.button = MouseButton.Left;
                }
                break;
            case 1:
                this.button = MouseButton.Middle;
                break;
            case 2:
                this.button = MouseButton.Right;
                break;
            default:
                this.button = MouseButton.Left;
                break;
        }
    }

    goto(url: string) {
        switch (this.button) {
            case MouseButton.Left:
                window.location.href = url;
                break;
            case MouseButton.Middle:
                window.open(url, "_blank");
                break;
            default:
                break;
        }
    }
}

export function getLocalTimestamp(d: Date): string {
    return (
        d.getFullYear() +
        "-" +
        (d.getMonth() + 1).toString().padStart(2, "0") +
        "-" +
        d.getDate().toString().padStart(2, "0") +
        "T" +
        d.getHours().toString().padStart(2, "0") +
        ":" +
        d.getMinutes().toString().padStart(2, "0") +
        ":" +
        d.getSeconds().toString().padStart(2, "0")
    );
}

export function getWeekNumber(date: Date): number {
    // Copy date so don't modify original
    const d = new Date(Date.UTC(date.getFullYear(), date.getMonth(), date.getDate()));
    // Set to nearest Thursday: current date + 4 - current day number
    // Make Sunday's day number 7
    d.setDate(d.getDate() + 4 - (d.getDay() || 7));
    // Get first day of year
    const yearStart = new Date(d.getUTCFullYear(), 0, 1);
    // Calculate full weeks to nearest Thursday
    return Math.ceil(((d.getTime() - yearStart.getTime()) / 86400000 + 1) / 7);
}

export function getYearFromWeek(date: Date): number {
    // Copy date so don't modify original
    const d = new Date(Date.UTC(date.getFullYear(), date.getMonth(), date.getDate()));
    // Set to nearest Thursday: current date + 4 - current day number
    // Make Sunday's day number 7
    d.setDate(d.getDate() + 4 - (d.getDay() || 7));
    return d.getUTCFullYear();
}

export function formatDuration(durationSeconds: number, short: boolean = false): string {
    const isNegative = durationSeconds < 0;
    durationSeconds = Math.abs(durationSeconds);
    const hours = Math.floor(durationSeconds / 3600);
    const minutes = Math.floor((durationSeconds % 3600) / 60);
    const seconds = Math.floor(durationSeconds % 60);
    if (short) {
        if (durationSeconds == 0) return "-";
        let output = "";
        if (hours > 0) output += hours + "h ";
        if (minutes > 0) output += minutes + "m ";
        if (seconds > 0) output += seconds + "s ";
        return isNegative ? "-" : "" + output.trim();
    }

    if (isNegative) return "-";

    return "" + hours + "h " + minutes.toString().padStart(2, "0") + "m " + seconds.toString().padStart(2, "0") + "s";
}

export class Job implements Entity {
    id: number = 0;
    name: string | null = null;
    companyName: string | null = null;
    companyLogo: string | null = null;
    description: string | null = null;
    created: string = "0000-00-00T00:00:00";
    disabled: boolean = false;
    activeTracker: Tracker | null = null;

    primary(): number {
        return this.id;
    }
    resource(): string {
        return "/api/jobs/" + this.primary();
    }
    resourceName(): string {
        return "job";
    }
    display(): string {
        return this.companyName || "Job " + this.primary();
    }
    rowStyle(): RowStyle | string | null {
        return this.disabled ? RowStyle.Disabled : null;
    }
}

export class Tracker implements Entity {
    id: number = 0;
    name: string | null = null;
    job: number | null = null;
    owner: number | null = null;
    created: string = "0000-00-00T00:00:00";
    validFrom: string | null = null;
    validUntil: string | null = null;
    timePensum: number | null = null;
    timePensumUnit: string = TimePensumUnit[TimePensumUnit.None];
    displayRangeUnit: string = DisplayRangeUnit[DisplayRangeUnit.Month];
    timeWorked: number = 0;
    isActive: boolean = false;

    primary(): number {
        return this.id;
    }

    resource(): string {
        return this.job ? "/api/jobs/" + this.job + "/trackers/" + this.primary() : "/api/trackers/" + this.primary();
    }

    resourceName(): string {
        return "tracker";
    }

    display(): string {
        return this.name || "Tracker " + this.primary();
    }

    rowStyle(): RowStyle | string | null {
        const validFrom = new Date(this.validFrom || "");
        const validUntil = new Date(this.validUntil || "");
        const now = new Date();
        const disabled = validFrom > now || validUntil < now;
        if (disabled) return RowStyle.Disabled;
        if (this.job != null && this.isActive) return RowStyle.Bold;
        return null;
    }
}

// Ordering has to be from small to big
export enum TimePensumUnit {
    Week,
    Month,
    Year,
    None
}

// Ordering has to be the same as TimePensumUnit
export enum DisplayRangeUnit {
    Week,
    Month,
    Year
}

export class Timeslot implements Entity {
    id: number = 0;
    tracker: number = 0;
    start: string = "0000-00-00T00:00:00";
    end: string | null = null;
    comment: string | null = null;

    primary(): number {
        return this.id;
    }

    resource(): string {
        return "/api/timeslots/" + this.primary();
    }

    resourceName(): string {
        return "timeslot";
    }

    display(): string {
        return this.comment || "Timeslot " + this.primary();
    }

    rowStyle(): RowStyle | string | null {
        return this.end == null ? RowStyle.Bold : null;
    }
}

export class ApiKey implements Entity {
    id: number = 0;
    name: string | null = null;
    disabled: boolean = false;
    validUntil: string | null = null;
    added: string = "";
    lastChanged: string = "";
    lastUsed: string | null = null;

    primary(): number {
        return this.id;
    }

    resource(): string {
        return "/api/keys/" + this.id;
    }

    resourceName(): string {
        return "api key";
    }

    display(): string {
        return this.name ? this.name : "Key " + this.primary();
    }

    isDisabled(): boolean {
        return this.disabled;
    }

    rowStyle(): RowStyle | string | null {
        return this.disabled ? RowStyle.Disabled : null;
    }
}

export class ApiKeyResponse {
    id: number = 0;
    key: string = "";
    name: string | null = null;
}
