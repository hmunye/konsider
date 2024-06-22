import { z } from "zod";

export type FetchParams = {
  url: string;
  method: string;
  requestBody?: any;
};

export const createPostFormSchema = z.object({
  title: z
    .string()
    .min(1, { message: "Title is required" })
    .max(50, { message: "Title can not be longer than 50 characters" }),
  content: z
    .string()
    .min(1, { message: "Content is required" })
    .max(255, { message: "Content can not be longer than 255 characters" }),
});
