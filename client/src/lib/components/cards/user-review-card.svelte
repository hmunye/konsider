<script lang="ts">
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as Table from "$lib/components/ui/table/index.js";
import { formatDate } from "$lib/utils";
import ArrowUpRight from "lucide-svelte/icons/arrow-up-right";

let { userReviews = undefined } = $props();
</script>

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
                    <Table.Head class="text-lg">Request #</Table.Head>
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
                            <Table.Cell class="font-medium text-lg text-right"
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
