<script lang="ts">
    import { Button } from "$lib/components/ui/button/index";
    import { theme } from "$lib/stores/theme";
    import Moon from "lucide-svelte/icons/moon";
    import Sun from "lucide-svelte/icons/sun";

    let isDarkMode: boolean = $derived($theme === "dark");
</script>

<Button
    on:click={() => theme.set(isDarkMode ? "light" : "dark")}
    class="rounded-lg"
    variant="outline"
    size="icon"
>
    {#if isDarkMode}
        <Moon />
    {:else}
        <Sun />
    {/if}
</Button>
