import { z } from "zod";

export const changeUserPasswordSchema = z
  .object({
    current_password: z.string().min(1, "Password required"),
    new_password: z
      .string()
      .min(8, "Password must be at least 8 characters")
      .max(128, "Password cannot exceed 128 characters"),
    confirm_password: z.string(),
  })
  .refine((data) => data.new_password !== data.current_password, {
    message: "Passwords must be different",
    path: ["new_password"],
  })
  .refine((data) => data.new_password === data.confirm_password, {
    message: "Passwords must match",
    path: ["confirm_password"],
  });

export type ChangeUserPasswordSchema = typeof changeUserPasswordSchema;
