<script lang="ts">
    import "../app.css";
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";

    const NO_LOGIN = ["", "login", "about"];
    let show: boolean = false;

    page.subscribe(async (page) => {
        if (page.url != null) {
            let path = trimPath(page.url.pathname);
            if (NO_LOGIN.includes(path)) {
                show = true;
            } else {
                let user = await window.getUser();
                if (user != null) {
                    show = true;
                } else {
                    goto("/login?path=/" + path);
                }
            }
        }
    });

    function trimPath(path: string): string {
        let start = path[0] == "/" ? 1 : 0;
        let end = path.length;
        if (path[end - 1] == "/" && end - 1 >= start) {
            end -= 1;
        }
        if (start != 0 || end != path.length) {
            return path.substring(start, end);
        }
        return path;
    }
</script>

{#if show}
    <slot />
{/if}
