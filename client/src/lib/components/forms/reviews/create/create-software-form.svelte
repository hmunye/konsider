<script lang="ts">
import * as Form from "$lib/components/ui/form";
import { Input } from "$lib/components/ui/input";
import { Textarea } from "$lib/components/ui/textarea";

let { form, formData } = $props();
</script>

<form>
    <div class="flex flex-col gap-8 [&>input]:mb-4 mt-8 rounded-lg p-8 py-8">
        <h1 class="text-2xl font-bold mb-4">Software Details</h1>

        <Form.Field {form} name="software_name">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Software Name</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.software_name}
                    type="text"
                    placeholder="Zoom"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="software_version">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Software Version</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.software_version}
                    type="text"
                    placeholder="1.0.0"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="developer_name">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Developer Name</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.developer_name}
                    type="text"
                    placeholder="Zoom Communications"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="description">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Description</Form.Label>
                <Textarea
                    {...attrs}
                    bind:value={$formData.description}
                    placeholder="Type your message here."
                    class="resize-none text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
    </div>
</form>
