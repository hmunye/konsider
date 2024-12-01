<script lang="ts">
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import * as Form from "$lib/components/ui/form";
import { Input } from "$lib/components/ui/input";
import { fetchRequest } from "$lib/fetch";
import { toast } from "svelte-sonner";
import { defaults, superForm } from "sveltekit-superforms";
import { zod, zodClient } from "sveltekit-superforms/adapters";
import { editSoftwareRequestSchema } from "./schema";

let { selectedSoftwareRequest } = $props();

let submitting: boolean = $state(false);

const initialData = {
  td_request_id: selectedSoftwareRequest.td_request_id,
};

const form = superForm(defaults(initialData, zod(editSoftwareRequestSchema)), {
  validators: zodClient(editSoftwareRequestSchema),
  SPA: true,
  dataType: "json",
  resetForm: false,
  async onUpdate({ form }) {
    submitting = true;

    if (!form.valid) {
      submitting = false;
      return;
    }

    const requestBody: Record<string, string> = {};

    if ($formData.td_request_id !== initialData.td_request_id) {
      requestBody.td_request_id = $formData.td_request_id;
    }

    if (Object.keys(requestBody).length === 0) {
      submitting = false;
      toast.error("No values changed");
      return;
    }

    const editSoftwareRequestResponse = new Promise<unknown>(
      (resolve, reject) => {
        // Simulate a timeout before making the request to show loading toast
        setTimeout(() => {
          fetchRequest<unknown>({
            url: `${PUBLIC_BASE_API_URL}/api/v1/requests/${selectedSoftwareRequest.id}`,
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
      },
    );

    toast.promise(editSoftwareRequestResponse, {
      loading: "Loading...",
      success: () => {
        submitting = false;
        return "Software Request details have been successfully updated";
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
        <h1 class="text-2xl font-bold mb-4">Edit Software Request</h1>

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
        <Form.Button
            class="bg-success text-success-foreground text-lg hover:bg-success hover:brightness-125 transition duration-300"
            disabled={submitting}
            aria-disabled={submitting}
        >
            Update
        </Form.Button>
    </div>
</form>
