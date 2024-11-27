<script lang="ts">
import Navbar from "$lib/components/custom/navbar/navbar.svelte";

let { children, data } = $props();
</script>

<div class="flex w-full flex-col overflow-hidden">
    <header
        class="bg-background sticky top-0 flex h-20 items-center gap-4 border-b px-4 md:px-6"
    >
        <Navbar data={data.current_user} />
    </header>
    <main class="flex flex-1 flex-col gap-4 p-4 md:gap-8 md:p-8 mt-4">
        {@render children()}
    </main>
</div>
