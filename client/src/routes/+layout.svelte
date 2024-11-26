<script lang="ts">
    import "../app.css";
    import Footer from "$lib/components/ui/footer/footer.svelte";
    import { Toaster } from "$lib/components/ui/sonner";
    import { theme } from "$lib/stores/theme";

    let toastTheme: "dark" | "light" | "system" | undefined = $derived(
        $theme === "dark" ? "dark" : "light",
    );

    let { children } = $props();
</script>

<Toaster richColors theme={toastTheme} />

<main class="min-h-screen flex flex-col items-center antialiased">
    <section class="flex flex-1 flex-col w-full items-center">
        {@render children()}
    </section>
    <Footer />
</main>
