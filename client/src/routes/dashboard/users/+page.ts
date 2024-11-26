import { PUBLIC_BASE_API_URL } from "$env/static/public";
import { fetchRequest } from "$lib/fetch.js";
import type { UserResponse } from "$lib/types/types.js";
import type { PageLoad } from "../../admin/users/$types.js";

export const load: PageLoad = async ({ fetch, url }) => {
    const page = url.searchParams.get("page") || "1";
    const perPage = url.searchParams.get("per_page") || "8";

    const response = await fetchRequest<UserResponse>(
        {
            url: `${PUBLIC_BASE_API_URL}/api/v1/users?per_page=${perPage}&page=${page}`,
            method: "GET",
        },
        fetch,
    );

    if (response.error) {
        return {
            error: response.error.message,
        };
    }

    return {
        users: response.success,
    };
};
