import { roleEnum } from "$lib/types/types";
import { z } from "zod";

export const editUserSchema = z.object({
  name: z
    .string()
    .min(1, "Name required")
    .max(100, "Name cannot exceed 100 characters"),
  email: z.string().email(),
  role: z.enum(roleEnum, {
    errorMap: (_issue, _context) => {
      return { message: "Role must be either 'ADMIN' or 'REVIEWER'" };
    },
  }),
});

export type EditUserSchema = typeof editUserSchema;
