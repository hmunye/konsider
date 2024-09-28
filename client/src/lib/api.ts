import { useFetch } from "@/src/hooks/useFetch";
import { LogInSchema } from "@/src/lib/types";

const API_URL = process.env.NEXT_PUBLIC_API_URL;

export async function LogIn(formData: LogInSchema) {
  try {
    const response = await useFetch({
      url: `${API_URL}/v1/auth/login`,
      method: "POST",
      requestBody: formData,
    });

    if (response.error) {
      return { error: response.error };
    }

    return response.success;
  } catch {
    return { error: "An error occurred during login" };
  }
}

export async function LogOut() {
  try {
    const response = await useFetch({
      url: `${API_URL}/v1/auth/logout`,
      method: "POST",
    });

    if (response.error) {
      return { error: response.error };
    }

    return response.success;
  } catch {
    return { error: "An error occurred during logout" };
  }
}

export async function CheckAuth(cookie: string) {
  try {
    const response = await useFetch({
      url: `${API_URL}/v1/auth/check`,
      cookie: cookie,
      method: "GET",
    });

    if (response.error) {
      return { error: response.error };
    }

    return response.success;
  } catch {
    return { error: "An error occurred during authentication check" };
  }
}
