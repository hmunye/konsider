<script lang="ts">
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import * as Form from "$lib/components/ui/form";
import { ScrollArea } from "$lib/components/ui/scroll-area";
import { Textarea } from "$lib/components/ui/textarea";
import { fetchRequest } from "$lib/fetch";
import { toast } from "svelte-sonner";
import { defaults, superForm } from "sveltekit-superforms";
import { zod, zodClient } from "sveltekit-superforms/adapters";
import SelectReviewOption from "../select-review-option.svelte";
import { editSofwareReviewSchema } from "./schema";

let { selectedReview } = $props();

let submitting: boolean = $state(false);

const initialData = {
  is_supported: selectedReview.is_supported,
  is_current_version: selectedReview.is_current_version,
  is_reputation_good: selectedReview.is_reputation_good,
  is_installation_from_developer: selectedReview.is_installation_from_developer,
  is_local_admin_required: selectedReview.is_local_admin_required,
  is_connected_to_brockport_cloud:
    selectedReview.is_connected_to_brockport_cloud,
  is_connected_to_cloud_services_or_client:
    selectedReview.is_connected_to_cloud_services_or_client,
  is_security_or_optimization_software:
    selectedReview.is_security_or_optimization_software,
  is_supported_by_current_os: selectedReview.is_supported_by_current_os,
  review_notes: selectedReview.review_notes,
};

const form = superForm(defaults(initialData, zod(editSofwareReviewSchema)), {
  validators: zodClient(editSofwareReviewSchema),
  SPA: true,
  dataType: "json",
  resetForm: false,
  async onUpdate({ form }) {
    submitting = true;

    if (!form.valid) {
      return;
    }

    const requestBody: Record<string, string> = {};

    if ($formData.is_supported !== initialData.is_supported) {
      requestBody.is_supported = $formData.is_supported;
    }
    if ($formData.is_current_version !== initialData.is_current_version) {
      requestBody.is_current_version = $formData.is_current_version;
    }
    if ($formData.is_reputation_good !== initialData.is_reputation_good) {
      requestBody.is_reputation_good = $formData.is_reputation_good;
    }
    if (
      $formData.is_installation_from_developer !==
      initialData.is_installation_from_developer
    ) {
      requestBody.is_installation_from_developer =
        $formData.is_installation_from_developer;
    }
    if (
      $formData.is_local_admin_required !== initialData.is_local_admin_required
    ) {
      requestBody.is_local_admin_required = $formData.is_local_admin_required;
    }
    if (
      $formData.is_connected_to_brockport_cloud !==
      initialData.is_connected_to_brockport_cloud
    ) {
      requestBody.is_connected_to_brockport_cloud =
        $formData.is_connected_to_brockport_cloud;
    }
    if (
      $formData.is_connected_to_cloud_services_or_client !==
      initialData.is_connected_to_cloud_services_or_client
    ) {
      requestBody.is_connected_to_cloud_services_or_client =
        $formData.is_connected_to_cloud_services_or_client;
    }
    if (
      $formData.is_security_or_optimization_software !==
      initialData.is_security_or_optimization_software
    ) {
      requestBody.is_security_or_optimization_software =
        $formData.is_security_or_optimization_software;
    }
    if (
      $formData.is_supported_by_current_os !==
      initialData.is_supported_by_current_os
    ) {
      requestBody.is_supported_by_current_os =
        $formData.is_supported_by_current_os;
    }
    if ($formData.review_notes !== initialData.review_notes) {
      requestBody.review_notes = $formData.review_notes;
    }

    if (Object.keys(requestBody).length === 0) {
      submitting = false;
      toast.error("No values changed");
      return;
    }

    const editSoftwareReviewResponse = new Promise<unknown>(
      (resolve, reject) => {
        // Simulate a timeout before making the request to show loading toast
        setTimeout(() => {
          fetchRequest<unknown>({
            url: `${PUBLIC_BASE_API_URL}/api/v1/reviews/${selectedReview.id}`,
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

    toast.promise(editSoftwareReviewResponse, {
      loading: "Loading...",
      success: () => {
        submitting = false;
        return `Software Review for ${selectedReview.software_request.software.software_name} details have been successfully updated`;
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
    <ScrollArea class="h-[700px] w-full p-4">
        <div
            class="flex flex-col gap-8 [&>input]:mb-4 mt-8 rounded-lg p-8 py-8"
        >
            <h1 class="text-2xl font-bold mb-4">Software Review Details</h1>
            <SelectReviewOption
                {form}
                formLabel="Is Supported"
                name="is_supported"
                bind:value={$formData.is_supported}
                desc={"Is the software still supported by the developer?"}
            />

            <SelectReviewOption
                {form}
                formLabel="Is Current Version"
                name="is_current_version"
                bind:value={$formData.is_current_version}
                desc={"Is the latest version of the software being requested?"}
            />

            <SelectReviewOption
                {form}
                formLabel="Is Reputation Good"
                name="is_reputation_good"
                bind:value={$formData.is_reputation_good}
                desc={"Does the developer have a good reputation?"}
            />

            <SelectReviewOption
                {form}
                formLabel="Is Installation from Developer"
                name="is_installation_from_developer"
                bind:value={$formData.is_installation_from_developer}
                desc={"Is the installation package from the developer/vendor?"}
            />

            <SelectReviewOption
                {form}
                formLabel="Is Local Admin Required"
                name="is_local_admin_required"
                bind:value={$formData.is_local_admin_required}
                desc={"Is a local administrator required for daily use?"}
            />

            <SelectReviewOption
                {form}
                formLabel="Is Connected to Brockport Cloud"
                name="is_connected_to_brockport_cloud"
                bind:value={$formData.is_connected_to_brockport_cloud}
                desc={"Does the software need to connect to Brockport cloud?"}
            />

            <SelectReviewOption
                {form}
                formLabel="Is Connected to Cloud Services or Client"
                name="is_connected_to_cloud_services_or_client"
                bind:value={$formData.is_connected_to_cloud_services_or_client}
                desc={"Does the software need to connect to other cloud services or is a client for a cloud service?"}
            />

            <SelectReviewOption
                {form}
                formLabel="Is Security or Optimization Software"
                name="is_security_or_optimization_software"
                bind:value={$formData.is_security_or_optimization_software}
                desc={"Is the software for security or system optimization?"}
            />

            <SelectReviewOption
                {form}
                formLabel="Is Supported by Current OS"
                name="is_supported_by_current_os"
                bind:value={$formData.is_supported_by_current_os}
                desc={"Is the software supported by current OS used by devices on campus?"}
            />

            <Form.Field {form} name="review_notes">
                <Form.Control let:attrs>
                    <Form.Label class="text-xl">Review Notes</Form.Label>
                    <Textarea
                        {...attrs}
                        bind:value={$formData.review_notes}
                        placeholder="Type your notes here."
                        class="resize-none text-lg placeholder:text-lg placeholder:font-light"
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
    </ScrollArea>
</form>
