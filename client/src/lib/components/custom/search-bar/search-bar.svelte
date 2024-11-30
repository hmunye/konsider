<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import { Button } from "$lib/components/ui/button";
import { Input } from "$lib/components/ui/input";
import * as Select from "$lib/components/ui/select/index";
import Search from "lucide-svelte/icons/search";
import { toast } from "svelte-sonner";

let {
  link,
  filterList,
}: { link: string; filterList: { value: string; label: string }[] } = $props();

let filter = $page.url.searchParams.get("filter") || "";
let filterParts = filter.split(":");

let searchQuery = $state("");
let selectedFilter = $state("");

const urlContainsFilter = filterList.some(
  (item) =>
    item.value === filterParts[0] &&
    item.label ===
      filterParts[0].charAt(0).toUpperCase() + filterParts[0].slice(1),
);

if (urlContainsFilter) {
  searchQuery = filterParts[1];
  selectedFilter = filterParts[0];
}

function handleSubmit(event: Event) {
  event.preventDefault();

  if (!searchQuery.trim()) {
    toast.error("No search query provided");
    return;
  }

  if (!selectedFilter.trim()) {
    toast.error("No filter selected");
    return;
  }

  if (selectedFilter === "role") searchQuery.toUpperCase();

  goto(`/dashboard/${link}?filter=${selectedFilter}:${searchQuery}`);
}

function handleReset() {
  searchQuery = "";
  selectedFilter = "";

  goto(`/dashboard/${link}`);
}
</script>

<form
    class="ml-auto flex flex-col sm:flex-row gap-4 sm:gap-4 w-full items-center"
    onsubmit={handleSubmit}
>
    <div class="relative">
        <Search
            class="text-muted-foreground absolute left-2.5 top-2.5 h-4 w-4"
        />
        <Input
            type="search"
            bind:value={searchQuery}
            name="search"
            placeholder={`Search by ${filterList.find((f: { value: string; label: string }) => f.value === selectedFilter)?.label || ""}...`}
            class="pl-8 sm:w-[200px] md:w-[200px] lg:w-[300px] text-lg"
        />
    </div>

    <Select.Root
        selected={{
            label:
                selectedFilter.charAt(0).toUpperCase() +
                selectedFilter.slice(1),
            value: selectedFilter,
        }}
        portal={null}
    >
        <Select.Trigger class="w-[180px]">
            <Select.Value placeholder="Filter By" class="text-lg" />
        </Select.Trigger>
        <Select.Content>
            <Select.Group>
                {#each filterList as filter}
                    <Select.Item
                        value={filter.value}
                        label={filter.label}
                        on:click={() => (selectedFilter = filter.value)}
                        class="text-md"
                    >
                        {filter.label}
                    </Select.Item>
                {/each}
            </Select.Group>
        </Select.Content>
        <Select.Input name="filterBy" />
    </Select.Root>

    <div>
        <Button variant="link" type="submit" class="text-success text-md">
            Apply
        </Button>
        <Button
            variant="link"
            type="button"
            class="text-destructive mr-0 md:mr-5 text-md"
            onclick={handleReset}
        >
            Reset
        </Button>
    </div>
</form>
