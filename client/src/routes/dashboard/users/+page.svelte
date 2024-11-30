<script lang="ts">
import { page } from "$app/stores";
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import Pagination from "$lib/components/custom/pagination/pagination.svelte";
import SearchBar from "$lib/components/custom/search-bar/search-bar.svelte";
import CreateUserForm from "$lib/components/forms/users/create/create-user-form.svelte";
import EditUserForm from "$lib/components/forms/users/edit/edit-user-form.svelte";
import * as AlertDialog from "$lib/components/ui/alert-dialog";
import * as Avatar from "$lib/components/ui/avatar/index.js";
import { Badge } from "$lib/components/ui/badge/index.js";
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as Dialog from "$lib/components/ui/dialog";
import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
import * as Table from "$lib/components/ui/table/index.js";
import { fetchRequest } from "$lib/fetch";
import { userStore } from "$lib/stores/userStore";
import type { User } from "$lib/types/types";
import { formatDate, getRandomColor } from "$lib/utils";
import Ellipsis from "lucide-svelte/icons/ellipsis";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import type { PageData } from "./$types";
import ChangeUserPasswordForm from "$lib/components/forms/users/change-password/change-user-password-form.svelte";

let { data }: { data: PageData } = $props();

onMount(() => {
  if (data.error) {
    toast.error(data.error);
  }
});

let filter = $derived($page.url.searchParams.get("filter") || "");

const filterList = [
  { value: "name", label: "Name" },
  { value: "email", label: "Email" },
  { value: "role", label: "Role" },
];

let submitting: boolean = $state(false);

let editDialogOpen: boolean = $state(false);
let passwordDialogOpen: boolean = $state(false);
let deleteAlertOpen: boolean = $state(false);
let revokeAlertOpen: boolean = $state(false);

let selectedUser: User | undefined = $state();

function handleDeleteUser() {
  submitting = true;

  const deleteUserResponse = new Promise<unknown>((resolve, reject) => {
    // Simulate a timeout before making the request to show loading toast
    setTimeout(() => {
      fetchRequest<unknown>({
        url: `${PUBLIC_BASE_API_URL}/api/v1/users/${selectedUser!.id}`,
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

  toast.promise(deleteUserResponse, {
    loading: "Loading...",
    success: () => {
      submitting = false;
      return `${selectedUser!.name}'s account has been successfully deleted`;
    },
    error: (error) => {
      submitting = false;
      return `${error}`;
    },
  });
}

function handleRevokeUserToken() {
  submitting = true;

  const revokeUserTokenResponse = new Promise<unknown>((resolve, reject) => {
    // Simulate a timeout before making the request to show loading toast
    setTimeout(() => {
      fetchRequest<unknown>({
        url: `${PUBLIC_BASE_API_URL}/api/v1/auth/revoke/${selectedUser!.id}`,
        method: "DELETE",
      })
        .then((response) => {
          if (response.error) {
            reject("This user has no current active sessions");
          } else {
            resolve(response);
          }
        })
        .catch((error) => {
          reject(error);
        });
    }, 2000);
  });

  toast.promise(revokeUserTokenResponse, {
    loading: "Loading...",
    success: () => {
      submitting = false;
      return `${selectedUser!.name}'s session has been revoked`;
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
            >Manage Users</Card.Description
        >
        <div class="flex gap-24 sm:gap-0">
            <SearchBar link={"users"} {filterList} />
            <Dialog.Root>
                <Dialog.Trigger>
                    <Button variant="default" class="text-lg">
                        Create User
                    </Button>
                </Dialog.Trigger>
                <Dialog.Content>
                    <CreateUserForm />
                </Dialog.Content>
            </Dialog.Root>
        </div>
    </Card.Header>
    <Card.Content>
        <Table.Root>
            <Table.Header>
                <Table.Row>
                    <Table.Head class="hidden w-[100px] sm:table-cell">
                        <span class="sr-only">Image</span>
                    </Table.Head>
                    <Table.Head>Name</Table.Head>
                    <Table.Head>Role</Table.Head>
                    <Table.Head class="hidden md:table-cell">Email</Table.Head>
                    <Table.Head class="hidden md:table-cell"
                        >Created At</Table.Head
                    >
                    <Table.Head>
                        <span class="sr-only">Actions</span>
                    </Table.Head>
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {#if data.users?.users && data.users.users.length > 0}
                    {#each data.users.users as user}
                        <Table.Row>
                            <Table.Cell class="hidden sm:table-cell">
                                <Avatar.Root class="hidden h-9 w-9 sm:flex">
                                    <Avatar.Fallback
                                        class={`text-lg ${getRandomColor()} text-white`}
                                        >{user.user.name
                                            .charAt(0)
                                            .toUpperCase()}</Avatar.Fallback
                                    >
                                </Avatar.Root>
                            </Table.Cell>
                            <Table.Cell class="font-medium text-lg"
                                >{user.user.name}</Table.Cell
                            >
                            <Table.Cell>
                                <Badge
                                    class="text-md"
                                    variant={user.user.role === "ADMIN"
                                        ? "default"
                                        : "secondary"}
                                >
                                    {user.user.role.charAt(0).toUpperCase() +
                                        user.user.role.slice(1)}
                                </Badge>
                            </Table.Cell>
                            <Table.Cell class="hidden md:table-cell text-lg"
                                >{user.user.email}</Table.Cell
                            >
                            <Table.Cell class="hidden md:table-cell text-lg"
                                >{formatDate(user.user.created_at)}</Table.Cell
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
                                                selectedUser = user.user;
                                                editDialogOpen = true;
                                            }}>Edit</DropdownMenu.Item
                                        >
                                        <DropdownMenu.Item
                                            class="text-md"
                                            onclick={() => {
                                                selectedUser = user.user;
                                                deleteAlertOpen = true;
                                            }}>Delete</DropdownMenu.Item
                                        >
                                        {#if user.user.email !== $userStore?.email && $userStore?.role === "ADMIN"}
                                            <DropdownMenu.Item
                                                class="text-md"
                                                onclick={() => {
                                                    selectedUser = user.user;
                                                    revokeAlertOpen = true;
                                                }}
                                                >Revoke Session</DropdownMenu.Item
                                            >
                                        {/if}
                                        {#if user.user.email === $userStore?.email}
                                            <DropdownMenu.Item
                                                class="text-md"
                                                onclick={() => {
                                                    passwordDialogOpen = true;
                                                }}
                                                >Change Password</DropdownMenu.Item
                                            >
                                        {/if}
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
                            No users found
                        </Table.Cell>
                    </Table.Row>
                {/if}
            </Table.Body>
        </Table.Root>
    </Card.Content>
    <Card.Footer class="flex justify-between items-center">
        <Pagination data={data.users!} link={"users"} {filter} />
    </Card.Footer>

    <Dialog.Root bind:open={editDialogOpen}>
        <Dialog.Content>
            <EditUserForm {selectedUser} />
        </Dialog.Content>
    </Dialog.Root>

    <Dialog.Root bind:open={passwordDialogOpen}>
        <Dialog.Content>
            <ChangeUserPasswordForm />
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
                    the user's account and remove their data from the server.
                </AlertDialog.Description>
            </AlertDialog.Header>
            <AlertDialog.Footer>
                <AlertDialog.Cancel class="text-md">Cancel</AlertDialog.Cancel>
                <AlertDialog.Action
                    class="text-md bg-destructive text-destructive-foreground hover:bg-destructive hover:brightness-125"
                    onclick={() => handleDeleteUser()}
                    disabled={submitting}
                    aria-disabled={submitting}>Delete</AlertDialog.Action
                >
            </AlertDialog.Footer>
        </AlertDialog.Content>
    </AlertDialog.Root>

    <AlertDialog.Root bind:open={revokeAlertOpen}>
        <AlertDialog.Content>
            <AlertDialog.Header>
                <AlertDialog.Title class="text-xl"
                    >Are you sure?</AlertDialog.Title
                >
                <AlertDialog.Description class="text-lg">
                    Revoking this user's session will log them out immediately.
                    They will need to log in again to access the system
                </AlertDialog.Description>
            </AlertDialog.Header>
            <AlertDialog.Footer>
                <AlertDialog.Cancel class="text-md">Cancel</AlertDialog.Cancel>
                <AlertDialog.Action
                    class="text-md bg-destructive text-destructive-foreground hover:bg-destructive hover:brightness-125"
                    onclick={() => handleRevokeUserToken()}
                    disabled={submitting}
                    aria-disabled={submitting}>Revoke</AlertDialog.Action
                >
            </AlertDialog.Footer>
        </AlertDialog.Content>
    </AlertDialog.Root>
</Card.Root>
