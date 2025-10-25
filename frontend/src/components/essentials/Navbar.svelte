<script lang="ts">
    import {
        Navbar,
        NavBrand,
        NavLi,
        NavUl,
        NavHamburger,
        Avatar,
        Dropdown,
        DropdownItem,
        DropdownHeader,
        DropdownDivider
    } from "flowbite-svelte";
    import { MouseClick, type User } from "../../app.ts";
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";
    $: activeUrl = $page.url.pathname;

    let user: User | null = null;

    onMount(async () => {
        user = await window.getUser();
    });

    function login() {
        document.cookie = "path=" + $page.url.pathname + "; path=/";
        window.location.href = "/auth";
    }

    async function logout() {
        let res = await fetch("/auth/logout", {
            method: "POST"
        });
        if (res.ok) {
            window.userPromise = null;
            window.user = null;
            user = null;
            goto("/");
        } else {
            alert(res.text());
        }
    }
</script>

<Navbar color="dark" class="border-b border-gray-200 dark:border-gray-600">
    <NavBrand href="/" class="order-0 flex-grow">
        <img
            src="https://github.com/riseupgroup.png"
            class="me-3 h-6 w-6 sm:h-9 sm:w-9"
            alt="RiseUpGroup Logo"
        />
        <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
            TimeTracker <span class="hidden sm:inline-block"> | RiseUpGroup</span>
        </span>
    </NavBrand>
    <NavUl class="order-2 md:order-1" {activeUrl}>
        <NavLi
            href="/"
            on:click={(mouseEvent) => {
                mouseEvent.preventDefault();
                new MouseClick(mouseEvent).goto("/");
            }}>Home</NavLi>
        {#if user != null}
            <NavLi
                href="/jobs"
                on:click={(mouseEvent) => {
                    mouseEvent.preventDefault();
                    new MouseClick(mouseEvent).goto("/jobs");
                }}>Jobs</NavLi
            >
            <NavLi
                href="/trackers"
                on:click={(mouseEvent) => {
                    mouseEvent.preventDefault();
                    new MouseClick(mouseEvent).goto("/trackers");
                }}>Trackers</NavLi
            >
            <NavLi href="http://github.com/riseupgroup">GitHub</NavLi>
        {:else}
            <NavLi href="http://github.com/riseupgroup">GitHub</NavLi>
            <NavLi class="cursor-pointer" on:click={login}>Login</NavLi>
        {/if}
    </NavUl>
    <div class="order-1 mx-4 flex flex-row md:order-2">
        {#if user != null}
            <Avatar
                class="cursor-pointer"
                id="avatar-menu"
                src={"/auth/users/" + user.id + "/picture"}
            />
            <Dropdown placement="bottom-end" class="w-40" triggeredBy="#avatar-menu">
                <DropdownHeader>
                    <span class="block text-sm">{user.name}</span>
                </DropdownHeader>
                <DropdownItem href="https://auth.riseupgroup.net">AuthServer</DropdownItem>
                <DropdownDivider />
                <DropdownItem on:click={logout}>Sign out</DropdownItem>
            </Dropdown>
        {/if}
        <NavHamburger />
    </div>
</Navbar>
