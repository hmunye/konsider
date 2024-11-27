<script lang="ts">
import { goto } from "$app/navigation";
import { Button } from "$lib/components/ui/button";

let { data, link = "", filter } = $props();

function buildUrl(
  base: string,
  params: Record<string, string | number | undefined>,
) {
  const queryString = Object.entries(params)
    .filter(([_, value]) => value !== "")
    .map(([key, value]) => `${key}=${value}`)
    .join("&");

  return `${base}?${queryString}`;
}
</script>

{#if data?.metadata}
    <div class="text-muted-foreground text-md">
        Showing Page <strong
            >{data.metadata.current_page} of {data.metadata.last_page}</strong
        >
    </div>
    <div class="flex gap-2">
        <Button
            class="text-md"
            variant={"ghost"}
            on:click={() =>
                goto(
                    buildUrl(`/dashboard/${link}`, {
                        per_page: 8,
                        page: data.metadata.current_page - 1,
                        filter: filter,
                    }),
                )}
            disabled={data.metadata.current_page === 1}
        >
            Previous
        </Button>
        <Button
            class="text-md"
            variant={"ghost"}
            on:click={() =>
                goto(
                    buildUrl(`/dashboard/${link}`, {
                        per_page: 8,
                        page: data.metadata.current_page + 1,
                        filter: filter,
                    }),
                )}
            disabled={data.metadata.current_page === data.metadata.last_page}
        >
            Next
        </Button>
    </div>
{:else}
    <div class="text-muted-foreground text-md">
        Showing Page <strong>1 of 1</strong>
    </div>
    <div class="flex gap-2">
        <Button class="text-md" variant={"ghost"} disabled={true}>
            Previous
        </Button>
        <Button class="text-md" variant={"ghost"} disabled={true}>Next</Button>
    </div>
{/if}
