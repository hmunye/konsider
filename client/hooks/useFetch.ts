import { FetchParams } from "@/types/types";

export class HttpError extends Error {
    constructor(public response: Response) {
        super(`HTTP Error ${response.status}`);
    }
}

export async function useFetch(params: FetchParams): Promise<any> {
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
        const response = await fetch(url, {
            ...fetchData,
            // Sets timeout for each fetch call
            signal: AbortSignal.timeout(3000),
        });

        if (!response.ok) {
            throw new Error("fetch error occured");
        }

        return response.json();

        // TODO: change any
    } catch (err: any) {
        if (err.name === "AbortError") {
            throw new Error("fetch call timed-out")
        }

        if (err instanceof HttpError) {
            if (err.response.status === 400) {
                console.log("BAD_REQUEST")
            }

            if (err.response.status === 401) {
                console.log("UNAUTHORIZED")
            }

            if (err.response.status === 500) {
                console.log("SERVER_ERROR")
            }

        }
    }
}
