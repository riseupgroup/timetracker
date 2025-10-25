<script lang="ts">
    let classDiv: string = "";
    export { classDiv as class };
</script>

<div
    class="divide-gray-200 rounded-md bg-gray-50 px-4 py-2 text-gray-500 dark:divide-gray-700 dark:bg-gray-800 dark:text-gray-400 {classDiv
        ? classDiv
        : ''}"
>
    <slot />
</div>
