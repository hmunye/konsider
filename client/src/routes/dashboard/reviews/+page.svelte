<script lang="ts">
import { page } from "$app/stores";
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import Pagination from "$lib/components/custom/pagination/pagination.svelte";
import SearchBar from "$lib/components/custom/search-bar/search-bar.svelte";
import * as AlertDialog from "$lib/components/ui/alert-dialog";
import { Badge } from "$lib/components/ui/badge/index.js";
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as HoverCard from "$lib/components/ui/hover-card";
import * as Dialog from "$lib/components/ui/dialog";
import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
import * as Table from "$lib/components/ui/table/index.js";
import { fetchRequest } from "$lib/fetch";
import type { SoftwareReview } from "$lib/types/types";
import { formatDate } from "$lib/utils";
import Ellipsis from "lucide-svelte/icons/ellipsis";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import type { PageData } from "./$types";
import CreateSoftwareReviewForm from "$lib/components/forms/reviews/create/create-software-review-form.svelte";

let { data }: { data: PageData } = $props();

onMount(() => {
  if (data.error) {
    toast.error(data.error);
  }
});

let filter = $derived($page.url.searchParams.get("filter") || "");

const filterList = [
  { value: "td_request_id", label: "Request #" },
  { value: "reviewer_email", label: "Reviewer Email" },
  { value: "requester_email", label: "Requester Email" },
  { value: "software_name", label: "Software Name" },
  { value: "exported", label: "Exported" },
];

let submitting: boolean = $state(false);

let editDialogOpen: boolean = $state(false);
let deleteAlertOpen: boolean = $state(false);

let selectedReview: SoftwareReview | undefined = $state();

function handleDeleteSoftwareReview() {
  submitting = true;

  const deleteSoftwareReviewResponse = new Promise<unknown>(
    (resolve, reject) => {
      // Simulate a timeout before making the request to show loading toast
      setTimeout(() => {
        fetchRequest<unknown>({
          url: `${PUBLIC_BASE_API_URL}/api/v1/reviews/${selectedReview!.id}`,
          method: "DELETE",
        })
          .then((response) => {
            if (response.error) {
              reject(response.error.message);
            } else {
              resolve(response);
            }
          })
          .catch((error) => {
            reject(error);
          });
      }, 2000);
    },
  );

  toast.promise(deleteSoftwareReviewResponse, {
    loading: "Loading...",
    success: () => {
      submitting = false;
      return `Review ${selectedReview!.software_request.td_request_id} has been successfully deleted`;
    },
    error: (error) => {
      submitting = false;
      return `${error}`;
    },
  });
}

function handleExportSoftwareReview() {
  submitting = true;

  const exportSoftwareReviewResponse = new Promise<unknown>(
    (resolve, reject) => {
      // Simulate a timeout before making the request to show loading toast
      fetchRequest<unknown>({
        url: `${PUBLIC_BASE_API_URL}/api/v1/reviews/${selectedReview!.id}/export`,
        method: "GET",
      })
        .then((response) => {
          if (response.error) {
            reject(response.error.message);
          } else {
            resolve(response);
          }
        })
        .catch((error) => {
          reject(error);
        });
    },
  );

  toast.promise(exportSoftwareReviewResponse, {
    loading: "Loading...",
    success: () => {
      submitting = false;
      return `Review for ${selectedReview!.software_request.software.software_name} has been successfully exported`;
    },
    error: (error) => {
      submitting = false;
      return `${error}`;
    },
  });
}
</script>

<Card.Root class="animate-in">
    <Card.Header class="flex flex-row justify-between">
        <Card.Description class="text-xl hidden lg:flex"
            >Manage Software Reviews</Card.Description
        >
        <div class="flex gap-24 sm:gap-0">
            <SearchBar link={"reviews"} {filterList} />
            <Dialog.Root>
                <Dialog.Trigger>
                    <Button
                        variant="default"
                        class="text-lg mb-[115px] sm:mb-0"
                    >
                        <span class="hidden sm:inline"
                            >Create Software Review</span
                        >
                        <span class="inline md:hidden">Create</span>
                    </Button>
                </Dialog.Trigger>
                <Dialog.Content>
                    <CreateSoftwareReviewForm />
                </Dialog.Content>
            </Dialog.Root>
        </div>
    </Card.Header>
    <Card.Content>
        <Table.Root>
            <Table.Header>
                <Table.Row>
                    <Table.Head class="hidden md:table-cell"
                        >Reviewer Name</Table.Head
                    >
                    <Table.Head class="hidden sm:table-cell"
                        >Requester Name</Table.Head
                    >
                    <Table.Head>Software Name</Table.Head>
                    <Table.Head class="hidden sm:table-cell"
                        >Request #</Table.Head
                    >
                    <Table.Head>Exported</Table.Head>
                    <Table.Head class="hidden md:table-cell"
                        >Created At</Table.Head
                    >
                    <Table.Head>
                        <span class="sr-only">Actions</span>
                    </Table.Head>
                    <Table.Head>
                        <span class="sr-only">Hover For Details</span>
                    </Table.Head>
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {#if data.software_reviews?.software_reviews && data.software_reviews.software_reviews.length > 0}
                    {#each data.software_reviews.software_reviews as review}
                        <Table.Row>
                            <Table.Cell
                                class="font-medium text-lg hidden md:table-cell"
                                >{review.software_review.reviewer
                                    .name}</Table.Cell
                            >
                            <Table.Cell
                                class="font-medium text-lg hidden sm:table-cell"
                                >{review.software_review.software_request
                                    .requester.name}</Table.Cell
                            >
                            <Table.Cell class="font-medium text-lg"
                                >{review.software_review.software_request
                                    .software.software_name}</Table.Cell
                            >

                            <Table.Cell
                                class="font-medium text-lg hidden sm:table-cell"
                                >{review.software_review.software_request
                                    .td_request_id}</Table.Cell
                            >
                            <Table.Cell>
                                <Badge
                                    class="text-md"
                                    variant={review.software_review.exported ===
                                    true
                                        ? "default"
                                        : "secondary"}
                                >
                                    {review.software_review.exported}
                                </Badge>
                            </Table.Cell>
                            <Table.Cell class="hidden md:table-cell text-lg"
                                >{formatDate(
                                    review.software_review.created_at,
                                )}</Table.Cell
                            >
                            <Table.Cell>
                                <DropdownMenu.Root>
                                    <DropdownMenu.Trigger asChild let:builder>
                                        <Button
                                            builders={[builder]}
                                            aria-haspopup="true"
                                            size="icon"
                                            variant="ghost"
                                        >
                                            <Ellipsis class="h-4 w-4" />
                                            <span class="sr-only"
                                                >Toggle menu</span
                                            >
                                        </Button>
                                    </DropdownMenu.Trigger>
                                    <DropdownMenu.Content align="end">
                                        <DropdownMenu.Label class="text-lg"
                                            >Actions</DropdownMenu.Label
                                        >
                                        <DropdownMenu.Item
                                            class="text-md"
                                            onclick={() => {
                                                selectedReview =
                                                    review.software_review;
                                                editDialogOpen = true;
                                            }}>Edit</DropdownMenu.Item
                                        >
                                        <DropdownMenu.Item
                                            class="text-md"
                                            onclick={() => {
                                                selectedReview =
                                                    review.software_review;
                                                deleteAlertOpen = true;
                                            }}>Delete</DropdownMenu.Item
                                        >
                                        <DropdownMenu.Item
                                            class="text-md"
                                            onclick={() => {
                                                selectedReview =
                                                    review.software_review;
                                                handleExportSoftwareReview();
                                            }}>Export PDF</DropdownMenu.Item
                                        >
                                    </DropdownMenu.Content>
                                </DropdownMenu.Root>
                            </Table.Cell>
                            <Table.Cell>
                                <HoverCard.Root>
                                    <HoverCard.Trigger
                                        class="text-md hover:underline"
                                    >
                                        Hover For Details
                                    </HoverCard.Trigger>
                                    <HoverCard.Content>
                                        <div class="text-md space-y-4">
                                            <p>
                                                <a
                                                    href={`/dashboard/users?filter=name:${review.software_review.reviewer.name}`}
                                                    class="text-md font-semibold hover:underline"
                                                >
                                                    Go To Reviewer
                                                </a>

                                                <span class="mx-2">|</span>
                                                <a
                                                    href={`/dashboard/requests?filter=td_request_id:${review.software_review.software_request.td_request_id}`}
                                                    class="text-md font-semibold hover:underline"
                                                >
                                                    Go To Software Request
                                                </a>
                                            </p>
                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Is Supported:</span
                                                >
                                                <span>
                                                    {review.software_review.is_supported
                                                        .split("_")
                                                        .map(
                                                            (word: string) =>
                                                                word
                                                                    .charAt(0)
                                                                    .toUpperCase() +
                                                                word
                                                                    .slice(1)
                                                                    .toLowerCase(),
                                                        )
                                                        .join(" ")}
                                                </span>
                                            </p>

                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Is Current Version:</span
                                                >
                                                <span>
                                                    {review.software_review.is_current_version
                                                        .split("_")
                                                        .map(
                                                            (word: string) =>
                                                                word
                                                                    .charAt(0)
                                                                    .toUpperCase() +
                                                                word
                                                                    .slice(1)
                                                                    .toLowerCase(),
                                                        )
                                                        .join(" ")}
                                                </span>
                                            </p>

                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Is Reputation Good:</span
                                                >
                                                <span>
                                                    {review.software_review.is_reputation_good
                                                        .split("_")
                                                        .map(
                                                            (word: string) =>
                                                                word
                                                                    .charAt(0)
                                                                    .toUpperCase() +
                                                                word
                                                                    .slice(1)
                                                                    .toLowerCase(),
                                                        )
                                                        .join(" ")}
                                                </span>
                                            </p>

                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Is Installation from
                                                    Developer:</span
                                                >
                                                <span>
                                                    {review.software_review.is_installation_from_developer
                                                        .split("_")
                                                        .map(
                                                            (word: string) =>
                                                                word
                                                                    .charAt(0)
                                                                    .toUpperCase() +
                                                                word
                                                                    .slice(1)
                                                                    .toLowerCase(),
                                                        )
                                                        .join(" ")}
                                                </span>
                                            </p>

                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Is Local Admin Required:</span
                                                >
                                                <span>
                                                    {review.software_review.is_local_admin_required
                                                        .split("_")
                                                        .map(
                                                            (word: string) =>
                                                                word
                                                                    .charAt(0)
                                                                    .toUpperCase() +
                                                                word
                                                                    .slice(1)
                                                                    .toLowerCase(),
                                                        )
                                                        .join(" ")}
                                                </span>
                                            </p>
                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Connected to Brockport
                                                    Cloud:</span
                                                >
                                                <span>
                                                    {review.software_review.is_connected_to_brockport_cloud
                                                        .split("_")
                                                        .map(
                                                            (word: string) =>
                                                                word
                                                                    .charAt(0)
                                                                    .toUpperCase() +
                                                                word
                                                                    .slice(1)
                                                                    .toLowerCase(),
                                                        )
                                                        .join(" ")}
                                                </span>
                                            </p>

                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Connected to Cloud Services
                                                    or Client:</span
                                                >
                                                <span>
                                                    {review.software_review.is_connected_to_cloud_services_or_client
                                                        .split("_")
                                                        .map(
                                                            (word: string) =>
                                                                word
                                                                    .charAt(0)
                                                                    .toUpperCase() +
                                                                word
                                                                    .slice(1)
                                                                    .toLowerCase(),
                                                        )
                                                        .join(" ")}
                                                </span>
                                            </p>
                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Is Security or Optimization
                                                    Software:</span
                                                >
                                                <span>
                                                    {review.software_review.is_security_or_optimization_software
                                                        .split("_")
                                                        .map(
                                                            (word: string) =>
                                                                word
                                                                    .charAt(0)
                                                                    .toUpperCase() +
                                                                word
                                                                    .slice(1)
                                                                    .toLowerCase(),
                                                        )
                                                        .join(" ")}
                                                </span>
                                            </p>

                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Is Supported by Current OS:</span
                                                >
                                                <span>
                                                    {review.software_review.is_supported_by_current_os
                                                        .split("_")
                                                        .map(
                                                            (word: string) =>
                                                                word
                                                                    .charAt(0)
                                                                    .toUpperCase() +
                                                                word
                                                                    .slice(1)
                                                                    .toLowerCase(),
                                                        )
                                                        .join(" ")}
                                                </span>
                                            </p>
                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Created At:</span
                                                >
                                                <span
                                                    >{formatDate(
                                                        review.software_review
                                                            .created_at,
                                                    )}</span
                                                >
                                            </p>
                                            <p class="text-md">
                                                <span class="font-semibold"
                                                    >Review Notes:</span
                                                >
                                                <span
                                                    >{review.software_review
                                                        .review_notes}</span
                                                >
                                            </p>
                                        </div>
                                    </HoverCard.Content>
                                </HoverCard.Root>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                {:else}
                    <Table.Row>
                        <Table.Cell
                            colspan={7}
                            class="text-muted-foreground text-center text-lg"
                        >
                            No software reviews found
                        </Table.Cell>
                    </Table.Row>
                {/if}
            </Table.Body>
        </Table.Root>
    </Card.Content>
    <Card.Footer class="flex justify-between items-center">
        <Pagination data={data.software_reviews!} link={"reviews"} {filter} />
    </Card.Footer>

    <Dialog.Root bind:open={editDialogOpen}>
        <Dialog.Content>
            <EditSoftwareReviewForm {selectedReview} />
        </Dialog.Content>
    </Dialog.Root>

    <AlertDialog.Root bind:open={deleteAlertOpen}>
        <AlertDialog.Content>
            <AlertDialog.Header>
                <AlertDialog.Title class="text-xl"
                    >Are you sure?</AlertDialog.Title
                >
                <AlertDialog.Description class="text-lg">
                    This action cannot be undone. This will permanently delete
                    the software review from the server.
                </AlertDialog.Description>
            </AlertDialog.Header>
            <AlertDialog.Footer>
                <AlertDialog.Cancel class="text-md">Cancel</AlertDialog.Cancel>
                <AlertDialog.Action
                    class="text-md bg-destructive text-destructive-foreground hover:bg-destructive hover:brightness-125"
                    onclick={() => handleDeleteSoftwareReview()}
                    disabled={submitting}
                    aria-disabled={submitting}>Delete</AlertDialog.Action
                >
            </AlertDialog.Footer>
        </AlertDialog.Content>
    </AlertDialog.Root>
</Card.Root>
