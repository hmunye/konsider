import { reviewOptionsEnum } from "$lib/types/types";
import { z } from "zod";

export const editSofwareReviewSchema = z.object({
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

  review_notes: z
    .string()
    .min(1, "Notes are required")
    .max(255, "Notes cannot exceed 255 characters"),
});

export type EditSoftwareReviewSchema = z.infer<typeof editSofwareReviewSchema>;
