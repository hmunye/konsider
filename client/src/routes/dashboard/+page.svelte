<script lang="ts">
import * as Card from "$lib/components/ui/card/index.js";
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
import UserReviewCard from "$lib/components/cards/user-review-card.svelte";
import RecentRequestsCard from "$lib/components/cards/recent-requests-card.svelte";

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
    <UserReviewCard {userReviews} />
    <RecentRequestsCard softwareRequests={data.software_requests} />
</div>
