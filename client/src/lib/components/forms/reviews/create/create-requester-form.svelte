<script lang="ts">
import * as Form from "$lib/components/ui/form";
import { Input } from "$lib/components/ui/input";

let { form, formData } = $props();
</script>

<form>
    <div class="flex flex-col gap-4 [&>input]:mb-4 mt-8 rounded-lg p-8 py-8">
        <h1 class="text-2xl font-bold mb-4">Requester Details</h1>

        <Form.Field {form} name="name">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Name</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.name}
                    type="text"
                    autocomplete="name"
                    placeholder="John"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="email">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Email</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.email}
                    type="email"
                    autocomplete="email"
                    placeholder="you@example.com"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="department">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Department</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.department}
                    type="text"
                    placeholder="IT"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="td_request_id">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Team Dynamix Request #</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.td_request_id}
                    type="text"
                    placeholder="12345678"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
    </div>
</form>
