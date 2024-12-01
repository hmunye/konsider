import { PUBLIC_BASE_API_URL } from "$env/static/public";
import { fetchRequest } from "$lib/fetch.js";
import type { User, UserResponse } from "$lib/types/types.js";
import { redirect } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ fetch, url }) => {
  const response = await fetchRequest<{ user: User }>(
    {
      url: `${PUBLIC_BASE_API_URL}/api/v1/auth/check`,
      method: "GET",
    },
    fetch,
  );

  if (response.error) {
    const fromUrl = url.pathname + url.search;

    throw redirect(
      302,
      `/?redirectTo=${fromUrl}&message=${response.error.message}`,
    );
  }

  let usersResponse = null;

  if (
    response.success?.user.role === "ADMIN" &&
    url.pathname === "/dashboard"
  ) {
    usersResponse = await fetchRequest<UserResponse>(
      {
        url: `${PUBLIC_BASE_API_URL}/api/v1/users?per_page=1`,
        method: "GET",
      },
      fetch,
    );
  }

  if (
    response.success?.user.role !== "ADMIN" &&
    url.pathname.includes("users")
  ) {
    throw redirect(302, "/dashboard");
  }

  return {
    users: usersResponse?.success ?? null,
    current_user: response.success?.user,
  };
};
