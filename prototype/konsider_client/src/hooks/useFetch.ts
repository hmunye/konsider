import { FetchParams } from "@/types/types";

async function useFetch(params: FetchParams): Promise<any> {
  const { url, method, requestBody } = params;

  const fetchData: RequestInit = {
    headers: {
      "Content-Type": "application/json",
    },
    method: method.toUpperCase(),
  };

  if (requestBody) {
    fetchData.body = JSON.stringify(requestBody);
  }

  try {
    const response = await fetch(url, fetchData);

    if (!response.ok) {
      throw new Error("Network Error");
    }

    return response.json();
  } catch (error) {
    throw new Error("Fetch Request Failed");
  }
}

export { useFetch };
