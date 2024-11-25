<script lang="ts">
import ThemeToggle from "$lib/components/theme-toggle.svelte";
import Package2 from "lucide-svelte/icons/package-2";
</script>

<nav
    class="flex w-full justify-between items-center border-b border-b-foreground/10 p-2 py-5 px-10 lg:px-20"
>
    <div class="flex justify-evenly items-center gap-4">
        <a
            href="##"
            class="flex items-center gap-2 text-lg font-semibold md:text-base"
        >
            <Package2 class="h-8 w-8" />
            <span class="sr-only">Konsider</span>
        </a>
        <h1 class="text-3xl font-bold">KONSIDER</h1>
    </div>
    <ThemeToggle />
</nav>
