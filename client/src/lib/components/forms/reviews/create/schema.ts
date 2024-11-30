import { reviewOptionsEnum } from "$lib/types/types";
import { z } from "zod";

export const createRequesterSchema = z.object({
  name: z
    .string()
    .min(1, "Name required")
    .max(100, "Name cannot exceed 100 characters"),
  email: z.string().email(),
  department: z
    .string()
    .min(1, "Department required")
    .max(100, "Department cannot exceed 100 characters"),
  td_request_id: z
    .string()
    .regex(/^\d{8}$/, "Request # must be exactly 8 digits"),
});

export type CreateRequesterSchema = typeof createRequesterSchema;

export const createSoftwareSchema = z.object({
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

export type CreateSoftwareSchema = typeof createSoftwareSchema;

export const reviewGroup1Schema = z.object({
  is_supported: z.enum(reviewOptionsEnum, {
    errorMap: () => ({
      message: "Must select a valid answer",
    }),
  }),
  is_current_version: z.enum(reviewOptionsEnum, {
    errorMap: () => ({
      message: "Must select a valid answer",
    }),
  }),
  is_reputation_good: z.enum(reviewOptionsEnum, {
    errorMap: () => ({
      message: "Must select a valid answer",
    }),
  }),
});

export const reviewGroup2Schema = z.object({
  is_installation_from_developer: z.enum(reviewOptionsEnum, {
    errorMap: () => ({
      message: "Must select a valid answer",
    }),
  }),
  is_local_admin_required: z.enum(reviewOptionsEnum, {
    errorMap: () => ({
      message: "Must select a valid answer",
    }),
  }),
  is_connected_to_brockport_cloud: z.enum(reviewOptionsEnum, {
    errorMap: () => ({
      message: "Must select a valid answer",
    }),
  }),
});

export const reviewGroup3Schema = z.object({
  is_connected_to_cloud_services_or_client: z.enum(reviewOptionsEnum, {
    errorMap: () => ({
      message: "Must select a valid answer",
    }),
  }),
  is_security_or_optimization_software: z.enum(reviewOptionsEnum, {
    errorMap: () => ({
      message: "Must select a valid answer",
    }),
  }),
  is_supported_by_current_os: z.enum(reviewOptionsEnum, {
    errorMap: () => ({
      message: "Must select a valid answer",
    }),
  }),
});

export const reviewNotesSchema = z.object({
  review_notes: z
    .string()
    .min(1, "Notes are required")
    .max(255, "Notes cannot exceed 255 characters"),
});

export type ReviewGroup1Schema = z.infer<typeof reviewGroup1Schema>;
export type ReviewGroup2Schema = z.infer<typeof reviewGroup2Schema>;
export type ReviewGroup3Schema = z.infer<typeof reviewGroup3Schema>;
export type ReviewNotesSchema = z.infer<typeof reviewNotesSchema>;
