<script lang="ts">
    export let open: boolean = false;
</script>

<div style="position: relative">
    <slot />
    <style lang="scss">
        .custom-tooltip {
            position: absolute;
            left: 0;
            right: 0;
            height: 0;
        }

        .custom-tooltip > div.arrow {
            position: absolute;
            left: 50%;
            width: 10px;
            height: 10px;
            transform: translateX(-50%) rotate(45deg);
        }

        .custom-tooltip > div[role="tooltip"] {
            position: absolute;
            top: 5px;
            left: 50%;
            transform: translateX(-50%);
        }
    </style>
    <div class="custom-tooltip" style="display: {open ? 'block' : 'none'}">
        <div
            role="tooltip"
            class="tooltip rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white shadow-md dark:bg-gray-700"
        >
            <slot name="content" />
        </div>
        <div class="arrow bg-gray-900 dark:bg-gray-700"></div>
    </div>
</div>
