/* eslint-disable react-hooks/rules-of-hooks */

import { useFetch } from "../hooks/useFetch";
import { LogInSchema } from "../lib/types";

const API_URL = import.meta.env.VITE_API_URL;

export async function logIn(formData: LogInSchema) {
  try {
    const response = await useFetch({
      url: `${API_URL}/v1/auth/login`,
      method: "POST",
      requestBody: formData,
    });

    if (response.error) {
      return { error: response.error };
    }

    return response;
  } catch {
    return { error: "An error occurred during login" };
  }
}

export async function logOut() {
  try {
    const response = await useFetch({
      url: `${API_URL}/v1/auth/logout`,
      method: "POST",
    });

    if (response.error) {
      return { error: response.error };
    }

    return response;
  } catch {
    return { error: "An error occurred during logout" };
  }
}
