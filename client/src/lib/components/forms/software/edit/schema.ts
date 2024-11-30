import { z } from "zod";

export const editSoftwareSchema = z.object({
  software_name: z
    .string()
    .min(1, "Name required")
    .max(100, "Name cannot exceed 100 characters"),
  software_version: z
    .string()
    .regex(/^\d+\.\d+\.\d+$/, "Version must be in the format x.y.z")
    .max(12, "Version cannot exceed 12 characters"),
  developer_name: z
    .string()
    .min(1, "Developer name required")
    .max(100, "Developer name cannot exceed 100 characters"),
  description: z
    .string()
    .min(1, "Description required")
    .max(255, "Description cannot exceed 255 characters"),
});

export type EditSoftwareSchema = typeof editSoftwareSchema;
