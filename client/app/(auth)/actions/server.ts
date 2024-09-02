"use client";

import { useFetch } from "@/hooks/useFetch";
import { API_LOGIN } from "@/lib/endpoints";
import { LogInSchema } from "@/lib/types";

export async function logIn(formData: LogInSchema) {
  try {
    const response = await useFetch({
      url: API_LOGIN,
      method: "POST",
      requestBody: formData,
    });

    if (response.error) {
      return { error: response.error };
    }

    return response;
  } catch (error) {
    return { error: "An error occurred during login" };
  }
}

export async function logOut() {}
