import { FetchParams } from "../lib/types";

class HttpError extends Error {
  public response: Response;

  constructor(response: Response) {
    super(`HTTP Error: ${response.status} ${response.statusText}`);
    this.response = response;
  }

  async getErrorMessage(): Promise<string> {
    try {
      const errorBody = await this.response.json();
      return errorBody.error || this.response.statusText || "An error occurred";
    } catch {
      return this.response.statusText || "An error occurred";
    }
  }
}

export async function useFetch(params: FetchParams): Promise<any> {
  const { url, method, requestBody } = params;

  const fetchData: RequestInit = {
    headers: {
      "Content-Type": "application/json",
    },
    method: method.toUpperCase(),
    credentials: "include",
  };

  if (requestBody) {
    fetchData.body = JSON.stringify(requestBody);
  }

  try {
    const response = await fetch(url, {
      ...fetchData,
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
        return { success: "request processed successfully", responseBody };
      } catch {
        return { error: "Failed to parse JSON response" };
      }
    } else {
      return { success: "request processed successfully. no response body" };
    }
  } catch (err: any) {
    if (err.name === "AbortError") {
      throw new Error("fetch call timed-out");
    }

    if (err instanceof HttpError) {
      const errorMessage = await err.getErrorMessage();

      return { error: errorMessage };
    }

    return { error: "Server Error Occurred" };
  }
}
