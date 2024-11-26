<script lang="ts">
    import Rabbit from "lucide-svelte/icons/rabbit";

    let { hidden = false } = $props();
</script>

<div class="flex justify-evenly items-center gap-4">
    <div class="flex items-center gap-2 text-lg font-semibold md:text-base">
        <Rabbit class="h-9 w-9" />
        <span class="sr-only">Konsider</span>
    </div>
    {#if !hidden}
        <h1 class="text-3xl font-bold">KONSIDER</h1>
    {/if}
</div>
