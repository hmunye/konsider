import { ApiResponse, FetchParams } from "@/src/lib/types";

export const API_URL = process.env.NEXT_PUBLIC_API_URL;

class HttpError extends Error {
  public response: Response;

  constructor(response: Response) {
    super(`HTTP Error: ${response.status} ${response.statusText}`);
    this.response = response;
  }

  async getErrorMessage(): Promise<string> {
    try {
      const errorBody = await this.response.json();
      return errorBody.error || this.response.statusText || "an error occurred";
    } catch {
      const text = await this.response.text();
      return text || this.response.statusText || "an error occurred";
    }
  }
}

export async function fetchData<T>(
  params: FetchParams,
): Promise<ApiResponse<T>> {
  const { url, method, cookie, requestBody } = params;

  const fetchOptions: RequestInit = {
    method: method.toUpperCase(),
    credentials: "include",
  };

  if (cookie) {
    fetchOptions.headers = {
      Cookie: cookie,
    };
  }

  if (requestBody) {
    fetchOptions.headers = {
      "Content-Type": "application/json",
      ...(fetchOptions.headers || {}), // Preserve existing headers
    };
    fetchOptions.body = JSON.stringify(requestBody);
  }

  try {
    const response = await fetch(url, {
      ...fetchOptions,
      // Sets timeout for each fetch call
      signal: AbortSignal.timeout(3000),
    });

    if (!response.ok) {
      throw new HttpError(response);
    }

    const contentLength = response.headers.get("content-length");

    // Check if the response contains a body
    if (contentLength && contentLength !== "0") {
      try {
        const responseBody = await response.json();
        return { success: responseBody };
      } catch {
        return { error: "failed to parse JSON response" };
      }
    } else {
      return { success: response as T };
    }
  } catch (err) {
    if (err instanceof Error && err.name === "AbortError") {
      throw new Error("request timed-out");
    }

    if (err instanceof HttpError) {
      const errorMessage = await err.getErrorMessage();
      return { error: errorMessage };
    }

    if (err instanceof Error) {
      return { error: err.message };
    }

    return { error: "server error occurred" };
  }
}
