<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import { Button } from "$lib/components/ui/button";
import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
import Logo from "$lib/components/custom/logo/logo.svelte";
import * as Sheet from "$lib/components/ui/sheet/index.js";
import { fetchRequest } from "$lib/fetch";
import { userStore } from "$lib/stores/userStore";
import CircleUser from "lucide-svelte/icons/circle-user";
import Menu from "lucide-svelte/icons/menu";
import { toast } from "svelte-sonner";
import ThemeToggle from "../theme-toggle/theme-toggle.svelte";

let { data = undefined } = $props();

if (data) userStore.set(data);

const handleLogOut = async () => {
  const response = await fetchRequest({
    url: `${PUBLIC_BASE_API_URL}/api/v1/auth/logout`,
    method: "POST",
  });

  if (response.error) {
    toast.error(
      response.error.message ?? "Error occured handling logout reqeust",
    );
    return;
  }

  userStore.set(null);

  goto("/", {});
};

let sheetOpen: boolean = $state(false);

let currentPath = $derived($page.url.pathname);

const paths = [
  { path: "/dashboard", label: "Dashboard" },
  { path: "/dashboard/users", label: "Users" },
  { path: "/dashboard/requesters", label: "Requesters" },
  { path: "/dashboard/software", label: "Software" },
  { path: "/dashboard/requests", label: "Requests" },
  { path: "/dashboard/reviews", label: "Reviews" },
];
</script>

{#if data === undefined}
    <nav
        class="flex w-full justify-between items-center border-b border-b-foreground/10 p-2 py-5 px-10 lg:px-20"
    >
        <Logo />
        <ThemeToggle />
    </nav>
{:else}
    <nav
        class="hidden flex-col gap-6 text-xl font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-xl lg:gap-6"
    >
        <Logo hidden={true} />
        {#each paths as { path, label }}
            <a
                href={path}
                class={`transition-colors ${
                    currentPath === path
                        ? "text-foreground font-bold"
                        : "text-muted-foreground"
                } hover:text-foreground`}
            >
                {label}
            </a>
        {/each}
    </nav>
    <Sheet.Root bind:open={sheetOpen}>
        <Sheet.Trigger asChild let:builder>
            <Button
                variant="outline"
                size="icon"
                class="shrink-0 md:hidden"
                builders={[builder]}
            >
                <Menu class="h-6 w-6" />
                <span class="sr-only">Toggle navigation menu</span>
            </Button>
        </Sheet.Trigger>
        <Sheet.Content side="left">
            <nav class="grid gap-6 text-xl font-medium">
                <Logo hidden={true} />
                {#each paths as { path, label }}
                    <a
                        href={path}
                        class={`transition-colors ${
                            currentPath === path
                                ? "text-foreground font-bold"
                                : "text-muted-foreground"
                        } hover:text-foreground`}
                        onclick={() => (sheetOpen = false)}
                    >
                        {label}
                    </a>
                {/each}
            </nav>
        </Sheet.Content>
    </Sheet.Root>
    <div
        class="relative flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4"
    >
        <div class="mx-auto">
            <h1 class="text-3xl font-bold block md:hidden">KONSIDER</h1>
        </div>
        <ThemeToggle />
        <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild let:builder>
                <Button
                    builders={[builder]}
                    variant="secondary"
                    size="icon"
                    class="rounded-full"
                >
                    <CircleUser class="h-6 w-6" />
                    <span class="sr-only">Toggle user menu</span>
                </Button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Content align="end">
                <div class="flex flex-col">
                    <DropdownMenu.Label class="text-lg font-semibold">
                        {$userStore?.name}
                    </DropdownMenu.Label>
                    <DropdownMenu.Label class="text-sm text-muted-foreground">
                        Role: {$userStore?.role}
                    </DropdownMenu.Label>
                </div>
                <DropdownMenu.Separator />
                <DropdownMenu.Item class="text-md" on:click={handleLogOut}
                    >Logout</DropdownMenu.Item
                >
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>
{/if}
