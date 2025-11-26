<script lang="ts">
    import { Label, Textarea } from "flowbite-svelte";

    export let text: string = "";
    export let rows: number = 3;
    export let placeholder: string = "Enter text";
</script>

<div class="relative">
    <Label
        class="absolute start-1 top-2 z-10 origin-left -translate-y-4 scale-75 cursor-text bg-white px-2 text-sm text-gray-500 peer-placeholder-shown:top-1/2 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:scale-100 peer-focus:top-2 peer-focus:-translate-y-4 peer-focus:scale-75 peer-focus:px-2 peer-focus:text-primary-600 rtl:origin-right dark:bg-gray-800 dark:text-gray-400 peer-focus:dark:text-primary-500"
        ><slot /></Label
    >
    <Textarea bind:value={text} class="bg-white dark:bg-gray-800" {placeholder} {rows} />
</div>
