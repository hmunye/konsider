import { z } from "zod";

export const logInSchema = z.object({
  email: z.string().email(),
  password: z.string().min(1, "Password required"),
});

export type LogInSchema = z.infer<typeof logInSchema>;

export type FetchParams = {
  url: string;
  method: string;
  cookie?: string;
  requestBody?: unknown;
};

export type ApiResponse<T> = {
  success?: T;
  error?: string;
};

export type User = {
  name: string;
  email: string;
  role: "Admin" | "Reviewer";
};
