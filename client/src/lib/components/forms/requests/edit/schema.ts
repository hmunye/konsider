import { z } from "zod";

export const editSoftwareRequestSchema = z.object({
  td_request_id: z
    .string()
    .regex(/^\d{8}$/, "Request # must be exactly 8 digits"),
});

export type EditSoftwareRequestSchema = typeof editSoftwareRequestSchema;
