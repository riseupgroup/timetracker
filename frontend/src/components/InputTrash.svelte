<script lang="ts">
    import { ButtonGroup, FloatingLabelInput, InputAddon, type InputType } from "flowbite-svelte";
    import { CircleMinusSolid, TrashBinSolid } from "flowbite-svelte-icons";

    export let name: string;
    export let type: InputType = "text";
    export let value: string = "";
    export let disabled: boolean = false;
    let classGroup: string | null = null;
    export { classGroup as class };
    export let classBackground: string | null = null;
</script>

<ButtonGroup class={classGroup}>
    <FloatingLabelInput
        {type}
        bind:value
        {disabled}
        style="outlined"
        classDiv="flex-grow"
        classLabel="cursor-text {classBackground}"
        classInput="rounded-r-none h-full"
    >
        {name}
    </FloatingLabelInput>
    <InputAddon class="border-gray-300 dark:border-gray-600 {classBackground}">
        <button
            class="-mx-3 inline-flex items-center px-3 {((value, disabled) => {
                if (value == '') return 'cursor-default';
                if (disabled) return 'cursor-not-allowed';
                return '';
            })(value, disabled)}"
            style="min-height: calc(1.625rem + 1.25em)"
            on:click={() => {
                if (!disabled) value = "";
            }}
        >
            {#if value == ""}
                <CircleMinusSolid color="gray" />
            {:else}
                <TrashBinSolid />
            {/if}
        </button>
    </InputAddon>
</ButtonGroup>
