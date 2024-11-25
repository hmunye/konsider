<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import ThemeToggle from "$lib/components/theme-toggle.svelte";
import { Button } from "$lib/components/ui/button";
import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
import { Input } from "$lib/components/ui/input";
import * as Sheet from "$lib/components/ui/sheet/index.js";
import { fetchRequest } from "$lib/fetch";
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import { userStore } from "$lib/stores/userStore";
import CircleUser from "lucide-svelte/icons/circle-user";
import Menu from "lucide-svelte/icons/menu";
import Package2 from "lucide-svelte/icons/package-2";
import Search from "lucide-svelte/icons/search";
import { toast } from "svelte-sonner";

let { children, data } = $props();

userStore.set(data.user);

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

  goto("/", {
    replaceState: true,
  });
};

let sheetOpen: boolean = $state(false);

let currentPath = $derived($page.url.pathname);
</script>

<div class="flex w-full flex-col overflow-hidden">
    <header
        class="bg-background sticky top-0 flex h-20 items-center gap-4 border-b px-4 md:px-6"
    >
        <nav
            class="hidden flex-col gap-6 text-xl font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-xl lg:gap-6"
        >
            <a
                href="##"
                class="flex items-center gap-2 text-lg font-semibold md:text-base"
            >
                <Package2 class="h-8 w-8" />
                <span class="sr-only">Konsider</span>
            </a>
            <a
                href="/admin"
                class={`transition-colors ${currentPath === "/admin" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
            >
                Dashboard
            </a>
            <a
                href="/admin/users"
                class={`transition-colors ${currentPath === "/admin/users" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
            >
                Users
            </a>
            <a
                href="/admin/requests"
                class={`transition-colors ${currentPath === "/admin/requests" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
            >
                Requests
            </a>
            <a
                href="/admin/software"
                class={`transition-colors ${currentPath === "/admin/software" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
            >
                Software
            </a>
            <a
                href="/admin/reviews"
                class={`transition-colors ${currentPath === "/admin/reviews" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
            >
                Reviews
            </a>
        </nav>
        <Sheet.Root bind:open={sheetOpen}>
            <Sheet.Trigger
                asChild
                let:builder
                on:click={() => (sheetOpen = true)}
            >
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
                    <a
                        href="##"
                        class="flex items-center gap-2 text-lg font-semibold"
                    >
                        <Package2 class="h-8 w-8" />
                        <span class="sr-only">Konsider</span>
                    </a>
                    <a
                        href="/admin"
                        class={`transition-colors ${currentPath === "/admin/dashboard" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
                        onclick={() => (sheetOpen = false)}
                    >
                        Dashboard
                    </a>
                    <a
                        href="/admin/users"
                        class={`transition-colors ${currentPath === "/admin/users" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
                        onclick={() => (sheetOpen = false)}
                    >
                        Users
                    </a>
                    <a
                        href="/admin/requests"
                        class={`transition-colors ${currentPath === "/admin/requests" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
                        onclick={() => (sheetOpen = false)}
                    >
                        Requests
                    </a>
                    <a
                        href="/admin/software"
                        class={`transition-colors ${currentPath === "/admin/software" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
                        onclick={() => (sheetOpen = false)}
                    >
                        Software
                    </a>
                    <a
                        href="/admin/reviews"
                        class={`transition-colors ${currentPath === "/admin/reviews" ? "text-foreground font-bold" : "text-muted-foreground"} hover:text-foreground`}
                        onclick={() => (sheetOpen = false)}
                    >
                        Reviews
                    </a>
                </nav>
            </Sheet.Content>
        </Sheet.Root>
        <div
            class="relative flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4"
        >
            <form class="ml-auto flex-1 sm:flex-initial">
                <div class="relative">
                    <Search
                        class="text-muted-foreground absolute left-2.5 top-2.5 h-4 w-4"
                    />
                    <Input
                        type="search"
                        name="search"
                        placeholder="Search..."
                        class="pl-8 sm:w-[300px] md:w-[200px] lg:w-[300px] text-lg"
                    />
                </div>
            </form>
            <DropdownMenu.Root>
                <DropdownMenu.Trigger asChild let:builder>
                    <ThemeToggle />
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
                        <DropdownMenu.Label
                            class="text-sm text-muted-foreground"
                        >
                            Role: {$userStore?.role}
                        </DropdownMenu.Label>
                    </div>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item class="text-md"
                        >Settings</DropdownMenu.Item
                    >
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item class="text-md" on:click={handleLogOut}
                        >Logout</DropdownMenu.Item
                    >
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </div>
    </header>
    <main class="flex flex-1 flex-col gap-4 p-4 md:gap-8 md:p-8 mt-4">
        {@render children()}
    </main>
</div>
