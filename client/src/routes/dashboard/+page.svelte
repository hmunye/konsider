<script lang="ts">
import * as Avatar from "$lib/components/ui/avatar/index.js";
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as Table from "$lib/components/ui/table/index.js";
import ArrowUpRight from "lucide-svelte/icons/arrow-up-right";
import FolderCode from "lucide-svelte/icons/folder-code";
import SquareChartGantt from "lucide-svelte/icons/square-chart-gantt";
import Tags from "lucide-svelte/icons/tags";
import UsersRound from "lucide-svelte/icons/users-round";
import UserRoundPen from "lucide-svelte/icons/user-round-pen";
import type { PageData } from "./$types";
import { userStore } from "$lib/stores/userStore";
import { onMount } from "svelte";
import { fetchRequest } from "$lib/fetch";
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import type { SoftwareReviewResponse } from "$lib/types/types";
import { toast } from "svelte-sonner";
import { formatDate, getRandomColor } from "$lib/utils";

let { data }: { data: PageData } = $props();

if (data.error) {
  toast.error(data.error.message);
}

let userReviews: SoftwareReviewResponse | undefined = $state();

onMount(async () => {
  if ($userStore?.email) {
    const response = await fetchRequest<SoftwareReviewResponse>({
      url: `${PUBLIC_BASE_API_URL}/api/v1/reviews?filter=reviewer_email:${$userStore.email}`,
      method: "GET",
    });

    if (response.error) {
      toast.error(
        response.error.message ?? "Error occured fetching your reviews",
      );
    } else {
      userReviews = response.success!;
    }
  }
});
</script>

<div class="animate-in grid gap-4 md:grid-cols-2 md:gap-8 xl:grid-cols-5">
    <Card.Root
        data-x-chunk-name="dashboard-01-chunk-0"
        data-x-chunk-description="A card showing the total number of registered users."
    >
        <Card.Header
            class="flex flex-row items-center justify-between space-y-0 pb-2"
        >
            <Card.Title class="text-3xl font-bold">Total Users</Card.Title>
            <UsersRound class="text-muted-foreground h-6 w-6" />
        </Card.Header>
        <Card.Content>
            <div class="text-2xl font-bold">
                {data.users?.metadata.total_records ?? 0}
                <span class="text-muted-foreground text-lg"> Registered</span>
            </div>
        </Card.Content>
    </Card.Root>
    <Card.Root
        data-x-chunk-name="dashboard-01-chunk-0"
        data-x-chunk-description="A card showing the total number of registered users."
    >
        <Card.Header
            class="flex flex-row items-center justify-between space-y-0 pb-2"
        >
            <Card.Title class="text-3xl font-bold">Total Requesters</Card.Title>
            <UserRoundPen class="text-muted-foreground h-6 w-6" />
        </Card.Header>
        <Card.Content>
            <div class="text-2xl font-bold">
                {data.requesters?.metadata.total_records ?? 0}
                <span class="text-muted-foreground text-lg"> Records</span>
            </div>
        </Card.Content>
    </Card.Root>
    <Card.Root
        data-x-chunk-name="dashboard-01-chunk-1"
        data-x-chunk-description="A card showing the total number of software records."
    >
        <Card.Header
            class="flex flex-row items-center justify-between space-y-0 pb-2"
        >
            <Card.Title class="text-3xl font-bold">Total Software</Card.Title>
            <FolderCode class="text-muted-foreground h-6 w-6" />
        </Card.Header>
        <Card.Content>
            <div class="text-2xl font-bold">
                {data.software?.metadata.total_records ?? 0}
                <span class="text-muted-foreground text-lg"> Records</span>
            </div>
        </Card.Content>
    </Card.Root>
    <Card.Root
        data-x-chunk-name="dashboard-01-chunk-2"
        data-x-chunk-description="A card showing the total number of requests created."
    >
        <Card.Header
            class="flex flex-row items-center justify-between space-y-0 pb-2"
        >
            <Card.Title class="text-3xl font-bold">Total Requests</Card.Title>
            <Tags class="text-muted-foreground h-6 w-6" />
        </Card.Header>
        <Card.Content>
            <div class="text-2xl font-bold">
                {data.software_requests?.metadata.total_records ?? 0}
                <span class="text-muted-foreground text-lg"> Made</span>
            </div>
        </Card.Content>
    </Card.Root>
    <Card.Root
        data-x-chunk-name="dashboard-01-chunk-3"
        data-x-chunk-description="A card showing the total number of software reviews created."
    >
        <Card.Header
            class="flex flex-row items-center justify-between space-y-0 pb-2"
        >
            <Card.Title class="text-3xl font-bold">Total Reviews</Card.Title>
            <SquareChartGantt class="text-muted-foreground h-6 w-6" />
        </Card.Header>
        <Card.Content>
            <div class="text-2xl font-bold">
                {data.software_reviews?.metadata.total_records ?? 0}
                <span class="text-muted-foreground text-lg"> Created</span>
            </div>
        </Card.Content>
    </Card.Root>
</div>
<div class="animate-in grid gap-4 md:gap-8 lg:grid-cols-2 xl:grid-cols-3">
    <Card.Root
        class="xl:col-span-2"
        data-x-chunk-name="dashboard-01-chunk-4"
        data-x-chunk-description="A card showing a table of reviews created by the current user."
    >
        <Card.Header class="flex flex-row items-center">
            <div class="grid gap-2">
                <Card.Title class="text-2xl">Your Reviews</Card.Title>
            </div>
            <Button
                href="/dashboard/reviews"
                size="sm"
                class="ml-auto gap-1 text-md"
            >
                View All Reviews
                <ArrowUpRight class="h-4 w-4" />
            </Button>
        </Card.Header>
        <Card.Content>
            <Table.Root>
                <Table.Header>
                    <Table.Row class="justify-center">
                        <Table.Head class="text-lg">Reviewer</Table.Head>
                        <Table.Head class="hidden md:table-cell text-lg"
                            >Software</Table.Head
                        >
                        <Table.Head class="text-lg">Request ID</Table.Head>
                        <Table.Head class="text-lg text-right"
                            >Date Created</Table.Head
                        >
                    </Table.Row>
                </Table.Header>
                <Table.Body>
                    {#if userReviews && userReviews.software_reviews.length > 0}
                        {#each userReviews.software_reviews as review}
                            <Table.Row>
                                <Table.Cell>
                                    <div class="font-medium text-lg">
                                        {review.software_review.reviewer.name}
                                    </div>
                                </Table.Cell>
                                <Table.Cell class="hidden md:table-cell">
                                    <div class="font-medium text-lg">
                                        {review.software_review.software_request
                                            .software.software_name}
                                    </div>
                                </Table.Cell>
                                <Table.Cell class="font-medium text-lg">
                                    {review.software_review.software_request
                                        .td_request_id}
                                </Table.Cell>
                                <Table.Cell
                                    class="font-medium text-lg text-right"
                                    >{formatDate(
                                        review.software_review.created_at,
                                    )}</Table.Cell
                                >
                            </Table.Row>
                        {/each}
                    {:else}
                        <Table.Row>
                            <Table.Cell
                                colspan={4}
                                class="text-muted-foreground text-center mb-[450px] text-lg"
                            >
                                You have no reviews
                            </Table.Cell>
                        </Table.Row>
                    {/if}
                </Table.Body>
            </Table.Root>
        </Card.Content>
    </Card.Root>
    <Card.Root
        data-x-chunk-name="dashboard-01-chunk-5"
        data-x-chunk-description="A card showing a list of recent requests with requester names and details."
    >
        <Card.Header>
            <Card.Title class="text-2xl">Recent Requests</Card.Title>
        </Card.Header>
        <Card.Content class="grid gap-8">
            {#if data.software_requests && data.software_requests.software_requests.length > 0}
                {#each data.software_requests.software_requests as request}
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
                                Request#: {request.software_request
                                    .td_request_id}
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
</div>
