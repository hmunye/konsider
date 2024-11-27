<script lang="ts">
import "../app.css";
import { Toaster } from "$lib/components/ui/sonner";
import { theme } from "$lib/stores/theme";
import Footer from "$lib/components/custom/footer/footer.svelte";

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
