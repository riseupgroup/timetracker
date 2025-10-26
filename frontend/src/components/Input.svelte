<script lang="ts">
    import { FloatingLabelInput, type InputType } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";

    export let classLabel: string | null = null;
    export let classInput: string | null = null;
    export let type: InputType = "text";
    export let maxlength: number | null = null;
    export let required: boolean = false;
    export let name: string;
    export let value: string = type == "number" ? "0" : "";

    let input: FloatingLabelInput;
    const dispatch = createEventDispatcher();
    let changeTimeout: number | null = null;
    let lastValue: string | number;
    export const focus = () => input.focus();

    function onChange() {
        if (value != lastValue) {
            lastValue = value;
            dispatch("change", { value: value });
        }
    }

    function onInput(e: KeyboardEvent) {
        if (type == "number") {
            if (e.key != "Backspace" && e.key != "Tab" && !e.key.startsWith("Arrow") && isNaN(parseInt(e.key))) e.preventDefault();
        }
        if (value != lastValue) {
            if (changeTimeout != null) clearTimeout(changeTimeout);
            changeTimeout = setTimeout(() => {
                lastValue = value;
                dispatch("changeExtended", { value: value });
            }, 500);
        }
    }
</script>

<FloatingLabelInput
    {type}
    {required}
    {maxlength}
    bind:value
    bind:this={input}
    on:change={onChange}
    on:keyup={onInput}
    style="outlined"
    {classInput}
    {classLabel}
>
    {name}
</FloatingLabelInput>
