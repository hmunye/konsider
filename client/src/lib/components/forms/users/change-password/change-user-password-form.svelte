<script lang="ts">
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import * as Form from "$lib/components/ui/form";
import { Input } from "$lib/components/ui/input";
import { fetchRequest } from "$lib/fetch";
import { toast } from "svelte-sonner";
import { defaults, superForm } from "sveltekit-superforms";
import { zod, zodClient } from "sveltekit-superforms/adapters";
import { changeUserPasswordSchema } from "./schema";
import { goto } from "$app/navigation";

let submitting: boolean = $state(false);

const initialData = {
  current_password: "",
  new_password: "",
  confirm_password: "",
};

const form = superForm(defaults(initialData, zod(changeUserPasswordSchema)), {
  validators: zodClient(changeUserPasswordSchema),
  SPA: true,
  dataType: "json",
  resetForm: false,
  async onUpdate({ form }) {
    submitting = true;

    if (!form.valid) {
      submitting = false;
      return;
    }

    const changeUserPasswordResponse = new Promise<unknown>(
      (resolve, reject) => {
        // Simulate a timeout before making the request to show loading toast
        setTimeout(() => {
          fetchRequest<unknown>({
            url: `${PUBLIC_BASE_API_URL}/api/v1/users/password`,
            method: "POST",
            requestBody: {
              current_password: $formData.current_password,
              new_password: $formData.new_password,
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
      },
    );

    toast.promise(changeUserPasswordResponse, {
      loading: "Loading...",
      success: () => {
        submitting = false;

        setTimeout(() => {
          goto("/");
        }, 3000);

        return "Password has been successfully updated";
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
        <h1 class="text-2xl font-bold mb-4">Change Password</h1>

        <Form.Field {form} name="current_password">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Current Password</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.current_password}
                    type="password"
                    placeholder="••••••••"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="new_password">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">New Password</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.new_password}
                    type="password"
                    placeholder="••••••••"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Form.Control>
            <Form.FieldErrors class="text-lg" />
        </Form.Field>
        <Form.Field {form} name="confirm_password">
            <Form.Control let:attrs>
                <Form.Label class="text-xl">Confirm New Password</Form.Label>
                <Input
                    {...attrs}
                    bind:value={$formData.confirm_password}
                    type="password"
                    placeholder="••••••••"
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
            Submit
        </Form.Button>
    </div>
</form>
