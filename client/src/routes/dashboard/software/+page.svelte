<script lang="ts">
import { page } from "$app/stores";
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import Pagination from "$lib/components/custom/pagination/pagination.svelte";
import SearchBar from "$lib/components/custom/search-bar/search-bar.svelte";
import * as AlertDialog from "$lib/components/ui/alert-dialog";
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as Dialog from "$lib/components/ui/dialog";
import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
import * as Table from "$lib/components/ui/table/index.js";
import { fetchRequest } from "$lib/fetch";
import { formatDate } from "$lib/utils";
import Ellipsis from "lucide-svelte/icons/ellipsis";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import type { PageData } from "./$types";
import type { Software } from "$lib/types/types";
import EditSoftwareForm from "$lib/components/forms/software/edit/edit-software-form.svelte";

let { data }: { data: PageData } = $props();

onMount(() => {
  if (data.error) {
    toast.error(data.error);
  }
});

let filter = $derived($page.url.searchParams.get("filter") || "");

const filterList = [
  { value: "software_name", label: "Software Name" },
  { value: "developer_name", label: "Developer Name" },
];

let submitting: boolean = $state(false);

let editDialogOpen: boolean = $state(false);
let deleteAlertOpen: boolean = $state(false);

let selectedSoftware: Software | undefined = $state();

function handleDeleteSoftware() {
  submitting = true;

  const deleteSoftwareResponse = new Promise<unknown>((resolve, reject) => {
    // Simulate a timeout before making the request to show loading toast
    setTimeout(() => {
      fetchRequest<unknown>({
        url: `${PUBLIC_BASE_API_URL}/api/v1/software/${selectedSoftware!.id}`,
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
  });

  toast.promise(deleteSoftwareResponse, {
    loading: "Loading...",
    success: () => {
      submitting = false;
      return `${selectedSoftware?.software_name} record has been successfully deleted`;
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
        <Card.Description class="text-xl hidden md:flex"
            >Manage Software</Card.Description
        >
        <div class="flex">
            <SearchBar link={"software"} {filterList} />
        </div>
    </Card.Header>
    <Card.Content>
        <Table.Root>
            <Table.Header>
                <Table.Row>
                    <Table.Head>Software Name</Table.Head>
                    <Table.Head>Software Version</Table.Head>
                    <Table.Head class="hidden md:table-cell"
                        >Developer Name</Table.Head
                    >
                    <Table.Head class="hidden md:table-cell"
                        >Description</Table.Head
                    >
                    <Table.Head class="hidden md:table-cell"
                        >Created At</Table.Head
                    >
                    <Table.Head>
                        <span class="sr-only">Actions</span>
                    </Table.Head>
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {#if data.software?.software && data.software.software.length > 0}
                    {#each data.software.software as software}
                        <Table.Row>
                            <Table.Cell class="font-medium text-lg"
                                >{software.software.software_name}</Table.Cell
                            >
                            <Table.Cell class="font-medium text-lg"
                                >{software.software
                                    .software_version}</Table.Cell
                            >
                            <Table.Cell class="hidden md:table-cell text-lg"
                                >{software.software.developer_name}</Table.Cell
                            >
                            <Table.Cell class="hidden md:table-cell text-lg"
                                >{software.software.description}</Table.Cell
                            >
                            <Table.Cell class="hidden md:table-cell text-lg"
                                >{formatDate(
                                    software.software.created_at,
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
                                                selectedSoftware =
                                                    software.software;
                                                editDialogOpen = true;
                                            }}>Edit</DropdownMenu.Item
                                        >
                                        <DropdownMenu.Item
                                            class="text-md"
                                            onclick={() => {
                                                selectedSoftware =
                                                    software.software;
                                                deleteAlertOpen = true;
                                            }}>Delete</DropdownMenu.Item
                                        >
                                    </DropdownMenu.Content>
                                </DropdownMenu.Root>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                {:else}
                    <Table.Row>
                        <Table.Cell
                            colspan={6}
                            class="text-muted-foreground text-center text-lg"
                        >
                            No software records found
                        </Table.Cell>
                    </Table.Row>
                {/if}
            </Table.Body>
        </Table.Root>
    </Card.Content>
    <Card.Footer class="flex justify-between items-center">
        <Pagination data={data.software!} link={"software"} {filter} />
    </Card.Footer>

    <Dialog.Root bind:open={editDialogOpen}>
        <Dialog.Content>
            <EditSoftwareForm {selectedSoftware} />
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
                    the software record from the server.
                </AlertDialog.Description>
            </AlertDialog.Header>
            <AlertDialog.Footer>
                <AlertDialog.Cancel class="text-md">Cancel</AlertDialog.Cancel>
                <AlertDialog.Action
                    class="text-md bg-destructive text-destructive-foreground hover:bg-destructive hover:brightness-125"
                    onclick={() => handleDeleteSoftware()}
                    disabled={submitting}
                    aria-disabled={submitting}>Delete</AlertDialog.Action
                >
            </AlertDialog.Footer>
        </AlertDialog.Content>
    </AlertDialog.Root>
</Card.Root>
