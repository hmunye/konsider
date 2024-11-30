<script lang="ts">
import * as Form from "$lib/components/ui/form";
import * as Select from "$lib/components/ui/select";

let { form, formLabel, name, value = $bindable(), desc } = $props();

let selectedOption = $derived(
  value
    ? {
        label: value
          .split("_")
          .map(
            (word: string) =>
              word.charAt(0).toUpperCase() + word.slice(1).toLowerCase(),
          )
          .join(" "),
        value: value,
      }
    : undefined,
);
</script>

<Form.Field {form} {name}>
    <Form.Control let:attrs>
        <Form.Label class="text-xl">{formLabel}</Form.Label>
        <Select.Root
            selected={selectedOption}
            onSelectedChange={(v) => {
                v && (value = v.value);
            }}
        >
            <Select.Trigger {...attrs}>
                <Select.Value
                    placeholder="Select Answer"
                    class="text-lg placeholder:text-lg placeholder:font-light"
                />
            </Select.Trigger>
            <Select.Content>
                <Select.Item value="TRUE" label="True" class="text-md" />
                <Select.Item value="FALSE" label="False" class="text-md" />
                <Select.Item
                    value="NOT_SURE"
                    label="Not Sure"
                    class="text-md"
                />
            </Select.Content>
        </Select.Root>
        <input hidden bind:value name={attrs.name} />
    </Form.Control>
    <Form.Description class="text-md">{desc}</Form.Description>
    <Form.FieldErrors />
</Form.Field>
