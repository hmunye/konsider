import { z } from "zod";

export const editRequesterSchema = z.object({
  name: z
    .string()
    .min(1, "Name required")
    .max(100, "Name cannot exceed 100 characters"),
  email: z.string().email(),
  department: z
    .string()
    .min(1, "Department required")
    .max(100, "Department cannot exceed 100 characters"),
});

export type EditRequesterSchema = typeof editRequesterSchema;
