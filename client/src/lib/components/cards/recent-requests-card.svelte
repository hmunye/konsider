<script lang="ts">
import * as Avatar from "$lib/components/ui/avatar/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import { formatDate, getRandomColor } from "$lib/utils";

let { softwareRequests } = $props();
</script>

<Card.Root
    data-x-chunk-name="dashboard-01-chunk-5"
    data-x-chunk-description="A card showing a list of recent requests with requester names and details."
>
    <Card.Header>
        <Card.Title class="text-2xl">Recent Requests</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-8">
        {#if softwareRequests && softwareRequests.software_requests.length > 0}
            {#each softwareRequests.software_requests as request}
                <div class="flex items-center gap-4">
                    <Avatar.Root class="hidden h-10 w-10 sm:flex">
                        <Avatar.Fallback
                            class={`text-lg ${getRandomColor()} text-white`}
                        >
                            {request.software_request.requester.name
                                .charAt(0)
                                .toUpperCase()}
                        </Avatar.Fallback>
                    </Avatar.Root>
                    <div class="grid gap-1">
                        <p class="text-lg font-medium leading-none">
                            {request.software_request.requester.name}
                        </p>
                        <p class="text-muted-foreground text-md">
                            {request.software_request.requester.email}
                        </p>
                        <p class="text-muted-foreground text-md">
                            Request#: {request.software_request.td_request_id}
                        </p>
                    </div>
                    <div class="ml-auto font-medium text-lg">
                        {formatDate(
                            request.software_request.requester.created_at,
                        )}
                    </div>
                </div>
            {/each}
        {:else}
            <p class="text-muted-foreground text-center mb-[450px] text-lg">
                No recent requests
            </p>
        {/if}
    </Card.Content>
</Card.Root>
