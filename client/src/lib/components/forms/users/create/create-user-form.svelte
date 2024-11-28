<script lang="ts">
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import * as Form from "$lib/components/ui/form";
import { Input } from "$lib/components/ui/input";
import * as Select from "$lib/components/ui/select";
import { fetchRequest } from "$lib/fetch";
import { toast } from "svelte-sonner";
import { defaults, superForm } from "sveltekit-superforms";
import { zod, zodClient } from "sveltekit-superforms/adapters";
import { createUserSchema } from "./schema";

let submitting: boolean = $state(false);

const initialData = {
  name: "",
  email: "",
  password: "",
  role: "",
};

const form = superForm(defaults(initialData, zod(createUserSchema)), {
  validators: zodClient(createUserSchema),
  SPA: true,
  dataType: "json",
  resetForm: false,
  async onUpdate({ form }) {
    submitting = true;

    if (!form.valid) {
      return;
    }

    const createUserResponse = new Promise<unknown>((resolve, reject) => {
      // Simulate a timeout before making the request to show loading toast
      setTimeout(() => {
        fetchRequest<unknown>({
          url: `${PUBLIC_BASE_API_URL}/api/v1/users`,
          method: "POST",
          requestBody: {
            name: $formData.name,
            email: $formData.email,
            password: $formData.password,
            role: $formData.role,
          },
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

    toast.promise(createUserResponse, {
      loading: "Loading...",
      success: () => {
        submitting = false;
        return `${$formData.name} has been successfully registered`;
      },
      error: (error) => {
        submitting = false;
        return `${error}`;
      },
    });
  },
});

const { form: formData, enhance } = form;

let selectedRole = $derived(
  $formData.role
    ? {
        label: $formData.role,
        value: $formData.role,
      }
    : undefined,
);
</script>

<form method="POST" use:enhance>
    <div class="flex flex-col gap-8 [&>input]:mb-4 mt-8 rounded-lg p-8 py-8">
        <h1 class="text-4xl font-bold mb-4">Create New User</h1>

        <Form.Field {form} name="name">
            <Form.Control let:attrs>
                <Form.Label class="text-2xl">Name</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.name}
                    type="text"
                    autocomplete="name"
                    placeholder="John"
                    class="text-xl placeholder:text-xl placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="email">
            <Form.Control let:attrs>
                <Form.Label class="text-2xl">Email</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.email}
                    type="text"
                    autocomplete="email"
                    placeholder="you@example.com"
                    class="text-xl placeholder:text-xl placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="password">
            <Form.Control let:attrs>
                <Form.Label class="text-2xl">Password</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.password}
                    type="password"
                    placeholder="••••••••"
                    class="text-xl placeholder:text-xl placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="role">
            <Form.Control let:attrs>
                <Form.Label class="text-2xl">Role</Form.Label>
                <Select.Root
                    selected={selectedRole}
                    onSelectedChange={(v) => {
                        v && ($formData.role = v.value);
                    }}
                >
                    <Select.Trigger {...attrs}>
                        <Select.Value
                            placeholder="Select User Role"
                            class="text-xl placeholder:text-xl placeholder:font-light"
                        />
                    </Select.Trigger>
                    <Select.Content>
                        <Select.Item
                            value="ADMIN"
                            label="ADMIN"
                            class="text-md"
                        />
                        <Select.Item
                            value="REVIEWER"
                            label="REVIEWER"
                            class="text-md"
                        />
                    </Select.Content>
                </Select.Root>
                <input hidden bind:value={$formData.role} name={attrs.name} />
            </Form.Control>
            <Form.FieldErrors />
        </Form.Field>
        <Form.Button
            class="bg-success text-success-foreground text-xl hover:bg-success hover:brightness-125 transition duration-300"
            disabled={submitting}
            aria-disabled={submitting}
        >
            Create
        </Form.Button>
    </div>
</form>
