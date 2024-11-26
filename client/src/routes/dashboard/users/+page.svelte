<script lang="ts">
import * as Avatar from "$lib/components/ui/avatar/index.js";
import { Badge } from "$lib/components/ui/badge/index.js";
import { Button } from "$lib/components/ui/button/index.js";
import * as Card from "$lib/components/ui/card/index.js";
import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
import * as Table from "$lib/components/ui/table/index.js";
import Ellipsis from "lucide-svelte/icons/ellipsis";
import type { PageData } from "./$types";
import { toast } from "svelte-sonner";
import { formatDate } from "$lib/utils";
import { goto } from "$app/navigation";

let { data }: { data: PageData } = $props();

if (data.error) {
  toast.error(data.error);
}
</script>

<Card.Root class="animate-in">
    <Card.Header class="flex flex-row justify-between">
        <Card.Description class="text-xl">Manage Users</Card.Description>
        <div class="flex gap-2">
            <Button variant="default" class="text-lg">Create User</Button>
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
                        >Joined At</Table.Head
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
                                        class="text-lg bg-purple text-purple-foreground"
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
                                    </DropdownMenu.Content>
                                </DropdownMenu.Root>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                {:else}
                    <Table.Row>
                        <Table.Cell
                            colspan={6}
                            class="text-center text-muted-foreground text-lg"
                        >
                            No users found
                        </Table.Cell>
                    </Table.Row>
                {/if}
            </Table.Body>
        </Table.Root>
    </Card.Content>
    <Card.Footer class="flex justify-between items-center">
        <div class="text-muted-foreground text-md">
            Showing Page <strong
                >{data.users?.metadata.current_page} of {data.users?.metadata
                    .last_page}</strong
            >
        </div>
        <div class="flex gap-2">
            <Button
                class="text-md"
                variant={"ghost"}
                on:click={() =>
                    goto(
                        `/admin/users?per_page=8&page=${data.users!.metadata.current_page - 1}`,
                    )}
                disabled={data.users?.metadata.current_page === 1}
            >
                Previous
            </Button>
            <Button
                class="text-md"
                variant={"ghost"}
                on:click={() =>
                    goto(
                        `/admin/users?per_page=8&page=${data.users!.metadata.current_page + 1}`,
                    )}
                disabled={data.users?.metadata.current_page ===
                    data.users?.metadata.last_page}
            >
                Next
            </Button>
        </div>
    </Card.Footer>
</Card.Root>
