import { z } from "zod";

const roleEnum = ["ADMIN", "REVIEWER"] as const;

export const createUserSchema = z.object({
  name: z
    .string()
    .min(1, "Name required")
    .max(100, "Name cannot exceed 100 characters"),
  email: z.string().email(),
  password: z
    .string()
    .min(8, "Password must be at least 8 characters")
    .max(128, "Password cannot exceed 128 characters"),
  role: z.enum(roleEnum, {
    errorMap: (_issue, _context) => {
      return { message: "Role must be either 'ADMIN' or 'REVIEWER'" };
    },
  }),
});

export type CreateUserSchema = typeof createUserSchema;
