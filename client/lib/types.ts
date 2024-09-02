import { z } from "zod";

export type FetchParams = {
  url: string;
  method: string;
  requestBody?: any;
};

export const logInSchema = z.object({
  email: z.string().email(),
  password: z.string().min(1, "Password required"),
});

export type LogInSchema = z.infer<typeof logInSchema>;
