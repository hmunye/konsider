type FetchParams = {
  url: string;
  method: string;
  requestBody?: unknown;
};

export type ApiResponse<T> = {
  success?: T;
  error?: {
    status: number;
    message: string;
  };
};

class HttpError extends Error {
  public response: Response;

  constructor(response: Response) {
    super(`HTTP Error: ${response.status} ${response.statusText}`);
    this.response = response;
  }

  async getErrorMessage(): Promise<string> {
    try {
      const errorBody = await this.response.json();
      return (
        errorBody.error ||
        this.response.statusText ||
        "an unexpected error occurred"
      );
    } catch {
      const text = await this.response.text();
      return text || this.response.statusText || "an unexpected error occurred";
    }
  }
}

export async function fetchRequest<T>(
  params: FetchParams,
  serverFetch?: (input: RequestInfo, init?: RequestInit) => Promise<Response>,
): Promise<ApiResponse<T>> {
  const { url, method, requestBody } = params;

  const fetchOptions: RequestInit = {
    method: method.toUpperCase(),
    credentials: "include",
  };

  if (requestBody) {
    fetchOptions.headers = {
      "Content-Type": "application/json",
      ...(fetchOptions.headers || {}), // Preserve existing headers
    };
    fetchOptions.body = JSON.stringify(requestBody);
  }

  try {
    let response: Response = new Response();

    // If using `fetch` passed in from server load function
    if (serverFetch) {
      response = await serverFetch(url, {
        ...fetchOptions,
        signal: AbortSignal.timeout(3000), // Set timeout for the fetch call
      });
    } else {
      response = await fetch(url, {
        ...fetchOptions,
        signal: AbortSignal.timeout(3000), // Set timeout for the fetch call
      });
    }

    if (!response.ok) {
      throw new HttpError(response);
    }

    const responseText = await response.text();

    if (responseText.trim() === "") {
      return { success: response as T };
    }

    const responseBody = JSON.parse(responseText);

    return { success: responseBody };
  } catch (err) {
    if (err instanceof Error && err.name === "AbortError") {
      return { error: { status: 408, message: "request timed-out" } };
    }

    if (err instanceof HttpError) {
      const errorMessage = await err.getErrorMessage();
      return {
        error: {
          status: err.response.status,
          message: errorMessage,
        },
      };
    }

    if (err instanceof Error) {
      return { error: { status: 500, message: err.message } };
    }

    return { error: { status: 500, message: "server error occurred" } };
  }
}
