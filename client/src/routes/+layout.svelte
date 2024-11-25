<script lang="ts">
    import "../app.css";

    let { children } = $props();
</script>

<main class="min-h-screen flex flex-col items-center antialiased">
    <section class="flex flex-1 flex-col w-full items-center">
        {@render children()}
    </section>
</main>
