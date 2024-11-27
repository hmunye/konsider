<script>
import { Input } from "$lib/components/ui/input";
import Search from "lucide-svelte/icons/search";
</script>

<form class="ml-auto flex-1 sm:flex-initial">
    <div class="relative">
        <Search
            class="text-muted-foreground absolute left-2.5 top-2.5 h-4 w-4"
        />
        <Input
            type="search"
            name="search"
            placeholder="Search..."
            class="pl-8 sm:w-[300px] md:w-[200px] lg:w-[300px] text-lg"
        />
    </div>
</form>
