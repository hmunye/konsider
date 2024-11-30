<script lang="ts">
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import * as Form from "$lib/components/ui/form";
import { Input } from "$lib/components/ui/input";
import { fetchRequest } from "$lib/fetch";
import { toast } from "svelte-sonner";
import { defaults, superForm } from "sveltekit-superforms";
import { zod, zodClient } from "sveltekit-superforms/adapters";
import { editSoftwareSchema } from "./schema";

let { selectedSoftware } = $props();

let submitting: boolean = $state(false);

const initialData = {
  software_name: selectedSoftware.software_name,
  software_version: selectedSoftware.software_version,
  developer_name: selectedSoftware.developer_name,
  description: selectedSoftware.description,
};

const form = superForm(defaults(initialData, zod(editSoftwareSchema)), {
  validators: zodClient(editSoftwareSchema),
  SPA: true,
  dataType: "json",
  resetForm: false,
  async onUpdate({ form }) {
    submitting = true;

    if (!form.valid) {
      return;
    }

    const requestBody: Record<string, string> = {};

    if ($formData.software_name !== initialData.software_name) {
      requestBody.software_name = $formData.software_name;
    }
    if ($formData.software_version !== initialData.software_version) {
      requestBody.software_version = $formData.software_version;
    }
    if ($formData.developer_name !== initialData.developer_name) {
      requestBody.developer_name = $formData.developer_name;
    }
    if ($formData.description !== initialData.description) {
      requestBody.description = $formData.description;
    }

    if (Object.keys(requestBody).length === 0) {
      submitting = false;
      toast.error("No values changed");
      return;
    }

    const editSoftwareResponse = new Promise<unknown>((resolve, reject) => {
      // Simulate a timeout before making the request to show loading toast
      setTimeout(() => {
        fetchRequest<unknown>({
          url: `${PUBLIC_BASE_API_URL}/api/v1/software/${selectedSoftware.id}`,
          method: "PATCH",
          requestBody,
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

    toast.promise(editSoftwareResponse, {
      loading: "Loading...",
      success: () => {
        submitting = false;
        return "Software details have been successfully updated";
      },
      error: (error) => {
        submitting = false;
        return `${error}`;
      },
    });
  },
});

const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance>
    <div class="flex flex-col gap-8 [&>input]:mb-4 mt-8 rounded-lg p-8 py-8">
        <h1 class="text-2xl font-bold mb-4">Edit Software Details</h1>

        <Form.Field {form} name="software_name">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Software Name</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.software_name}
                    type="text"
                    autocomplete="name"
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
                <Input
                    {...attrs}
                    bind:value={$formData.description}
                    type="text"
                    placeholder=""
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Button
            class="bg-success text-success-foreground text-lg hover:bg-success hover:brightness-125 transition duration-300"
            disabled={submitting}
            aria-disabled={submitting}
        >
            Update
        </Form.Button>
    </div>
</form>
