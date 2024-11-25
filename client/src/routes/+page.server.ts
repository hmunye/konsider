import { superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { logInSchema } from "../lib/components/forms/login/schema.js";
import type { PageServerLoad } from "./$types.js";

export const load: PageServerLoad = async () => {
  return {
    form: await superValidate(zod(logInSchema)),
  };
};
