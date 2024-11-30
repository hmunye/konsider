<script lang="ts">
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import * as Form from "$lib/components/ui/form";
import { Input } from "$lib/components/ui/input";
import * as Select from "$lib/components/ui/select";
import { fetchRequest } from "$lib/fetch";
import { toast } from "svelte-sonner";
import { defaults, superForm } from "sveltekit-superforms";
import { zod, zodClient } from "sveltekit-superforms/adapters";
import { editRequesterSchema } from "./schema";

let { selectedRequester } = $props();

let submitting: boolean = $state(false);

const initialData = {
  name: selectedRequester.name,
  email: selectedRequester.email,
  department: selectedRequester.department,
};

const form = superForm(defaults(initialData, zod(editRequesterSchema)), {
  validators: zodClient(editRequesterSchema),
  SPA: true,
  dataType: "json",
  resetForm: false,
  async onUpdate({ form }) {
    submitting = true;

    if (!form.valid) {
      return;
    }

    const requestBody: Record<string, string> = {};

    if ($formData.name !== initialData.name) {
      requestBody.name = $formData.name;
    }
    if ($formData.email !== initialData.email) {
      requestBody.email = $formData.email;
    }
    if ($formData.department !== initialData.department) {
      requestBody.department = $formData.department;
    }

    if (Object.keys(requestBody).length === 0) {
      submitting = false;
      toast.error("No values changed");
      return;
    }

    const editRequesterResponse = new Promise<unknown>((resolve, reject) => {
      // Simulate a timeout before making the request to show loading toast
      setTimeout(() => {
        fetchRequest<unknown>({
          url: `${PUBLIC_BASE_API_URL}/api/v1/requesters/${selectedRequester.id}`,
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

    toast.promise(editRequesterResponse, {
      loading: "Loading...",
      success: () => {
        submitting = false;
        return "Requester details have been successfully updated";
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
        <h1 class="text-2xl font-bold mb-4">Edit Requester</h1>

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
                    type="text"
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
                    placeholder="BITS"
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
