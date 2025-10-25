<script lang="ts">
    import { Button } from "flowbite-svelte";
    import {
        ArrowRightOutline,
        CashSolid,
        UsersGroupSolid,
        LockSolid
    } from "flowbite-svelte-icons";
    import { onMount } from "svelte";
    import CreateJob from "../../components/job/Create.svelte";
    import CreateTracker from "../../components/tracker/Create.svelte";
    import type { User } from "../../app";

    let isCreateJobOpen = false;
    let isCreateTrackerOpen = false;

    let user: User | null = null;
    let userLoaded = false;

    onMount(async () => {
        user = await window.getUser();
        userLoaded = true;
    });
</script>

<section>
    <div class="py-8">
        <div class="mr-auto place-self-center lg:col-span-7">
            <h1
                class="mb-4 max-w-2xl text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl xl:text-6xl dark:text-gray-50"
            >
                Track Your Time Worked
            </h1>
            <p class="mb-6 max-w-2xl font-light md:text-lg lg:mb-8 lg:text-xl">
                Our TimeTracker helps you effortlessly keep track of the time you spend on different activities.
                You can organize your time entries within individual trackers, and even combine multiple trackers into a single job.
                For example, if your employment contract changes and you need to fulfill a different workload, you can easily adapt your setup without losing your previous records.
            </p>
            <div class="{userLoaded?"":"opacity-0"}">
                {#if user == null}
                    <Button on:click={() => window.location.href = "/login"}>
                        Get started <ArrowRightOutline />
                    </Button>
                {:else}
                    <Button class="mr-2" on:click={() => isCreateJobOpen = true}>
                        Create a Job <ArrowRightOutline />
                    </Button>
                    <Button on:click={() => isCreateTrackerOpen = true}>
                        Create a Tracker <ArrowRightOutline />
                    </Button>
                {/if}
            </div>
        </div>
    </div>
</section>
<section>
    <div class="py-8 sm:py-16">
        <div class="mb-8 max-w-screen-md lg:mb-16">
            <h2 class="mb-4 text-4xl font-extrabold tracking-tight text-gray-900 dark:text-gray-50">
                About us, RiseUpGroup 🚀
            </h2>
            <p class="sm:text-xl">
                At RiseUpGroup, we focus on empowering individuals through technology and
                innovation. Our TimeTracker is just one of the ways we simplify digital life, creating
                tools that add real value and enhance everyday experiences.
            </p>
        </div>
        <div class="space-y-8 md:grid md:grid-cols-2 md:gap-12 md:space-y-0 lg:grid-cols-3">
            <div>
                <div
                    class="mb-4 flex h-10 w-10 items-center justify-center rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900"
                >
                    <CashSolid />
                </div>
                <h3 class="mb-2 text-xl font-bold text-gray-900 dark:text-gray-50">Free for all</h3>
                <p>
                    This TimeTracker is completely free to use, with no hidden costs or fees.
                    It’s a simple, effective tool that helps you record your time more efficiently and manage your work more professionally.
                </p>
            </div>
            <div>
                <div
                    class="mb-4 flex h-10 w-10 items-center justify-center rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900"
                >
                    <UsersGroupSolid />
                </div>
                <h3 class="mb-2 text-xl font-bold text-gray-900 dark:text-gray-50">Security</h3>
                <p>
                    Due to our authentication server, TimeTracker can be used as a single user or as a
                    team. We provide a secure and reliable service that you can trust.
                </p>
            </div>
            <div>
                <div
                    class="mb-4 flex h-10 w-10 items-center justify-center rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900"
                >
                    <LockSolid />
                </div>
                <h3 class="mb-2 text-xl font-bold text-gray-900 dark:text-gray-50">Privacy</h3>
                <p>
                    We take your privacy seriously. Our TimeTracker never knows any private Data about
                    you. Since the authentication is handled by our AuthServer, a service like this
                    one never knows your password. In fact, TimeTracker doesn't even know your email
                    address. Only a user id and a user name is stored in your browser.
                </p>
            </div>
        </div>
    </div>
</section>
<section>
    <div class="max-w-screen-lg py-8 sm:text-lg lg:py-16">
        <h2 class="mb-4 text-4xl font-bold tracking-tight text-gray-900 dark:text-gray-50">
            We didn't reinvent the wheel
        </h2>
        <p class="mb-4 font-light">
            ...but we made it smoother. At RiseUpGroup, we take what works and refine it, tweaking
            every detail until it meets our standards. Our TimeTracker, like everything we create, is
            built to be exactly how we want it – simple, effective, and just right.
        </p>
        <p class="mb-4 font-medium">
            We started with a simple problem: as computer science students working part-time, we had to write down our hours on a piece of paper.
            Our TimeTracker was born to make this process simple and stress-free.
            Now you can edit and manage your hours from anywhere.
            The only thing left is that, at the end of the month, you still have to copy everything back onto that old piece of paper.
        </p>
    </div>
</section>

<CreateJob
    bind:isOpen={isCreateJobOpen}
    on:created={(e) => {
        let job = e.detail;
        window.location.href = "/jobs/" + job.primary();
    }}
/>

<CreateTracker
    bind:isOpen={isCreateTrackerOpen}
    on:created={(e) => {
        let tracker = e.detail;
        window.location.href = "/trackers/" + tracker.primary();
    }}
/>
