<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { PUBLIC_BASE_API_URL } from "$env/static/public";
    import ResponseMessage from "$lib/components/custom/response-message/response-message.svelte";
    import * as Form from "$lib/components/ui/form";
    import { Input } from "$lib/components/ui/input";
    import { fetchRequest } from "$lib/fetch";
    import type { Message } from "$lib/types/types";
    import { defaults, superForm } from "sveltekit-superforms";
    import { zod, zodClient } from "sveltekit-superforms/adapters";
    import { logInSchema } from "./schema";

    let responseMessage: Message | undefined = $state();

    let redirectMessage: Message | undefined = $derived(
        $page.url.searchParams.get("message")
            ? { error: $page.url.searchParams.get("message")! }
            : undefined,
    );

    const initialData = {
        email: "",
        password: "",
    };

    const form = superForm(defaults(initialData, zod(logInSchema)), {
        validators: zodClient(logInSchema),
        SPA: true,
        dataType: "json",
        resetForm: false,
        async onUpdate({ form }) {
            responseMessage = undefined;

            if (!form.valid) {
                return;
            }

            const response = await fetchRequest<unknown>({
                url: `${PUBLIC_BASE_API_URL}/api/v1/auth/login`,
                method: "POST",
                requestBody: {
                    email: $formData.email,
                    password: $formData.password,
                },
            });

            if (response.error) {
                responseMessage = { error: response.error.message };
                return;
            }

            const redirectTo = $page.url.searchParams.get("redirectTo");

            goto(redirectTo ? `/${redirectTo.slice(1)}` : "/dashboard", {});
        },
    });

    const { form: formData, enhance, submitting } = form;
</script>

<div class="flex flex-col flex-1 max-w-full items-center p-4">
    <form
        method="POST"
        class="flex flex-1 flex-col w-screen justify-center gap-2 text-foreground [&>input]:mb-6 max-w-lg p-4 animate-in"
        use:enhance
    >
        <div
            class="flex flex-col gap-10 [&>input]:mb-4 mt-8 rounded-lg p-8 py-16 border"
        >
            <h1 class="text-2xl font-bold mb-4">Log In</h1>
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
            <Form.Field {form} name="password">
                <Form.Control let:attrs>
                    <Form.Label class="text-xl">Password</Form.Label>
                    <Input
                        {...attrs}
                        bind:value={$formData.password}
                        type="password"
                        placeholder="••••••••"
                        class="text-lg placeholder:text-lg placeholder:font-light"
                    />
                </Form.Control>
                <Form.FieldErrors class="text-lg" />
            </Form.Field>
            <Form.Button
                class="bg-success text-success-foreground text-lg hover:bg-success hover:brightness-125 transition duration-300"
                disabled={$submitting}
                aria-disabled={$submitting}
            >
                {#if $submitting}<div
                        class="animate-spin border-4 border-solid border-l-transparent rounded-2xl w-6 h-6 border-foreground brightness-75"
                    ></div>
                {:else}
                    Submit
                {/if}
            </Form.Button>
            {#if responseMessage}
                <ResponseMessage
                    class="animate-in text-lg mt-3"
                    message={responseMessage}
                />
            {:else if redirectMessage}
                <ResponseMessage
                    class="animate-in text-lg mt-3"
                    message={redirectMessage}
                />
            {/if}
        </div>
    </form>
</div>
