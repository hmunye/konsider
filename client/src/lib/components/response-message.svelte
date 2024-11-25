<script lang="ts">
    import type { Message } from "$lib/types/types";
    import { cn } from "$lib/utils";

    interface Props {
        message: Message;
        class?: string;
    }

    let { message, class: className = "" }: Props = $props();
</script>

<div class={cn("flex flex-col gap-2 w-full max-w-md text-sm", className)}>
    {#if "success" in message}
        <div class="text-success border-l-2 border-primary px-4">
            {message.success}
        </div>
    {:else if "error" in message}
        <div class="text-destructive border-l-2 border-destructive px-4">
            {message.error}
        </div>
    {:else if "message" in message}
        <div class="text-foreground border-l-2 px-4">
            {message.message}
        </div>
    {/if}
</div>
