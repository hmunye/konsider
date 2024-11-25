<script lang="ts">
    import type { PageData } from "./$types.js";
    import Hero from "$lib/components/ui/hero/hero.svelte";
    import LoginForm from "$lib/components/forms/login/login-form.svelte";
    import Navbar from "$lib/components/ui/navbar/navbar.svelte";

    let { data }: { data: PageData } = $props();
</script>

<Navbar />
<div class="flex flex-1 flex-row w-full items-center px-3 animate-in">
    <div class="hidden w-[400px] ml-32 lg:block">
        <Hero />
    </div>
    <LoginForm data={data.form} />
</div>
