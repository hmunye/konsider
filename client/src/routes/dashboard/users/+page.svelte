<script lang="ts">
import { page } from "$app/stores";
import Pagination from "$lib/components/custom/pagination/pagination.svelte";
import SearchBar from "$lib/components/custom/search-bar/search-bar.svelte";
import CreateUserForm from "$lib/components/forms/users/create/create-user-form.svelte";
import * as Avatar from "$lib/components/ui/avatar/index.js";
import { Badge } from "$lib/components/ui/badge/index.js";
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as Dialog from "$lib/components/ui/dialog";
import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
import * as Table from "$lib/components/ui/table/index.js";
import { userStore } from "$lib/stores/userStore";
import { formatDate, getRandomColor } from "$lib/utils";
import Ellipsis from "lucide-svelte/icons/ellipsis";
import type { PageData } from "./$types";
import { toast } from "svelte-sonner";
import { onMount } from "svelte";

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
                    <CreateUserForm data={data.form} />
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
                                        <DropdownMenu.Item class="text-md"
                                            >Edit</DropdownMenu.Item
                                        >
                                        <DropdownMenu.Item class="text-md"
                                            >Delete</DropdownMenu.Item
                                        >
                                        {#if user.user.email === $userStore?.email}
                                            <DropdownMenu.Item class="text-md"
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
</Card.Root>
