<script lang="ts">
import * as Form from "$lib/components/ui/form";
import { Textarea } from "$lib/components/ui/textarea";
import { fetchRequest } from "$lib/fetch";
import { defaults, superForm } from "sveltekit-superforms";
import { zod, zodClient } from "sveltekit-superforms/adapters";
import SelectReviewOption from "../select-review-option.svelte";
import CreateRequesterForm from "./create-requester-form.svelte";
import CreateSoftwareForm from "./create-software-form.svelte";
import {
  createRequesterSchema,
  createSoftwareSchema,
  reviewGroup1Schema,
  reviewGroup2Schema,
  reviewGroup3Schema,
  reviewNotesSchema,
} from "./schema";
import { PUBLIC_BASE_API_URL } from "$env/static/public";
import { toast } from "svelte-sonner";
import { userStore } from "$lib/stores/userStore";

let currentStep: number = $state(1);
let submitting: boolean = $state(false);

const requesterForm = superForm(
  defaults(
    {
      name: "",
      email: "",
      department: "",
      td_request_id: "",
    },
    zod(createRequesterSchema),
  ),
  {
    validators: zodClient(createRequesterSchema),
    SPA: true,
    taintedMessage: true,
    dataType: "json",
  },
);
const { form: requesterFormData, validateForm: validateRequesterForm } =
  requesterForm;

const softwareForm = superForm(
  defaults(
    {
      software_name: "",
      software_version: "",
      developer_name: "",
      description: "",
    },
    zod(createSoftwareSchema),
  ),
  {
    validators: zodClient(createSoftwareSchema),
    SPA: true,
    taintedMessage: true,
    dataType: "json",
  },
);
const { form: softwareFormData, validateForm: validateSoftwareForm } =
  softwareForm;

const reviewGroup1Form = superForm(
  defaults(
    {
      is_supported: undefined,
      is_current_version: undefined,
      is_reputation_good: undefined,
    },
    zod(reviewGroup1Schema),
  ),
  {
    validators: zodClient(reviewGroup1Schema),
    SPA: true,
    taintedMessage: true,
    dataType: "json",
  },
);
const { form: reviewGroup1FormData, validateForm: validateReviewGroup1Form } =
  reviewGroup1Form;

const reviewGroup2Form = superForm(
  defaults(
    {
      is_installation_from_developer: undefined,
      is_local_admin_required: undefined,
      is_connected_to_brockport_cloud: undefined,
    },
    zod(reviewGroup2Schema),
  ),
  {
    validators: zodClient(reviewGroup2Schema),
    SPA: true,
    taintedMessage: true,
    dataType: "json",
  },
);
const { form: reviewGroup2FormData, validateForm: validateReviewGroup2Form } =
  reviewGroup2Form;

const reviewGroup3Form = superForm(
  defaults(
    {
      is_connected_to_cloud_services_or_client: undefined,
      is_security_or_optimization_software: undefined,
      is_supported_by_current_os: undefined,
    },
    zod(reviewGroup3Schema),
  ),
  {
    validators: zodClient(reviewGroup3Schema),
    SPA: true,
    taintedMessage: true,
    dataType: "json",
  },
);
const { form: reviewGroup3FormData, validateForm: validateReviewGroup3Form } =
  reviewGroup3Form;

const reviewNotesForm = superForm(
  defaults(
    {
      review_notes: "",
    },
    zod(reviewNotesSchema),
  ),
  {
    validators: zodClient(reviewNotesSchema),
    SPA: true,
    taintedMessage: true,
    dataType: "json",
    async onSubmit() {
      submitting = true;

      const result = await validateReviewNotesForm({ update: true });

      if (!result.valid) {
        submitting = false;
        return;
      }

      const requestBody = {
        software_request: {
          td_request_id: allFormData.td_request_id,
          software: {
            software_name: allFormData.software_name,
            software_version: allFormData.software_version,
            developer_name: allFormData.developer_name,
            description: allFormData.description,
          },
          requester: {
            name: allFormData.name,
            email: allFormData.email,
            department: allFormData.department,
          },
        },
        reviewer_id: $userStore?.id,
        is_supported: allFormData.is_supported,
        is_current_version: allFormData.is_current_version,
        is_reputation_good: allFormData.is_reputation_good,
        is_installation_from_developer:
          allFormData.is_installation_from_developer,
        is_local_admin_required: allFormData.is_local_admin_required,
        is_connected_to_brockport_cloud:
          allFormData.is_connected_to_brockport_cloud,
        is_connected_to_cloud_services_or_client:
          allFormData.is_connected_to_cloud_services_or_client,
        is_security_or_optimization_software:
          allFormData.is_security_or_optimization_software,
        is_supported_by_current_os: allFormData.is_supported_by_current_os,
        review_notes: allFormData.review_notes,
      };

      const createSoftwareReviewResponse = new Promise<unknown>(
        (resolve, reject) => {
          // Simulate a timeout before making the request to show loading toast
          setTimeout(() => {
            fetchRequest<unknown>({
              url: `${PUBLIC_BASE_API_URL}/api/v1/reviews`,
              method: "POST",
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

      toast.promise(createSoftwareReviewResponse, {
        loading: "Loading...",
        success: () => {
          submitting = false;
          return `Review for ${allFormData.software_name} has been successfully created`;
        },
        error: (error) => {
          submitting = false;
          return `${error}`;
        },
      });
    },
  },
);
const {
  form: reviewNotesFormData,
  validateForm: validateReviewNotesForm,
  enhance,
} = reviewNotesForm;

const allFormData = $derived({
  ...$requesterFormData,
  ...$softwareFormData,
  ...$reviewGroup1FormData,
  ...$reviewGroup2FormData,
  ...$reviewGroup3FormData,
  ...$reviewNotesFormData,
});

async function nextStep() {
  let valid = false;

  if (currentStep === 1) {
    const result = await validateRequesterForm({ update: true });
    valid = result.valid;
  } else if (currentStep === 2) {
    const result = await validateSoftwareForm({ update: true });
    valid = result.valid;
  } else if (currentStep === 3) {
    const result = await validateReviewGroup1Form({ update: true });
    valid = result.valid;
  } else if (currentStep === 4) {
    const result = await validateReviewGroup2Form({ update: true });
    valid = result.valid;
  } else if (currentStep === 5) {
    const result = await validateReviewGroup3Form({ update: true });
    valid = result.valid;
  }

  if (valid) {
    currentStep += 1;
  }
}

function previousStep() {
  if (currentStep > 1) {
    currentStep -= 1;
  }
}
</script>

<form method="POST" use:enhance>
    <div class="flex flex-col [&>input]:mb-4 rounded-lg">
        {#if currentStep === 1}
            <CreateRequesterForm
                form={requesterForm}
                formData={requesterFormData}
            />
        {/if}

        {#if currentStep === 2}
            <CreateSoftwareForm
                form={softwareForm}
                formData={softwareFormData}
            />
        {/if}

        {#if currentStep > 2}
            <div
                class="flex flex-col gap-8 [&>input]:mb-4 mt-8 rounded-lg p-8 py-8"
            >
                <h1 class="text-2xl font-bold mb-4">Software Review Details</h1>

                {#if currentStep === 3}
                    <SelectReviewOption
                        form={reviewGroup1Form}
                        formLabel="Is Supported"
                        name="is_supported"
                        bind:value={$reviewGroup1FormData.is_supported}
                        desc={"Is the software still supported by the developer?"}
                    />

                    <SelectReviewOption
                        form={reviewGroup1Form}
                        formLabel="Is Current Version"
                        name="is_current_version"
                        bind:value={$reviewGroup1FormData.is_current_version}
                        desc={"Is the latest version of the software being requested?"}
                    />

                    <SelectReviewOption
                        form={reviewGroup1Form}
                        formLabel="Is Reputation Good"
                        name="is_reputation_good"
                        bind:value={$reviewGroup1FormData.is_reputation_good}
                        desc={"Does the developer have a good reputation?"}
                    />
                {/if}

                {#if currentStep === 4}
                    <SelectReviewOption
                        form={reviewGroup2Form}
                        formLabel="Is Installation from Developer"
                        name="is_installation_from_developer"
                        bind:value={$reviewGroup2FormData.is_installation_from_developer}
                        desc={"Is the installation package from the developer/vendor?"}
                    />

                    <SelectReviewOption
                        form={reviewGroup2Form}
                        formLabel="Is Local Admin Required"
                        name="is_local_admin_required"
                        bind:value={$reviewGroup2FormData.is_local_admin_required}
                        desc={"Is a local administrator required for daily use?"}
                    />

                    <SelectReviewOption
                        form={reviewGroup2Form}
                        formLabel="Is Connected to Brockport Cloud"
                        name="is_connected_to_brockport_cloud"
                        bind:value={$reviewGroup2FormData.is_connected_to_brockport_cloud}
                        desc={"Does the software need to connect to Brockport cloud?"}
                    />
                {/if}

                {#if currentStep === 5}
                    <SelectReviewOption
                        form={reviewGroup3Form}
                        formLabel="Is Connected to Cloud Services or Client"
                        name="is_connected_to_cloud_services_or_client"
                        bind:value={$reviewGroup3FormData.is_connected_to_cloud_services_or_client}
                        desc={"Does the software need to connect to other cloud services or is a client for a cloud service?"}
                    />

                    <SelectReviewOption
                        form={reviewGroup3Form}
                        formLabel="Is Security or Optimization Software"
                        name="is_security_or_optimization_software"
                        bind:value={$reviewGroup3FormData.is_security_or_optimization_software}
                        desc={"Is the software for security or system optimization?"}
                    />

                    <SelectReviewOption
                        form={reviewGroup3Form}
                        formLabel="Is Supported by Current OS"
                        name="is_supported_by_current_os"
                        bind:value={$reviewGroup3FormData.is_supported_by_current_os}
                        desc={"Is the software supported by current OS used by devices on campus?"}
                    />
                {/if}

                {#if currentStep === 6}
                    <Form.Field form={reviewNotesForm} name="review_notes">
                        <Form.Control let:attrs>
                            <Form.Label class="text-xl">Review Notes</Form.Label
                            >
                            <Textarea
                                {...attrs}
                                bind:value={$reviewNotesFormData.review_notes}
                                placeholder="Type your notes here."
                                class="resize-none text-lg placeholder:text-lg placeholder:font-light"
                            />
                        </Form.Control>
                        <Form.FieldErrors class="text-lg" />
                    </Form.Field>
                {/if}
            </div>
        {/if}
        <div class="flex mt-4 justify-between">
            {#if currentStep > 1}
                <Form.Button
                    variant="ghost"
                    class="text-lg hover:brightness-125 transition duration-300 mr-auto"
                    type="button"
                    onclick={previousStep}
                >
                    Previous
                </Form.Button>
            {/if}

            {#if currentStep < 6}
                <Form.Button
                    variant="ghost"
                    class="text-lg hover:brightness-125 transition duration-300 ml-auto"
                    type="button"
                    onclick={nextStep}
                >
                    Next
                </Form.Button>
            {/if}

            {#if currentStep === 6}
                <Form.Button
                    class="bg-success text-success-foreground text-lg hover:bg-success hover:brightness-125 transition duration-300"
                    disabled={submitting}
                    aria-disabled={submitting}
                    type="submit"
                >
                    Submit
                </Form.Button>
            {/if}
        </div>
    </div>
</form>
